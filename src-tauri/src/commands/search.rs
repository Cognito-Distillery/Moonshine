use tauri::State;

use crate::ai::embedding::{resolve_embedding_config, EmbeddingTaskType};
use crate::commands::DbState;
use crate::db;
use crate::models::{GraphData, Mash, RecentSearch};

#[tauri::command]
pub fn search_keyword(
    state: State<DbState>,
    query: String,
) -> Result<Vec<Mash>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mashes::search_mashes(&conn, &query)
}

#[tauri::command]
pub async fn search_semantic(
    state: State<'_, DbState>,
    query: String,
) -> Result<GraphData, String> {
    let (config, cached) = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let config = resolve_embedding_config(&conn)?;
        let provider = config.provider.as_str().to_string();
        let cached = db::search_cache::find_cached_embedding(&conn, &query, &provider)?;
        (config, cached)
    };

    let embedding = if let Some((_cache_id, emb, _result_ids)) = &cached {
        log::info!("Search cache hit for query: {}", query);
        emb.clone()
    } else {
        log::info!("Search cache miss for query: {}", query);
        let embeddings = crate::ai::embedding::generate_embeddings(
            &config,
            vec![query.clone()],
            EmbeddingTaskType::Query,
        )
        .await?;
        embeddings
            .into_iter()
            .next()
            .flatten()
            .ok_or_else(|| "Failed to generate query embedding".to_string())?
    };

    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let threshold = db::settings::get_setting(&conn, "search_threshold")?
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.3);
    let search_top_k = db::settings::get_setting(&conn, "search_top_k")?
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10);

    let similar = crate::similarity::find_similar_mashes(&conn, "", &embedding, search_top_k, threshold)?;

    let result_ids: Vec<String> = similar.iter().map(|s| s.target_id.clone()).collect();

    if cached.is_some() {
        // Update result_ids on cache hit (results may have changed)
        let cache_id = cached.unwrap().0;
        db::search_cache::update_cache_results(&conn, cache_id, &result_ids)?;
    } else {
        // Save new cache entry
        let provider = config.provider.as_str();
        db::search_cache::save_search_cache(&conn, &query, provider, &embedding, &result_ids)?;
    }

    if similar.is_empty() {
        return Ok(GraphData {
            nodes: vec![],
            edges: vec![],
        });
    }

    let first_id = &similar[0].target_id;
    db::edges::get_node_neighbors(&conn, first_id)
}

#[tauri::command]
pub fn get_recent_searches(state: State<DbState>) -> Result<Vec<RecentSearch>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::search_cache::get_recent_searches(&conn, 30)
}

#[tauri::command]
pub fn replay_cached_search(
    state: State<DbState>,
    cache_id: i64,
) -> Result<GraphData, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let threshold = db::settings::get_setting(&conn, "search_threshold")?
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.3);
    let search_top_k = db::settings::get_setting(&conn, "search_top_k")?
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10);

    let embedding = db::search_cache::get_cached_embedding(&conn, cache_id)?
        .ok_or_else(|| "Cache entry not found".to_string())?;

    let similar = crate::similarity::find_similar_mashes(&conn, "", &embedding, search_top_k, threshold)?;

    // Update result_ids
    let result_ids: Vec<String> = similar.iter().map(|s| s.target_id.clone()).collect();
    db::search_cache::update_cache_results(&conn, cache_id, &result_ids)?;

    if similar.is_empty() {
        return Ok(GraphData {
            nodes: vec![],
            edges: vec![],
        });
    }

    let first_id = &similar[0].target_id;
    db::edges::get_node_neighbors(&conn, first_id)
}

#[tauri::command]
pub fn delete_cached_search(state: State<DbState>, cache_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::search_cache::delete_search_cache(&conn, cache_id)
}
