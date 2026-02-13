use tauri::State;

use crate::ai::embedding::EmbeddingProvider;
use crate::commands::DbState;
use crate::db;

#[tauri::command]
pub fn get_setting(state: State<DbState>, key: String) -> Result<Option<String>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::settings::get_setting(&conn, &key)
}

#[tauri::command]
pub fn set_setting(state: State<DbState>, key: String, value: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::settings::set_setting(&conn, &key, &value)
}

#[tauri::command]
pub fn get_all_settings(state: State<DbState>) -> Result<Vec<(String, String)>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::settings::get_all_settings(&conn)
}

#[tauri::command]
pub fn switch_embedding_provider(state: State<DbState>, provider: String) -> Result<u32, String> {
    EmbeddingProvider::from_str(&provider)?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let current = db::settings::get_setting(&conn, "embedding_provider")?
        .unwrap_or_else(|| "openai".to_string());

    if current == provider {
        return Ok(0);
    }

    let reset_count = db::mashes::reset_all_embeddings(&conn)?;
    db::settings::set_setting(&conn, "embedding_provider", &provider)?;

    Ok(reset_count)
}
