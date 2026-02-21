use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection};

use crate::ai::embedding::EmbeddingConfig;
use crate::ai::relationship::{extract_batch, extract_relations, RelationCandidate};
use crate::db::now_ms;
use crate::models::{PipelineProgress, SimilarPair};
use crate::similarity::{deserialize_embedding, find_similar_in_batch, find_similar_mashes};

fn update_progress(
    progress: &Arc<Mutex<Option<PipelineProgress>>>,
    phase: &str,
    step: &str,
    current: u32,
    total: u32,
) {
    if let Ok(mut p) = progress.lock() {
        *p = Some(PipelineProgress {
            phase: phase.to_string(),
            step: step.to_string(),
            current,
            total,
        });
    }
}

struct DistilledMash {
    id: String,
    summary: String,
    embedding: Option<Vec<f32>>,
}

pub async fn jar_mashes(
    conn: &Arc<Mutex<Connection>>,
    config: &EmbeddingConfig,
) -> Result<u32, String> {
    // Step 1: Read DISTILLED mashes (sync)
    let distilled: Vec<DistilledMash> = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, summary, embedding FROM mashes WHERE status = 'DISTILLED'")
            .map_err(|e| e.to_string())?;
        let result = stmt
            .query_map([], |row| {
                let embedding_blob: Option<Vec<u8>> = row.get(2)?;
                Ok(DistilledMash {
                    id: row.get(0)?,
                    summary: row.get(1)?,
                    embedding: embedding_blob.map(|b| deserialize_embedding(&b)),
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());
        result?
    };

    if distilled.is_empty() {
        log::info!("No mashes to jar");
        return Ok(0);
    }

    log::info!("Jarring {} mashes", distilled.len());

    // Read similarity settings
    let (threshold, pipeline_top_k) = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        let threshold = crate::db::settings::get_setting(&conn, "pipeline_threshold")?
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.3);
        let top_k = crate::db::settings::get_setting(&conn, "pipeline_top_k")?
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(5);
        (threshold, top_k)
    };

    // Step 2: Find similar pairs (sync - all in-memory)
    let similar_pairs = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        find_all_similar_pairs(&conn, &distilled, threshold, pipeline_top_k)?
    };
    log::info!("Found {} similar pairs", similar_pairs.len());

    // Step 3: Build candidates for AI extraction (sync)
    let candidates = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        build_candidates(&conn, &similar_pairs, &distilled)?
    };
    log::info!("Built {} candidates for AI extraction", candidates.len());

    // Step 4: AI relationship extraction (async)
    let relations = extract_relations(config, candidates).await?;
    log::info!("Extracted {} relations", relations.len());

    // Step 5: Create edges and update status (sync)
    let conn = conn.lock().map_err(|e| e.to_string())?;
    let now = now_ms();
    let mut edges_created = 0u32;
    for rel in &relations {
        let result = conn.execute(
            "INSERT INTO edges (source_id, target_id, relation_type, source, confidence, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'ai', ?4, ?5, ?6)
             ON CONFLICT(source_id, target_id) DO UPDATE SET
                relation_type = excluded.relation_type,
                confidence = excluded.confidence,
                updated_at = excluded.updated_at",
            params![rel.source_id, rel.target_id, rel.relation, rel.confidence, now, now],
        );
        match result {
            Ok(_) => edges_created += 1,
            Err(e) => log::warn!("Failed to create edge: {}", e),
        }
    }

    let mut jarred = 0u32;
    for mash in &distilled {
        conn.execute(
            "UPDATE mashes SET status = 'JARRED', updated_at = ?1 WHERE id = ?2",
            params![now, mash.id],
        )
        .map_err(|e| e.to_string())?;
        jarred += 1;
    }

    log::info!("Jarring complete: {} mashes, {} edges", jarred, edges_created);
    Ok(jarred)
}

fn find_all_similar_pairs(
    conn: &Connection,
    distilled: &[DistilledMash],
    threshold: f32,
    pipeline_top_k: usize,
) -> Result<Vec<SimilarPair>, String> {
    let with_embedding: Vec<(&DistilledMash, &Vec<f32>)> = distilled
        .iter()
        .filter_map(|m| m.embedding.as_ref().map(|e| (m, e)))
        .collect();

    if with_embedding.is_empty() {
        return Ok(vec![]);
    }

    let mut jarred_pairs = Vec::new();
    for (mash, embedding) in &with_embedding {
        match find_similar_mashes(conn, &mash.id, embedding, pipeline_top_k, threshold) {
            Ok(pairs) => jarred_pairs.extend(pairs),
            Err(e) => log::warn!("Similarity search failed for {}: {}", mash.id, e),
        }
    }

    let batch_items: Vec<(String, Vec<f32>)> = with_embedding
        .iter()
        .map(|(m, e)| (m.id.clone(), (*e).clone()))
        .collect();
    let batch_pairs = find_similar_in_batch(&batch_items, threshold, pipeline_top_k);

    let mut seen = HashSet::new();
    let mut all_pairs = Vec::new();
    for pair in jarred_pairs.into_iter().chain(batch_pairs) {
        let mut key = vec![pair.source_id.clone(), pair.target_id.clone()];
        key.sort();
        let key_str = key.join(":");
        if seen.insert(key_str) {
            all_pairs.push(pair);
        }
    }

    log::info!("Found {} unique similar pairs", all_pairs.len());
    Ok(all_pairs)
}

/// RE_EXTRACT: re-extract relationships only (embeddings already exist), then go to JARRED.
pub async fn reextract_mashes(
    conn: &Arc<Mutex<Connection>>,
    config: &EmbeddingConfig,
    progress: &Arc<Mutex<Option<PipelineProgress>>>,
) -> Result<u32, String> {
    let targets: Vec<DistilledMash> = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, summary, embedding FROM mashes WHERE status = 'RE_EXTRACT'")
            .map_err(|e| e.to_string())?;
        let result = stmt
            .query_map([], |row| {
                let embedding_blob: Option<Vec<u8>> = row.get(2)?;
                Ok(DistilledMash {
                    id: row.get(0)?,
                    summary: row.get(1)?,
                    embedding: embedding_blob.map(|b| deserialize_embedding(&b)),
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());
        result?
    };

    if targets.is_empty() {
        return Ok(0);
    }

    let total = targets.len() as u32;
    log::info!("Re-extracting relationships for {} mashes", total);

    // Delete AI edges only for RE_EXTRACT target mashes (not all edges globally)
    {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        let deleted = conn
            .execute(
                "DELETE FROM edges WHERE source = 'ai'
                 AND (source_id IN (SELECT id FROM mashes WHERE status = 'RE_EXTRACT')
                   OR target_id IN (SELECT id FROM mashes WHERE status = 'RE_EXTRACT'))",
                [],
            )
            .map_err(|e| e.to_string())?;
        log::info!("Re-extract: deleted {} AI edges for target mashes", deleted);
    }

    update_progress(progress, "re_extract", "similarity", 0, total);

    let (threshold, pipeline_top_k) = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        let threshold = crate::db::settings::get_setting(&conn, "pipeline_threshold")?
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.3);
        let top_k = crate::db::settings::get_setting(&conn, "pipeline_top_k")?
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(5);
        (threshold, top_k)
    };

    let similar_pairs = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        find_all_similar_pairs_reextract(&conn, &targets, threshold, pipeline_top_k)?
    };
    log::info!("Found {} similar pairs for re-extraction", similar_pairs.len());

    let candidates = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        build_candidates(&conn, &similar_pairs, &targets)?
    };
    log::info!("Built {} candidates for re-extraction", candidates.len());

    // Process candidates in batches: API call → save edges → update mash status per batch
    let mut edges_created = 0u32;
    let mut processed_mash_ids = std::collections::HashSet::new();
    let mut count = 0u32;
    let batches: Vec<&[RelationCandidate]> = candidates.chunks(5).collect();
    let total_batches = batches.len();

    for (batch_idx, batch) in batches.into_iter().enumerate() {
        update_progress(progress, "re_extract", "api", batch_idx as u32, total_batches as u32);

        let relations = match extract_batch(config, batch).await {
            Ok(r) => r,
            Err(e) => {
                log::warn!("Relationship extraction batch failed, skipping: {}", e);
                continue;
            }
        };

        let now = now_ms();
        let conn = conn.lock().map_err(|e| e.to_string())?;

        // Save edges
        for rel in &relations {
            let result = conn.execute(
                "INSERT INTO edges (source_id, target_id, relation_type, source, confidence, created_at, updated_at)
                 VALUES (?1, ?2, ?3, 'ai', ?4, ?5, ?6)
                 ON CONFLICT(source_id, target_id) DO UPDATE SET
                    relation_type = excluded.relation_type,
                    confidence = excluded.confidence,
                    updated_at = excluded.updated_at
                 WHERE edges.source = 'ai'",
                params![rel.source_id, rel.target_id, rel.relation, rel.confidence, now, now],
            );
            match result {
                Ok(_) => edges_created += 1,
                Err(e) => log::warn!("Failed to create edge: {}", e),
            }
        }

        // Update mash status for mashes involved in this batch
        for candidate in batch {
            for id in [&candidate.source_id, &candidate.target_id] {
                if processed_mash_ids.insert(id.clone()) {
                    if targets.iter().any(|t| t.id == *id) {
                        conn.execute(
                            "UPDATE mashes SET status = 'JARRED', updated_at = ?1 WHERE id = ?2 AND status = 'RE_EXTRACT'",
                            params![now, id],
                        )
                        .map_err(|e| e.to_string())?;
                        count += 1;
                    }
                }
            }
        }
    }

    // Update remaining mashes that had no candidates (no similar pairs found)
    {
        let now = now_ms();
        let conn = conn.lock().map_err(|e| e.to_string())?;
        for mash in &targets {
            if !processed_mash_ids.contains(&mash.id) {
                conn.execute(
                    "UPDATE mashes SET status = 'JARRED', updated_at = ?1 WHERE id = ?2 AND status = 'RE_EXTRACT'",
                    params![now, mash.id],
                )
                .map_err(|e| e.to_string())?;
                count += 1;
            }
        }
    }

    update_progress(progress, "re_extract", "saving", total, total);

    log::info!("Re-extraction complete: {} mashes, {} edges", count, edges_created);
    Ok(count)
}

/// For RE_EXTRACT: find similar pairs among the targets and existing JARRED mashes.
fn find_all_similar_pairs_reextract(
    conn: &Connection,
    targets: &[DistilledMash],
    threshold: f32,
    pipeline_top_k: usize,
) -> Result<Vec<SimilarPair>, String> {
    let with_embedding: Vec<(&DistilledMash, &Vec<f32>)> = targets
        .iter()
        .filter_map(|m| m.embedding.as_ref().map(|e| (m, e)))
        .collect();

    if with_embedding.is_empty() {
        return Ok(vec![]);
    }

    let mut all_jarred_pairs = Vec::new();

    let batch_items: Vec<(String, Vec<f32>)> = with_embedding
        .iter()
        .map(|(m, e)| (m.id.clone(), (*e).clone()))
        .collect();
    let batch_pairs = find_similar_in_batch(&batch_items, threshold, pipeline_top_k);

    for (mash, embedding) in &with_embedding {
        match find_similar_mashes(conn, &mash.id, embedding, pipeline_top_k, threshold) {
            Ok(pairs) => all_jarred_pairs.extend(pairs),
            Err(e) => log::warn!("Similarity search failed for {}: {}", mash.id, e),
        }
    }

    let mut seen = HashSet::new();
    let mut all_pairs = Vec::new();
    for pair in all_jarred_pairs.into_iter().chain(batch_pairs) {
        let mut key = vec![pair.source_id.clone(), pair.target_id.clone()];
        key.sort();
        let key_str = key.join(":");
        if seen.insert(key_str) {
            all_pairs.push(pair);
        }
    }

    Ok(all_pairs)
}

fn build_candidates(
    conn: &Connection,
    pairs: &[SimilarPair],
    distilled: &[DistilledMash],
) -> Result<Vec<RelationCandidate>, String> {
    if pairs.is_empty() {
        return Ok(vec![]);
    }

    let mut summary_map: HashMap<String, String> =
        distilled.iter().map(|m| (m.id.clone(), m.summary.clone())).collect();

    let missing_ids: Vec<String> = pairs
        .iter()
        .flat_map(|p| vec![p.source_id.clone(), p.target_id.clone()])
        .filter(|id| !summary_map.contains_key(id))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    for id in &missing_ids {
        let mut stmt = conn
            .prepare("SELECT summary FROM mashes WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        if let Ok(summary) = stmt.query_row(params![id], |row| row.get::<_, String>(0)) {
            summary_map.insert(id.clone(), summary);
        }
    }

    Ok(pairs
        .iter()
        .filter(|p| summary_map.contains_key(&p.source_id) && summary_map.contains_key(&p.target_id))
        .map(|p| RelationCandidate {
            source_id: p.source_id.clone(),
            source_summary: summary_map[&p.source_id].clone(),
            target_id: p.target_id.clone(),
            target_summary: summary_map[&p.target_id].clone(),
        })
        .collect())
}
