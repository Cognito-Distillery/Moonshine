use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection};

use crate::ai::embedding::EmbeddingConfig;
use crate::ai::relationship::{extract_relations, RelationCandidate};
use crate::db::now_ms;
use crate::models::SimilarPair;
use crate::similarity::{deserialize_embedding, find_similar_in_batch, find_similar_mashes};

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

    // Step 2: Find similar pairs (sync - all in-memory)
    let similar_pairs = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        find_all_similar_pairs(&conn, &distilled)?
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
        match find_similar_mashes(conn, &mash.id, embedding, 5, 0.3) {
            Ok(pairs) => jarred_pairs.extend(pairs),
            Err(e) => log::warn!("Similarity search failed for {}: {}", mash.id, e),
        }
    }

    let batch_items: Vec<(String, Vec<f32>)> = with_embedding
        .iter()
        .map(|(m, e)| (m.id.clone(), (*e).clone()))
        .collect();
    let batch_pairs = find_similar_in_batch(&batch_items, 0.3);

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
