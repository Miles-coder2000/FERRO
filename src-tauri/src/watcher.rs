use crate::models::FolderUpdatePayload;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use walkdir::WalkDir;

pub struct WatcherState {
    pub watcher: Mutex<Option<RecommendedWatcher>>,
}

impl WatcherState {
    pub fn new() -> Self {
        Self {
            watcher: Mutex::new(None),
        }
    }
}

pub fn compute_folder_stats(dir_path: &str) -> (usize, u64) {
    let mut file_count = 0usize;
    let mut total_size = 0u64;

    for entry in WalkDir::new(dir_path)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            file_count += 1;
            if let Ok(meta) = entry.metadata() {
                total_size += meta.len();
            }
        }
    }
    (file_count, total_size)
}

#[tauri::command]
pub fn watch_pinned_folders(
    paths: Vec<String>,
    app: AppHandle,
    state: State<'_, WatcherState>,
) -> Result<(), String> {
    let watched_paths = paths.clone();
    let app_handle = app.clone();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                for changed_path in &event.paths {
                    for root in &watched_paths {
                        if changed_path.starts_with(Path::new(root)) {
                            let (file_count, total_size_bytes) = compute_folder_stats(root);
                            let _ = app_handle.emit(
                                "folder-updated",
                                FolderUpdatePayload {
                                    path: root.clone(),
                                    file_count,
                                    total_size_bytes,
                                },
                            );
                            break;
                        }
                    }
                }
            }
        },
        Config::default(),
    )
    .map_err(|e| format!("Failed to initialize watcher: {}", e))?;

    for p in &paths {
        let path_obj = Path::new(p);
        if path_obj.exists() {
            let _ = watcher.watch(path_obj, RecursiveMode::NonRecursive);
        }
    }

    let mut guard = state.watcher.lock().map_err(|e| e.to_string())?;
    *guard = Some(watcher);
    Ok(())
}
