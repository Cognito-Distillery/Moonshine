use rusqlite::{params, Connection, OptionalExtension};

use crate::db::now_ms;
use crate::models::RecentSearch;
use crate::similarity::{deserialize_embedding, serialize_embedding};

const MAX_CACHE_SIZE: i64 = 30;

/// Find a cached embedding for an exact query+provider match.
/// Returns (cache_id, embedding, result_ids).
pub fn find_cached_embedding(
    conn: &Connection,
    query: &str,
    provider: &str,
) -> Result<Option<(i64, Vec<f32>, Vec<String>)>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, embedding, result_ids FROM search_cache
             WHERE query = ?1 AND provider = ?2
             ORDER BY created_at DESC LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_row(params![query, provider], |row| {
            let id: i64 = row.get(0)?;
            let blob: Vec<u8> = row.get(1)?;
            let result_ids_json: String = row.get(2)?;
            Ok((id, blob, result_ids_json))
        })
        .optional()
        .map_err(|e| e.to_string())?;

    match result {
        Some((id, blob, json)) => {
            let embedding = deserialize_embedding(&blob);
            let result_ids: Vec<String> =
                serde_json::from_str(&json).unwrap_or_default();
            Ok(Some((id, embedding, result_ids)))
        }
        None => Ok(None),
    }
}

/// Save a search cache entry. Enforces max cache size by deleting oldest entries.
pub fn save_search_cache(
    conn: &Connection,
    query: &str,
    provider: &str,
    embedding: &[f32],
    result_ids: &[String],
) -> Result<i64, String> {
    let blob = serialize_embedding(embedding);
    let result_ids_json = serde_json::to_string(result_ids).map_err(|e| e.to_string())?;
    let now = now_ms();

    conn.execute(
        "INSERT INTO search_cache (query, provider, embedding, result_ids, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![query, provider, blob, result_ids_json, now],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    // Enforce max cache size
    conn.execute(
        "DELETE FROM search_cache WHERE id NOT IN (
            SELECT id FROM search_cache ORDER BY created_at DESC LIMIT ?1
        )",
        params![MAX_CACHE_SIZE],
    )
    .map_err(|e| e.to_string())?;

    Ok(id)
}

/// Update result_ids for an existing cache entry.
pub fn update_cache_results(
    conn: &Connection,
    cache_id: i64,
    result_ids: &[String],
) -> Result<(), String> {
    let result_ids_json = serde_json::to_string(result_ids).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE search_cache SET result_ids = ?1 WHERE id = ?2",
        params![result_ids_json, cache_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Get recent search entries for display.
pub fn get_recent_searches(
    conn: &Connection,
    limit: usize,
) -> Result<Vec<RecentSearch>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, query, result_ids, created_at FROM search_cache
             ORDER BY created_at DESC LIMIT ?1",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![limit as i64], |row| {
            let id: i64 = row.get(0)?;
            let query: String = row.get(1)?;
            let result_ids_json: String = row.get(2)?;
            let created_at: i64 = row.get(3)?;
            Ok((id, query, result_ids_json, created_at))
        })
        .map_err(|e| e.to_string())?;

    let mut searches = Vec::new();
    for row in rows {
        let (id, query, json, created_at) = row.map_err(|e| e.to_string())?;
        let result_ids: Vec<String> = serde_json::from_str(&json).unwrap_or_default();
        searches.push(RecentSearch {
            id,
            query,
            result_count: result_ids.len(),
            created_at,
        });
    }
    Ok(searches)
}

/// Delete a single cache entry.
pub fn delete_search_cache(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM search_cache WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Get cached embedding by cache ID (for replay).
pub fn get_cached_embedding(conn: &Connection, id: i64) -> Result<Option<Vec<f32>>, String> {
    let mut stmt = conn
        .prepare("SELECT embedding FROM search_cache WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_row(params![id], |row| {
            let blob: Vec<u8> = row.get(0)?;
            Ok(blob)
        })
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(result.map(|blob| deserialize_embedding(&blob)))
}
