use crate::db::Database;
use crate::models::{AppEntry, ContextProfile, FolderCard, GameEntry};
use crate::telemetry::TelemetryState;
use crate::watcher::compute_folder_stats;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use walkdir::WalkDir;

fn hash_id(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.to_lowercase().hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn categorize_app(name: &str, path: &str) -> String {
    let lower = format!("{} {}", name, path).to_lowercase();
    if lower.contains("code")
        || lower.contains("studio")
        || lower.contains("git")
        || lower.contains("terminal")
        || lower.contains("powershell")
        || lower.contains("rust")
        || lower.contains("python")
        || lower.contains("node")
        || lower.contains("docker")
    {
        "Developer".to_string()
    } else if lower.contains("blender")
        || lower.contains("photoshop")
        || lower.contains("gimp")
        || lower.contains("obs")
        || lower.contains("figma")
        || lower.contains("audacity")
    {
        "Creative".to_string()
    } else if lower.contains("discord")
        || lower.contains("slack")
        || lower.contains("teams")
        || lower.contains("zoom")
        || lower.contains("chrome")
        || lower.contains("firefox")
        || lower.contains("edge")
        || lower.contains("brave")
    {
        "Comms & Web".to_string()
    } else {
        "System & Utility".to_string()
    }
}

#[cfg(target_os = "windows")]
fn extract_windows_icon(target_path: &str, icon_dir: &Path) -> Option<String> {
    use image::{ImageBuffer, Rgba};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, BITMAPINFO, BITMAPINFOHEADER,
        BI_RGB, DIB_RGB_COLORS,
    };
    use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL;
    use windows_sys::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
    use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, ICONINFO};

    let out_file = icon_dir.join(format!("{}.png", hash_id(target_path)));
    if out_file.exists() {
        return Some(out_file.to_string_lossy().to_string());
    }

    let wide: Vec<u16> = OsStr::new(target_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut shfi: SHFILEINFOW = std::mem::zeroed();
        let res = SHGetFileInfoW(
            wide.as_ptr(),
            FILE_ATTRIBUTE_NORMAL,
            &mut shfi,
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );

        if res == 0 || shfi.hIcon.is_null() {
            return None;
        }

        let mut icon_info: ICONINFO = std::mem::zeroed();
        if GetIconInfo(shfi.hIcon, &mut icon_info) == 0 {
            DestroyIcon(shfi.hIcon);
            return None;
        }

        let hdc = CreateCompatibleDC(std::ptr::null_mut());
        let width: i32 = 32;
        let height: i32 = 32;
        let mut bmi: BITMAPINFO = std::mem::zeroed();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = width;
        bmi.bmiHeader.biHeight = -height;
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB;

        let mut pixels = vec![0u8; (width * height * 4) as usize];
        let copied = GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height as u32,
            pixels.as_mut_ptr() as *mut _,
            &mut bmi,
            DIB_RGB_COLORS,
        );

        if !icon_info.hbmColor.is_null() {
            DeleteObject(icon_info.hbmColor);
        }
        if !icon_info.hbmMask.is_null() {
            DeleteObject(icon_info.hbmMask);
        }
        DeleteDC(hdc);
        DestroyIcon(shfi.hIcon);

        if copied == 0 {
            return None;
        }

        // Convert BGRA to RGBA
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }

        if let Some(img) = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, pixels)
        {
            if img.save(&out_file).is_ok() {
                return Some(out_file.to_string_lossy().to_string());
            }
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn extract_windows_icon(_target_path: &str, _icon_dir: &Path) -> Option<String> {
    None
}

fn ensure_icon_dir(app: &AppHandle) -> PathBuf {
    let base = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")));
    let dir = base.join("icons");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[tauri::command]
pub async fn scan_installed_apps(
    app: AppHandle,
    db_state: State<'_, Mutex<Database>>,
    telemetry: State<'_, TelemetryState>,
) -> Result<Vec<AppEntry>, String> {
    let icon_dir = ensure_icon_dir(&app);

    let apps = tokio::task::spawn_blocking(move || {
        let mut roots: Vec<PathBuf> = Vec::new();
        #[cfg(target_os = "windows")]
        {
            if let Ok(pd) = std::env::var("ProgramData") {
                roots.push(PathBuf::from(pd).join("Microsoft\\Windows\\Start Menu\\Programs"));
            }
            if let Ok(ad) = std::env::var("APPDATA") {
                roots.push(PathBuf::from(ad).join("Microsoft\\Windows\\Start Menu\\Programs"));
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            roots.push(PathBuf::from("/Applications"));
            roots.push(PathBuf::from("/usr/share/applications"));
        }

        let mut seen = HashSet::new();
        let mut results = Vec::new();

        for root in roots {
            if !root.exists() {
                continue;
            }
            for entry in WalkDir::new(&root)
                .max_depth(4)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                #[cfg(target_os = "windows")]
                let valid = ext == "lnk" || ext == "exe";
                #[cfg(not(target_os = "windows"))]
                let valid = ext == "desktop" || ext == "app";

                if !valid {
                    continue;
                }

                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();

                if name.is_empty() {
                    continue;
                }

                let lower_name = name.to_lowercase();
                if lower_name.contains("uninstall")
                    || lower_name.contains("unins")
                    || lower_name.contains("readme")
                    || lower_name.contains("help")
                    || lower_name.contains("license")
                    || lower_name.contains("website")
                {
                    continue;
                }

                if !seen.insert(lower_name) {
                    continue;
                }

                let full_path = path.to_string_lossy().to_string();
                let icon_path = extract_windows_icon(&full_path, &icon_dir);
                let category = categorize_app(&name, &full_path);

                results.push(AppEntry {
                    id: hash_id(&full_path),
                    name,
                    path: full_path,
                    icon_path,
                    category,
                    launch_count: 0,
                    last_launched: None,
                });
            }
        }
        results
    })
    .await
    .map_err(|e| e.to_string())?;

    let saved = {
        let mut db = db_state.lock().map_err(|e| e.to_string())?;
        if !apps.is_empty() {
            db.upsert_apps(&apps)?;
        }
        db.get_apps()?
    };

    if let Ok(mut count) = telemetry.indexed_items_count.lock() {
        *count = saved.len();
    }

    Ok(saved)
}

fn extract_vdf_field(content: &str, key: &str) -> Option<String> {
    let target = format!("\"{}\"", key.to_lowercase());
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().starts_with(&target) {
            let parts: Vec<&str> = trimmed.split('"').collect();
            if parts.len() >= 4 {
                return Some(parts[3].replace("\\\\", "\\"));
            }
        }
    }
    None
}

fn scan_steam_games() -> Vec<GameEntry> {
    let mut games = Vec::new();
    let steam_roots = [
        PathBuf::from("C:\\Program Files (x86)\\Steam"),
        PathBuf::from("C:\\Program Files\\Steam"),
    ];

    for steam_root in steam_roots {
        if !steam_root.exists() {
            continue;
        }
        let mut library_dirs = vec![steam_root.clone()];
        let vdf_path = steam_root.join("steamapps\\libraryfolders.vdf");
        if let Ok(vdf_content) = std::fs::read_to_string(&vdf_path) {
            for line in vdf_content.lines() {
                let trimmed = line.trim();
                if trimmed.to_lowercase().starts_with("\"path\"") {
                    let parts: Vec<&str> = trimmed.split('"').collect();
                    if parts.len() >= 4 {
                        let lib_path = PathBuf::from(parts[3].replace("\\\\", "\\"));
                        if lib_path.exists() && !library_dirs.contains(&lib_path) {
                            library_dirs.push(lib_path);
                        }
                    }
                }
            }
        }

        let cache_dir = steam_root.join("appcache\\librarycache");

        for lib in library_dirs {
            let steamapps = lib.join("steamapps");
            let Ok(entries) = std::fs::read_dir(&steamapps) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if !fname.starts_with("appmanifest_") || !fname.ends_with(".acf") {
                    continue;
                }
                let Ok(acf) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let Some(appid) = extract_vdf_field(&acf, "appid") else {
                    continue;
                };
                let Some(title) = extract_vdf_field(&acf, "name") else {
                    continue;
                };
                if title.to_lowercase().contains("redistributables")
                    || title.to_lowercase().contains("proton")
                {
                    continue;
                }

                let header_img = [
                    cache_dir.join(format!("{}_header.jpg", appid)),
                    cache_dir.join(&appid).join("header.jpg"),
                    cache_dir.join(format!("{}_library_600x900.jpg", appid)),
                ]
                .into_iter()
                .find(|p| p.exists())
                .map(|p| p.to_string_lossy().to_string());

                games.push(GameEntry {
                    id: format!("steam-{}", appid),
                    title,
                    platform: "Steam".to_string(),
                    executable_path: format!("steam://rungameid/{}", appid),
                    launch_args: vec![],
                    cover_image: header_img,
                    icon_path: None,
                    launch_count: 0,
                });
            }
        }
    }
    games
}

fn scan_epic_games() -> Vec<GameEntry> {
    let mut games = Vec::new();
    let manifest_dir = PathBuf::from("C:\\ProgramData\\Epic\\EpicGamesLauncher\\Data\\Manifests");
    let Ok(entries) = std::fs::read_dir(manifest_dir) else {
        return games;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("item") {
            continue;
        }
        let Ok(raw) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&raw) else {
            continue;
        };
        let Some(title) = json.get("DisplayName").and_then(|v| v.as_str()) else {
            continue;
        };
        let install_loc = json
            .get("InstallLocation")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let launch_exe = json
            .get("LaunchExecutable")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let app_name = json
            .get("AppName")
            .and_then(|v| v.as_str())
            .unwrap_or(title);

        let exe_path = if !install_loc.is_empty() && !launch_exe.is_empty() {
            PathBuf::from(install_loc)
                .join(launch_exe)
                .to_string_lossy()
                .to_string()
        } else {
            format!("com.epicgames.launcher://apps/{}?action=launch&silent=true", app_name)
        };

        games.push(GameEntry {
            id: format!("epic-{}", hash_id(app_name)),
            title: title.to_string(),
            platform: "Epic".to_string(),
            executable_path: exe_path,
            launch_args: vec![],
            cover_image: None,
            icon_path: None,
            launch_count: 0,
        });
    }
    games
}

#[tauri::command]
pub async fn scan_game_libraries(
    db_state: State<'_, Mutex<Database>>,
    telemetry: State<'_, TelemetryState>,
) -> Result<Vec<GameEntry>, String> {
    let detected = tokio::task::spawn_blocking(|| {
        let mut all = scan_steam_games();
        all.extend(scan_epic_games());
        all
    })
    .await
    .map_err(|e| e.to_string())?;

    let saved = {
        let mut db = db_state.lock().map_err(|e| e.to_string())?;
        if !detected.is_empty() {
            db.upsert_games(&detected)?;
        }
        db.get_games()?
    };

    if let Ok(mut count) = telemetry.indexed_items_count.lock() {
        if let Ok(db) = db_state.lock() {
            let apps_len = db.get_apps().map(|a| a.len()).unwrap_or(0);
            *count = apps_len + saved.len();
        }
    }

    Ok(saved)
}

#[tauri::command]
pub fn add_custom_game(
    title: String,
    executable_path: String,
    platform: String,
    db_state: State<'_, Mutex<Database>>,
) -> Result<Vec<GameEntry>, String> {
    let game = GameEntry {
        id: format!("local-{}", hash_id(&executable_path)),
        title,
        platform,
        executable_path,
        launch_args: vec![],
        cover_image: None,
        icon_path: None,
        launch_count: 0,
    };
    let mut db = db_state.lock().map_err(|e| e.to_string())?;
    db.upsert_games(&[game])?;
    db.get_games()
}

#[tauri::command]
pub fn get_folder_details(path: String) -> Result<FolderCard, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    let name = p
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&path)
        .to_string();
    let (file_count, total_size_bytes) = compute_folder_stats(&path);
    Ok(FolderCard {
        id: hash_id(&path),
        name,
        path,
        file_count,
        total_size_bytes,
        is_pinned: true,
        icon_name: Some("Folder".to_string()),
    })
}

#[tauri::command]
pub fn get_pinned_folders(db_state: State<'_, Mutex<Database>>) -> Result<Vec<FolderCard>, String> {
    let mut folders = {
        let db = db_state.lock().map_err(|e| e.to_string())?;
        db.get_pinned_folders()?
    };
    for f in &mut folders {
        if Path::new(&f.path).exists() {
            let (count, size) = compute_folder_stats(&f.path);
            f.file_count = count;
            f.total_size_bytes = size;
        }
    }
    Ok(folders)
}

#[tauri::command]
pub fn pin_folder(
    path: String,
    name: Option<String>,
    db_state: State<'_, Mutex<Database>>,
) -> Result<Vec<FolderCard>, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("Directory does not exist: {}", path));
    }
    let folder_name = name.unwrap_or_else(|| {
        p.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&path)
            .to_string()
    });
    let card = FolderCard {
        id: hash_id(&path),
        name: folder_name,
        path,
        file_count: 0,
        total_size_bytes: 0,
        is_pinned: true,
        icon_name: Some("FolderKanban".to_string()),
    };
    {
        let db = db_state.lock().map_err(|e| e.to_string())?;
        db.save_pinned_folder(&card)?;
    }
    get_pinned_folders(db_state)
}

#[tauri::command]
pub fn unpin_folder(
    id: String,
    db_state: State<'_, Mutex<Database>>,
) -> Result<Vec<FolderCard>, String> {
    {
        let db = db_state.lock().map_err(|e| e.to_string())?;
        db.remove_pinned_folder(&id)?;
    }
    get_pinned_folders(db_state)
}

#[tauri::command]
pub fn get_contexts(db_state: State<'_, Mutex<Database>>) -> Result<Vec<ContextProfile>, String> {
    let db = db_state.lock().map_err(|e| e.to_string())?;
    db.get_contexts()
}

#[tauri::command]
pub fn save_context(
    profile: ContextProfile,
    db_state: State<'_, Mutex<Database>>,
) -> Result<Vec<ContextProfile>, String> {
    let db = db_state.lock().map_err(|e| e.to_string())?;
    db.save_context(&profile)?;
    db.get_contexts()
}

#[tauri::command]
pub fn delete_context(
    id: String,
    db_state: State<'_, Mutex<Database>>,
) -> Result<Vec<ContextProfile>, String> {
    let db = db_state.lock().map_err(|e| e.to_string())?;
    db.delete_context(&id)?;
    db.get_contexts()
}

#[tauri::command]
pub fn search_indexed_items(
    query: String,
    db_state: State<'_, Mutex<Database>>,
) -> Result<Vec<(String, String)>, String> {
    let db = db_state.lock().map_err(|e| e.to_string())?;
    db.search_fts(&query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vdf_parser() {
        let sample = r#"
"AppState"
{
    "appid"     "1091500"
    "name"      "Cyberpunk 2077"
}
"#;
        assert_eq!(extract_vdf_field(sample, "appid").as_deref(), Some("1091500"));
        assert_eq!(extract_vdf_field(sample, "name").as_deref(), Some("Cyberpunk 2077"));
    }
}


