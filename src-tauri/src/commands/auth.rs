use tauri::State;

use crate::commands::DbState;
use crate::db;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    pub password_set: bool,
    pub authenticated: bool,
}

// In-memory session token
static SESSION: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

#[tauri::command]
pub fn check_auth(state: State<DbState>) -> Result<AuthStatus, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let password_set = db::settings::get_setting(&conn, "password_hash")?.is_some();
    let session = SESSION.lock().map_err(|e| e.to_string())?;
    let authenticated = !password_set || session.is_some();
    Ok(AuthStatus {
        password_set,
        authenticated,
    })
}

#[tauri::command]
pub fn login(state: State<DbState>, password: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let hash = db::settings::get_setting(&conn, "password_hash")?
        .ok_or_else(|| "No password set".to_string())?;

    let valid =
        bcrypt::verify(&password, &hash).map_err(|e| e.to_string())?;

    if valid {
        let token = uuid::Uuid::new_v4().to_string();
        let mut session = SESSION.lock().map_err(|e| e.to_string())?;
        *session = Some(token);
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub fn set_password(
    state: State<DbState>,
    current: Option<String>,
    new_password: String,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let existing_hash = db::settings::get_setting(&conn, "password_hash")?;

    if let Some(ref hash) = existing_hash {
        let current = current.ok_or_else(|| "Current password required".to_string())?;
        let valid = bcrypt::verify(&current, hash).map_err(|e| e.to_string())?;
        if !valid {
            return Err("Current password is incorrect".to_string());
        }
    }

    let new_hash = bcrypt::hash(&new_password, 10).map_err(|e| e.to_string())?;
    db::settings::set_setting(&conn, "password_hash", &new_hash)?;

    // Auto-authenticate after setting password
    let token = uuid::Uuid::new_v4().to_string();
    let mut session = SESSION.lock().map_err(|e| e.to_string())?;
    *session = Some(token);

    Ok(())
}

#[tauri::command]
pub fn remove_password(state: State<DbState>, current: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let hash = db::settings::get_setting(&conn, "password_hash")?
        .ok_or_else(|| "No password set".to_string())?;

    let valid = bcrypt::verify(&current, &hash).map_err(|e| e.to_string())?;
    if !valid {
        return Err("Password is incorrect".to_string());
    }

    db::settings::delete_setting(&conn, "password_hash")?;
    let mut session = SESSION.lock().map_err(|e| e.to_string())?;
    *session = None;
    Ok(())
}

#[tauri::command]
pub fn logout() -> Result<(), String> {
    let mut session = SESSION.lock().map_err(|e| e.to_string())?;
    *session = None;
    Ok(())
}
