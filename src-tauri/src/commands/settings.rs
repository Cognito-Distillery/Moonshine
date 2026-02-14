use tauri::State;

use crate::ai::embedding::{
    default_chat_model, default_embedding_model, resolve_embedding_config, EmbeddingProvider,
};
use crate::commands::DbState;
use crate::db;
use crate::pipeline::scheduler::PipelineSchedulerState;

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
pub async fn switch_embedding_provider(
    db_state: State<'_, DbState>,
    scheduler_state: State<'_, PipelineSchedulerState>,
    provider: String,
) -> Result<u32, String> {
    let new_provider = EmbeddingProvider::from_str(&provider)?;

    let reset_count = {
        let conn = db_state.0.lock().map_err(|e| e.to_string())?;

        let current = db::settings::get_setting(&conn, "embedding_provider")?
            .unwrap_or_else(|| "openai".to_string());

        if current == provider {
            return Ok(0);
        }

        let reset_count = db::mashes::reset_for_reembed(&conn)?;
        db::settings::set_setting(&conn, "embedding_provider", &provider)?;

        db::settings::set_setting(
            &conn,
            "embedding_model",
            default_embedding_model(&new_provider),
        )?;
        db::settings::set_setting(&conn, "chat_model", default_chat_model(&new_provider))?;

        reset_count
    };

    // Auto-trigger pipeline
    if reset_count > 0 {
        if let Ok(config) = {
            let conn = db_state.0.lock().map_err(|e| e.to_string())?;
            resolve_embedding_config(&conn)
        } {
            let _ = scheduler_state.trigger_now(&db_state.0, &config).await;
        }
    }

    Ok(reset_count)
}

#[tauri::command]
pub async fn switch_embedding_model(
    db_state: State<'_, DbState>,
    scheduler_state: State<'_, PipelineSchedulerState>,
    model: String,
) -> Result<u32, String> {
    let reset_count = {
        let conn = db_state.0.lock().map_err(|e| e.to_string())?;

        let current = db::settings::get_setting(&conn, "embedding_model")?;
        if current.as_deref() == Some(&model) {
            return Ok(0);
        }

        let reset_count = db::mashes::reset_for_reembed(&conn)?;
        db::settings::set_setting(&conn, "embedding_model", &model)?;

        reset_count
    };

    if reset_count > 0 {
        if let Ok(config) = {
            let conn = db_state.0.lock().map_err(|e| e.to_string())?;
            resolve_embedding_config(&conn)
        } {
            let _ = scheduler_state.trigger_now(&db_state.0, &config).await;
        }
    }

    Ok(reset_count)
}

#[tauri::command]
pub fn switch_chat_model(state: State<DbState>, model: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::settings::set_setting(&conn, "chat_model", &model)
}

#[tauri::command]
pub async fn reextract_relationships(
    db_state: State<'_, DbState>,
    scheduler_state: State<'_, PipelineSchedulerState>,
) -> Result<u32, String> {
    let reset_count = {
        let conn = db_state.0.lock().map_err(|e| e.to_string())?;
        // Delete AI-generated edges only; human-created edges are preserved
        db::edges::delete_ai_edges(&conn)?;
        db::mashes::reset_for_reextract(&conn)?
    };

    if reset_count > 0 {
        if let Ok(config) = {
            let conn = db_state.0.lock().map_err(|e| e.to_string())?;
            resolve_embedding_config(&conn)
        } {
            let _ = scheduler_state.trigger_now(&db_state.0, &config).await;
        }
    }

    Ok(reset_count)
}
