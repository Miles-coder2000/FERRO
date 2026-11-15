use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub icon_path: Option<String>,
    pub category: String,
    pub launch_count: u32,
    pub last_launched: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameEntry {
    pub id: String,
    pub title: String,
    pub platform: String,
    pub executable_path: String,
    pub launch_args: Vec<String>,
    pub cover_image: Option<String>,
    pub icon_path: Option<String>,
    pub launch_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderCard {
    pub id: String,
    pub name: String,
    pub path: String,
    pub file_count: usize,
    pub total_size_bytes: u64,
    pub is_pinned: bool,
    pub icon_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub app_ids: Vec<String>,
    pub folder_paths: Vec<String>,
    pub shortcut_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetrySnapshot {
    pub cpu_usage_percent: f32,
    pub ram_usage_mb: f32,
    pub indexed_items_count: usize,
    pub last_launch_latency_ms: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderUpdatePayload {
    pub path: String,
    pub file_count: usize,
    pub total_size_bytes: u64,
}
