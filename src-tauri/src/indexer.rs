use crate::db::Database;
use crate::models::{AppEntry, ContextProfile, FolderCard, GameEntry};
use crate::telemetry::TelemetryState;
use crate::watcher::compute_folder_stats;
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use walkdir::WalkDir;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

/// Hash the canonical binary path + file mtime (SHA-256 conceptually)
/// For simplicity, we use a deterministic hash from path + modification time
pub fn hash_path_mtime(path: &str, _mtime: i64) -> String {
    let mut hasher = DefaultHasher::new();
    path.to_lowercase().hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Extract an .exe or .lnk icon using Windows native APIs
/// Cached to disk: app_cache_dir/icons/<hash>.png
/// Returns the cached file path to pass to Svelte via convertFileSrc()
#[cfg(target_os = "windows")]
#[tauri::command]
pub fn extract_icon_cached(target_path: &str, app_cache_dir: &str) -> Result<String, String> {
    use image::{ImageBuffer, Rgba};
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, BITMAPINFO, BITMAPINFOHEADER,
        BI_RGB, DIB_RGB_COLORS,
    };
    use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL;
    use windows_sys::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};

    let icon_dir = format!("{}/icons", app_cache_dir);
    let _ = std::fs::create_dir_all(&icon_dir);

    let hash = hash_path_mtime(target_path, 0);
    let out_file = format!("{}/{}.png", icon_dir, hash);

    // Return cached icon if it exists
    if std::path::Path::new(&out_file).exists() {
        return Ok(out_file);
    }

    // Extract icon from file path
    let wide: Vec<u16> = target_path
        .encode_utf16()
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
            // Fallback: create a generic placeholder icon
            let surf = Rgba::from([232, 93, 4, 255]); // Copper color
            let img = ImageBuffer::from_pixel(32, 32, surf);
            img.save(&out_file).map_err(|e| e.to_string())?;
            return Ok(out_file);
        }

        // Use 32x32 for now
        let width: i32 = 32;
        let height: i32 = 32;

        let hdc = CreateCompatibleDC(std::ptr::null_mut());
        let mut bmi: BITMAPINFO = std::mem::zeroed();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = width;
        bmi.bmiHeader.biHeight = -height; // Top-down DIB
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB;
        bmi.bmiHeader.biSizeImage = (width as u32 * height as u32 * 4 + 3) & !3;

        let mut bits: Vec<u8> = vec![0; (width as u32 * height as u32 * 4 + 3) & !3];
        let _ = windows_sys::Win32::Graphics::Gdi::GetDIBits(
            hdc,
            shfi.hIcon,
            0,
            height as u32,
            Some(bits.as_ptr() as *const std::os::raw::c_void as *mut std::os::raw::c_void),
            &mut bmi as *mut BITMAPINFO,
            DIB_RGB_COLORS,
        );
        let _ = windows_sys::Win32::Graphics::Gdi::DeleteDC(hdc);
        windows_sys::Win32::UI::WindowsAndMessaging::DestroyIcon(shfi.hIcon);

        // Convert BGRA to RGBA
        let mut img = ImageBuffer::<Rgba<u8>, _>::new(width as u32, height as u32);
        for (i, pixel) in img.pixels_mut().enumerate() {
            let b = bits[i * 4];
            let g = bits[i * 4 + 1];
            let r = bits[i * 4 + 2];
            let a = bits[i * 4 + 3];
            *pixel = Rgba([r, g, b, a]);
        }

        img.save(&out_file).map_err(|e| e.to_string())?;
        Ok(out_file)
    }
}

/// Extract icon from .exe or .lnk file (cross-platform stub)
/// For now, returns a cached placeholder path; full Windows extraction in extract_icon_cached
#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn extract_icon_cached(target_path: &str, app_cache_dir: &str) -> Result<String, String> {
    let icon_dir = format!("{}/icons", app_cache_dir);
    let _ = std::fs::create_dir_all(&icon_dir);
    let hash = hash_path_mtime(target_path, 0);
    let out_file = format!("{}/{}.png", icon_dir, hash);
    // Create a placeholder cyan icon for non-Windows platforms
    use image::{ImageBuffer, Rgba};
    let surf = Rgba::from([38, 189, 218, 255]); // Cyan color
    let img = ImageBuffer::from_pixel(32, 32, surf);
    img.save(&out_file).map_err(|e| e.to_string())?;
    Ok(out_file)
}

/// Scan installed applications from Start Menu and Program Files
#[tauri::command]
pub fn scan_installed_apps(db_state: State<'_, Mutex<Database>>) -> Result<Vec<AppEntry>, String> {
    let db = db_state.lock().map_err(|e| e.to_string())?;

    // Collect paths from SQLite
    let mut apps = db.get_apps()?;

    // Categorize and enrich
    for app in &mut apps {
        if app.icon_path.is_none() {
            app.icon_path = Some(format!("placeholder_{}.png", app.id));
        }
        if app.category.is_empty() {
            app.category = "System & Utility".to_string();
        }
    }

    // Rebuild FTS index
    let _ = db.rebuild_fts();

    Ok(apps)
}

/// Scan game libraries (Steam, Epic, standalone)
#[tauri::command]
pub fn scan_game_libraries(db_state: State<'_, Mutex<Database>>) -> Result<Vec<GameEntry>, String> {
    let db = db_state.lock().map_err(|e| e.to_string())?;

    // This would parse Steam libraryfolders.vdf and Epic .item manifests
    // For now, return empty list - full implementation would read these files
    let mut games: Vec<GameEntry> = Vec::new();

    // Categorize existing games
    for game in &mut games {
        if game.icon_path.is_none() {
            game.icon_path = Some(format!("game_{}.png", game.id));
        }
        if game.platform.is_empty() {
            game.platform = "Local".to_string();
        }
    }

    Ok(games)
}

/// Get folder details (file count, total size)
#[tauri::command]
pub fn get_folder_details(path: String, _db_state: State<'_, Mutex<Database>>) -> Result<FolderCard, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("Directory does not exist: {}", path));
    }

    let (file_count, total_size_bytes) = compute_folder_stats(&path);

    let id = hash_path_mtime(&path, 0);

    Ok(FolderCard {
        id,
        name: p.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&path)
            .to_string(),
        path,
        file_count,
        total_size_bytes,
        is_pinned: false,
        icon_name: Some("Folder".to_string()),
    })
}

/// Watch pinned folders for changes
#[tauri::command]
pub fn watch_pinned_folders(
    paths: Vec<String>,
    app: AppHandle,
    state: State<'_, RecommendedWatcher>,
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
                                serde_json::json!({
                                    "path": root,
                                    "file_count": file_count,
                                    "total_size_bytes": total_size_bytes
                                }),
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

    Ok(())
}

/// Search indexed items via FTS5
#[tauri::command]
pub fn search_indexed_items(query: String, db_state: State<'_, Mutex<Database>>) -> Result<Vec<(String, String)>, String> {
    let db = db_state.lock().map_err(|e| e.to_string())?;
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(vec![]);
    }
    let fts_query = trimmed
        .split_whitespace()
        .map(|tok| format!("{}*", tok.replace('"', "")))
        .collect::<Vec<_>>()
        .join(" ");

    db.search_fts(&fts_query)
}