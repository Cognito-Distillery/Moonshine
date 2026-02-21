mod ai;
mod commands;
mod db;
mod models;
mod pipeline;
mod similarity;

use std::sync::{Arc, Mutex};

use commands::DbState;
use pipeline::scheduler::PipelineSchedulerState;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn toggle_floating_memo(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("floating-memo") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    } else {
        let _ = tauri::WebviewWindowBuilder::new(
            app,
            "floating-memo",
            tauri::WebviewUrl::App("/floating-memo".into()),
        )
        .title("Quick Mash")
        .inner_size(480.0, 360.0)
        .always_on_top(true)
        .decorations(false)
        .center()
        .skip_taskbar(true)
        .build();
    }
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[cfg(target_os = "linux")]
struct DbusInterface {
    app: tauri::AppHandle,
}

#[cfg(target_os = "linux")]
#[zbus::interface(name = "com.moonshine.App")]
impl DbusInterface {
    fn toggle_floating_memo(&self) {
        toggle_floating_memo(&self.app);
    }
}

#[tauri::command]
fn hide_floating_memo(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("floating-memo") {
        let _ = window.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");
            let conn = db::init_db(&app_data_dir);

            // Read pipeline interval from settings
            let interval_min = db::settings::get_setting(&conn, "pipeline_interval_min")
                .ok()
                .flatten()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30u64);

            let conn = Arc::new(Mutex::new(conn));

            // Pipeline scheduler
            let scheduler = PipelineSchedulerState::new(interval_min);
            scheduler.spawn(conn.clone());

            app.manage(DbState(conn));
            app.manage(scheduler);

            // --- Tray icon ---
            let show = MenuItemBuilder::with_id("show", "Show Moonshine").build(app)?;
            let floating = MenuItemBuilder::with_id("floating", "Quick Mash").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show)
                .item(&floating)
                .separator()
                .item(&quit)
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Moonshine")
                .menu(&menu)
                .on_menu_event({
                    let handle = app.handle().clone();
                    move |_tray, event| match event.id().as_ref() {
                        "show" => show_main_window(&handle),
                        "floating" => toggle_floating_memo(&handle),
                        "quit" => {
                            handle.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event({
                    let handle = app.handle().clone();
                    move |_tray, event| {
                        if let tauri::tray::TrayIconEvent::Click { button, .. } = event {
                            if button == tauri::tray::MouseButton::Left {
                                show_main_window(&handle);
                            }
                        }
                    }
                })
                .build(app)?;

            // --- Global shortcut (X11 only, silently fails on Wayland) ---
            let shortcut = if cfg!(target_os = "macos") {
                Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyM)
            } else {
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM)
            };

            let _ = app.global_shortcut().on_shortcut(shortcut, {
                let handle = app.handle().clone();
                move |_app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        toggle_floating_memo(&handle);
                    }
                }
            });

            // --- DBus service for Wayland (Linux only) ---
            #[cfg(target_os = "linux")]
            {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    let iface = DbusInterface { app: handle };
                    let conn = zbus::connection::Builder::session()
                        .expect("failed to create DBus session builder")
                        .name("com.moonshine.App")
                        .expect("failed to set DBus name")
                        .serve_at("/com/moonshine/App", iface)
                        .expect("failed to serve DBus interface")
                        .build()
                        .await
                        .expect("failed to build DBus connection");
                    let _conn = conn;
                    std::future::pending::<()>().await;
                });
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            // Hide main window on close instead of quitting
            if window.label() == "main" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Mash CRUD
            commands::mashes::get_mashes_by_status,
            commands::mashes::add_mash,
            commands::mashes::add_mash_with_ai,
            commands::mashes::delete_mash,
            commands::mashes::update_mash,
            commands::mashes::set_mash_status,
            commands::mashes::search_mashes,
            // Graph
            commands::graph::get_graph,
            commands::graph::get_node_detail,
            commands::graph::expand_node,
            commands::graph::add_edge,
            commands::graph::update_edge,
            commands::graph::delete_edge,
            commands::graph::update_node,
            // Search
            commands::search::search_keyword,
            commands::search::search_semantic,
            commands::search::get_recent_searches,
            commands::search::replay_cached_search,
            commands::search::delete_cached_search,
            // Auth
            commands::auth::check_auth,
            commands::auth::login,
            commands::auth::set_password,
            commands::auth::remove_password,
            commands::auth::logout,
            // Settings
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::settings::switch_embedding_provider,
            commands::settings::switch_embedding_model,
            commands::settings::switch_chat_model,
            commands::settings::reextract_relationships,
            commands::settings::reembed_all,
            // Pipeline
            commands::pipeline::trigger_pipeline,
            commands::pipeline::set_pipeline_interval,
            commands::pipeline::get_pipeline_status,
            // Floating memo
            hide_floating_memo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
