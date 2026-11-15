use std::fs;
use std::path::PathBuf;

/// Returns the application's log directory, creating it if needed.
///
/// Priority order:
/// 1. `$env:FERRO_LOG_DIR` if set (explicit user override)
/// 2. `D:/FERRO/logs/runtime` if it exists or can be created (developer machine)
/// 3. Platform-appropriate app data directory as fallback
///
/// This function never fails - it will create directories as needed
/// and fall back through progressively more permissive locations.

pub fn log_dir() -> PathBuf {
    // Step 1: Check FERRO_LOG_DIR environment variable first
    if let Some(custom) = std::env::var_os("FERRO_LOG_DIR") {
        let path = PathBuf::from(&custom);
        let _ = fs::create_dir_all(&path);
        return path;
    }

    // Step 2: Check if D:/FERRO/logs/runtime exists (developer machine)
    let dev_path = PathBuf::from("D:/FERRO/logs/runtime");
    if dev_path.exists() || fs::create_dir_all(&dev_path).is_ok() {
        return dev_path;
    }

    // Step 3: Fallback to platform-appropriate directory
    #[cfg(target_os = "windows")]
    {
        // Windows: %LOCALAPPDATA%\com.ferro.launcher\logs
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let path = PathBuf::from(&local_app_data)
                .join("com.ferro.launcher")
                .join("logs");
            let _ = fs::create_dir_all(&path);
            return path;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Unix: $XDG_DATA_HOME/logs or home/.local/share/ferro/logs
        if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
            let path = PathBuf::from(&xdg_data_home).join("ferro").join("logs");
            let _ = fs::create_dir_all(&path);
            return path;
        }
    }

    // Last resort: system temp directory
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("ferro").join("logs");
    let _ = fs::create_dir_all(&path);
    path
}
