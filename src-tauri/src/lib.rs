mod db;
mod indexer;
mod launcher;
mod models;
mod telemetry;
mod watcher;
mod utils;

use db::Database;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use telemetry::TelemetryState;
use watcher::WatcherState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging: resolve log directory for environment-based routing
    let _log_dir = utils::get_log_dir();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let toggle_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
                        if shortcut == &toggle_shortcut {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            let app_data = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."));
            let db_path = app_data.join("ferro.db");
            let database = Database::new(db_path).expect("Failed to initialize SQLite database");

            app.manage(Mutex::new(database));
            app.manage(TelemetryState::new());
            app.manage(WatcherState::new());

            let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
            let _ = app.global_shortcut().register(shortcut);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            indexer::scan_installed_apps,
            indexer::scan_game_libraries,
            indexer::add_custom_game,
            indexer::get_folder_details,
            indexer::get_pinned_folders,
            indexer::pin_folder,
            indexer::unpin_folder,
            indexer::get_contexts,
            indexer::save_context,
            indexer::delete_context,
            indexer::search_indexed_items,
            watcher::watch_pinned_folders,
            launcher::launch_target,
            launcher::execute_shell_command,
            launcher::open_folder_in_explorer,
            launcher::reveal_file_location,
            telemetry::get_system_telemetry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running FERRO tauri application");
}