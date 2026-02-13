use tauri::State;

use crate::commands::DbState;
use crate::db;
use crate::models::{Edge, GraphData, GraphFilters};

#[tauri::command]
pub fn get_graph(
    state: State<DbState>,
    filters: GraphFilters,
) -> Result<GraphData, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::edges::get_graph(&conn, &filters)
}

#[tauri::command]
pub fn get_node_detail(
    state: State<DbState>,
    id: String,
) -> Result<GraphData, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::edges::get_node_neighbors(&conn, &id)
}

#[tauri::command]
pub fn expand_node(
    state: State<DbState>,
    id: String,
    depth: u32,
) -> Result<GraphData, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::edges::expand_node(&conn, &id, depth)
}

#[tauri::command]
pub fn add_edge(
    state: State<DbState>,
    source_id: String,
    target_id: String,
    relation_type: String,
    source: String,
    confidence: f64,
) -> Result<Edge, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::edges::add_edge(&conn, &source_id, &target_id, &relation_type, &source, confidence)
}

#[tauri::command]
pub fn update_edge(
    state: State<DbState>,
    id: i64,
    relation_type: Option<String>,
    confidence: Option<f64>,
) -> Result<Edge, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::edges::update_edge(&conn, id, relation_type.as_deref(), confidence)
}

#[tauri::command]
pub fn delete_edge(
    state: State<DbState>,
    id: i64,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::edges::delete_edge(&conn, id)
}

#[tauri::command]
pub fn update_node(
    state: State<DbState>,
    id: String,
    mash_type: Option<String>,
    summary: Option<String>,
    context: Option<String>,
    memo: Option<String>,
) -> Result<crate::models::Mash, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mashes::update_mash(
        &conn,
        &id,
        mash_type.as_deref(),
        summary.as_deref(),
        context.as_deref(),
        memo.as_deref(),
    )
}
