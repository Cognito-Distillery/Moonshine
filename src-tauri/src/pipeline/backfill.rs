use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection};

use crate::ai::embedding::EmbeddingConfig;
use crate::ai::relationship::{extract_relations, RelationCandidate};
use crate::db::now_ms;
use crate::similarity::{deserialize_embedding, find_similar_mashes};

pub async fn backfill_isolated_nodes(
    conn: &Arc<Mutex<Connection>>,
    config: &EmbeddingConfig,
) -> Result<u32, String> {
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

    // Step 1: Find isolated JARRED nodes (sync)
    let (isolated, candidates) = {
        let conn = conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                "SELECT m.id, m.summary, m.embedding
                 FROM mashes m
                 WHERE m.status = 'JARRED'
                   AND m.embedding IS NOT NULL
                   AND m.id NOT IN (SELECT source_id FROM edges)
                   AND m.id NOT IN (SELECT target_id FROM edges)",
            )
            .map_err(|e| e.to_string())?;

        let isolated: Vec<(String, String, Vec<f32>)> = stmt
            .query_map([], |row| {
                let blob: Vec<u8> = row.get(2)?;
                Ok((row.get(0)?, row.get(1)?, deserialize_embedding(&blob)))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        if isolated.is_empty() {
            log::info!("Backfill: no isolated nodes");
            return Ok(0);
        }

        log::info!("Backfill: found {} isolated nodes", isolated.len());

        // Find similar pairs
        let mut all_pairs = Vec::new();
        for (id, _, embedding) in &isolated {
            match find_similar_mashes(&conn, id, embedding, pipeline_top_k, threshold) {
                Ok(pairs) => all_pairs.extend(pairs),
                Err(e) => log::warn!("Backfill similarity search failed for {}: {}", id, e),
            }
        }

        if all_pairs.is_empty() {
            log::info!("Backfill: no similar pairs found");
            return Ok(0);
        }

        // Build candidates
        let mut summary_map: HashMap<String, String> = isolated
            .iter()
            .map(|(id, summary, _)| (id.clone(), summary.clone()))
            .collect();

        let missing_ids: Vec<String> = all_pairs
            .iter()
            .map(|p| p.target_id.clone())
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

        let candidates: Vec<RelationCandidate> = all_pairs
            .iter()
            .filter(|p| {
                summary_map.contains_key(&p.source_id) && summary_map.contains_key(&p.target_id)
            })
            .map(|p| RelationCandidate {
                source_id: p.source_id.clone(),
                source_summary: summary_map[&p.source_id].clone(),
                target_id: p.target_id.clone(),
                target_summary: summary_map[&p.target_id].clone(),
            })
            .collect();

        (isolated, candidates)
    };

    // Step 2: AI relationship extraction (async)
    let relations = extract_relations(config, candidates).await?;

    // Step 3: Create edges (sync)
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
            Err(e) => log::warn!("Backfill: failed to create edge: {}", e),
        }
    }

    log::info!(
        "Backfill complete: {} isolated, {} edges created",
        isolated.len(),
        edges_created
    );
    Ok(edges_created)
}
