use rusqlite::{params, Connection};

use crate::models::SimilarPair;

pub fn serialize_embedding(vec: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vec.len() * 4);
    for &v in vec {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    bytes
}

pub fn deserialize_embedding(blob: &[u8]) -> Vec<f32> {
    blob.chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f64;
    let mut norm_a = 0.0f64;
    let mut norm_b = 0.0f64;
    for i in 0..a.len().min(b.len()) {
        let ai = a[i] as f64;
        let bi = b[i] as f64;
        dot += ai * bi;
        norm_a += ai * ai;
        norm_b += bi * bi;
    }
    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        (dot / denom) as f32
    }
}

pub fn find_similar_mashes(
    conn: &Connection,
    source_id: &str,
    embedding: &[f32],
    limit: usize,
    threshold: f32,
) -> Result<Vec<SimilarPair>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, embedding FROM mashes
             WHERE status = 'JARRED' AND embedding IS NOT NULL AND id != ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![source_id], |row| {
            let id: String = row.get(0)?;
            let blob: Vec<u8> = row.get(1)?;
            Ok((id, blob))
        })
        .map_err(|e| e.to_string())?;

    let mut pairs: Vec<SimilarPair> = Vec::new();
    for row in rows {
        let (id, blob) = row.map_err(|e| e.to_string())?;
        let other_embedding = deserialize_embedding(&blob);
        let sim = cosine_similarity(embedding, &other_embedding);
        if sim >= threshold {
            pairs.push(SimilarPair {
                source_id: source_id.to_string(),
                target_id: id,
                similarity: sim as f64,
            });
        }
    }

    pairs.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
    pairs.truncate(limit);
    Ok(pairs)
}

pub fn find_similar_in_batch(
    items: &[(String, Vec<f32>)],
    threshold: f32,
    limit: usize,
) -> Vec<SimilarPair> {
    use std::collections::HashMap;

    // Collect all pairs per source, keeping top-N
    let mut per_source: HashMap<usize, Vec<(usize, f32)>> = HashMap::new();

    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            let sim = cosine_similarity(&items[i].1, &items[j].1);
            if sim >= threshold {
                per_source.entry(i).or_default().push((j, sim));
                per_source.entry(j).or_default().push((i, sim));
            }
        }
    }

    // Keep top-N per source, deduplicate
    let mut seen = std::collections::HashSet::new();
    let mut pairs = Vec::new();

    for (src, mut candidates) in per_source {
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        candidates.truncate(limit);
        for (tgt, sim) in candidates {
            let (a, b) = if src < tgt { (src, tgt) } else { (tgt, src) };
            if seen.insert((a, b)) {
                pairs.push(SimilarPair {
                    source_id: items[a].0.clone(),
                    target_id: items[b].0.clone(),
                    similarity: sim as f64,
                });
            }
        }
    }

    pairs
}
