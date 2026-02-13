use rusqlite::{params, Connection};

use crate::ai::embedding::{generate_embeddings, EmbeddingConfig, EmbeddingTaskType};
use crate::db::now_ms;
use crate::similarity::serialize_embedding;

pub async fn distill_mashes(
    conn: &std::sync::Arc<std::sync::Mutex<Connection>>,
    config: &EmbeddingConfig,
) -> Result<u32, String> {
    // Step 1: Read ON_STILL mashes (sync)
    let mashes: Vec<(String, String, String, String)> = {
        let conn = conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, summary, context, memo FROM mashes WHERE status = 'ON_STILL'")
            .map_err(|e| e.to_string())?;
        let result = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());
        result?
    };

    if mashes.is_empty() {
        log::info!("No mashes to distill");
        return Ok(0);
    }

    log::info!("Distilling {} mashes", mashes.len());

    // Step 2: Generate embeddings (async) — combine summary + context + memo
    let texts: Vec<String> = mashes
        .iter()
        .map(|(_, summary, context, memo)| {
            let mut parts = vec![summary.as_str()];
            if !context.is_empty() {
                parts.push(context.as_str());
            }
            if !memo.is_empty() {
                parts.push(memo.as_str());
            }
            parts.join("\n")
        })
        .collect();
    let embeddings = generate_embeddings(config, texts, EmbeddingTaskType::Document).await?;

    // Step 3: Write results back (sync)
    let conn = conn.lock().map_err(|e| e.to_string())?;
    let now = now_ms();
    let mut distilled = 0u32;

    for (i, (id, _, _, _)) in mashes.iter().enumerate() {
        if let Some(Some(ref embedding)) = embeddings.get(i) {
            let blob = serialize_embedding(embedding);
            conn.execute(
                "UPDATE mashes SET status = 'DISTILLED', embedding = ?1, updated_at = ?2 WHERE id = ?3",
                params![blob, now, id],
            )
            .map_err(|e| e.to_string())?;
            distilled += 1;
        }
    }

    log::info!("Distillation complete: {} mashes", distilled);
    Ok(distilled)
}
