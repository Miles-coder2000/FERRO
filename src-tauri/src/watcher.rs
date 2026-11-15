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