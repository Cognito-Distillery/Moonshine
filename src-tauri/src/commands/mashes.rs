use tauri::State;

use crate::commands::DbState;
use crate::db;
use crate::models::Mash;

#[tauri::command]
pub fn get_mashes_by_status(
    state: State<DbState>,
    status: String,
    query: Option<String>,
) -> Result<Vec<Mash>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mashes::get_mashes_by_status(&conn, &status, query.as_deref())
}

#[tauri::command]
pub fn add_mash(
    state: State<DbState>,
    mash_type: String,
    summary: String,
    context: String,
    memo: String,
) -> Result<Mash, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mashes::add_mash(&conn, &mash_type, &summary, &context, &memo)
}

#[tauri::command]
pub fn delete_mash(state: State<DbState>, id: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mashes::delete_mash(&conn, &id)
}

#[tauri::command]
pub fn update_mash(
    state: State<DbState>,
    id: String,
    mash_type: Option<String>,
    summary: Option<String>,
    context: Option<String>,
    memo: Option<String>,
) -> Result<Mash, String> {
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

#[tauri::command]
pub fn set_mash_status(
    state: State<DbState>,
    id: String,
    status: String,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mashes::set_mash_status(&conn, &id, &status)
}

#[tauri::command]
pub fn search_mashes(state: State<DbState>, query: String) -> Result<Vec<Mash>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::mashes::search_mashes(&conn, &query)
}
