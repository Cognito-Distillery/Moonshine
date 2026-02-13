use tauri::State;

use crate::ai::embedding::resolve_embedding_config;
use crate::commands::DbState;
use crate::db;
use crate::models::PipelineStatus;
use crate::pipeline::scheduler::PipelineSchedulerState;

#[tauri::command]
pub async fn trigger_pipeline(
    db_state: State<'_, DbState>,
    scheduler_state: State<'_, PipelineSchedulerState>,
) -> Result<(), String> {
    let config = {
        let conn = db_state.0.lock().map_err(|e| e.to_string())?;
        resolve_embedding_config(&conn)?
    };

    scheduler_state.trigger_now(&db_state.0, &config).await
}

#[tauri::command]
pub async fn set_pipeline_interval(
    db_state: State<'_, DbState>,
    scheduler_state: State<'_, PipelineSchedulerState>,
    minutes: u64,
) -> Result<(), String> {
    if !(5..=60).contains(&minutes) {
        return Err("Interval must be between 5 and 60 minutes".to_string());
    }

    {
        let conn = db_state.0.lock().map_err(|e| e.to_string())?;
        db::settings::set_setting(&conn, "pipeline_interval_min", &minutes.to_string())?;
    }

    scheduler_state.update_interval(minutes);
    Ok(())
}

#[tauri::command]
pub fn get_pipeline_status(
    db_state: State<DbState>,
    scheduler_state: State<PipelineSchedulerState>,
) -> Result<PipelineStatus, String> {
    let conn = db_state.0.lock().map_err(|e| e.to_string())?;

    let interval_min = db::settings::get_setting(&conn, "pipeline_interval_min")?
        .and_then(|v| v.parse().ok())
        .unwrap_or(30u64);

    let last_run = db::settings::get_setting(&conn, "pipeline_last_run")?
        .and_then(|v| v.parse().ok());

    let on_still_count = {
        let mashes = db::mashes::get_mashes_by_status(&conn, "ON_STILL", None)?;
        mashes.len() as u32
    };

    let distilled_count = {
        let mashes = db::mashes::get_mashes_by_status(&conn, "DISTILLED", None)?;
        mashes.len() as u32
    };

    let jarred_count = {
        let mashes = db::mashes::get_mashes_by_status(&conn, "JARRED", None)?;
        mashes.len() as u32
    };

    let next_run = scheduler_state.next_run();

    let running = scheduler_state.is_running();

    Ok(PipelineStatus {
        last_run,
        next_run,
        interval_min,
        distilled_count,
        jarred_count,
        on_still_count,
        running,
    })
}
