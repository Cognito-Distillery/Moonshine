use tauri::State;

use crate::ai::embedding::{default_chat_model, default_embedding_model, EmbeddingProvider};
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
    let new_provider = EmbeddingProvider::from_str(&provider)?;

    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let current = db::settings::get_setting(&conn, "embedding_provider")?
        .unwrap_or_else(|| "openai".to_string());

    if current == provider {
        return Ok(0);
    }

    let reset_count = db::mashes::reset_all_embeddings(&conn)?;
    db::settings::set_setting(&conn, "embedding_provider", &provider)?;

    // Reset models to defaults for the new provider
    db::settings::set_setting(
        &conn,
        "embedding_model",
        default_embedding_model(&new_provider),
    )?;
    db::settings::set_setting(&conn, "chat_model", default_chat_model(&new_provider))?;

    Ok(reset_count)
}

#[tauri::command]
pub fn switch_embedding_model(state: State<DbState>, model: String) -> Result<u32, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let current = db::settings::get_setting(&conn, "embedding_model")?;
    if current.as_deref() == Some(&model) {
        return Ok(0);
    }

    let reset_count = db::mashes::reset_all_embeddings(&conn)?;
    db::settings::set_setting(&conn, "embedding_model", &model)?;

    Ok(reset_count)
}

#[tauri::command]
pub fn switch_chat_model(state: State<DbState>, model: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::settings::set_setting(&conn, "chat_model", &model)
}
