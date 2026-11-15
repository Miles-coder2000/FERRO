use crate::db::Database;
use crate::telemetry::TelemetryState;
use std::process::Command;
use std::sync::Mutex;
use std::time::Instant;
use tauri::State;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[tauri::command]
pub fn launch_target(
    target_path: String,
    args: Vec<String>,
    as_admin: bool,
    db_state: State<'_, Mutex<Database>>,
    telemetry: State<'_, TelemetryState>,
) -> Result<u64, String> {
    let start = Instant::now();

    #[cfg(target_os = "windows")]
    {
        if target_path.starts_with("steam://")
            || target_path.starts_with("com.epicgames.launcher://")
            || target_path.starts_with("http://")
            || target_path.starts_with("https://")
        {
            open::that(&target_path).map_err(|e| format!("Failed to open URI: {}", e))?;
        } else if as_admin {
            let escaped = target_path.replace('\'', "''");
            let arg_list = if args.is_empty() {
                String::new()
            } else {
                let joined = args
                    .iter()
                    .map(|a| format!("'{}'", a.replace('\'', "''")))
                    .collect::<Vec<_>>()
                    .join(",");
                format!(" -ArgumentList {}", joined)
            };
            let ps_cmd = format!("Start-Process -FilePath '{}'{} -Verb RunAs", escaped, arg_list);
            Command::new("powershell")
                .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps_cmd])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .map_err(|e| format!("Admin launch error: {}", e))?;
        } else if target_path.to_lowercase().ends_with(".lnk") {
            open::that(&target_path).map_err(|e| format!("Shortcut launch error: {}", e))?;
        } else {
            let mut cmd = Command::new(&target_path);
            if !args.is_empty() {
                cmd.args(&args);
            }
            if let Some(parent) = std::path::Path::new(&target_path).parent() {
                if parent.exists() {
                    cmd.current_dir(parent);
                }
            }
            match cmd.spawn() {
                Ok(_) => {}
                Err(_) => {
                    open::that(&target_path)
                        .map_err(|e| format!("Failed to launch {}: {}", target_path, e))?;
                }
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = as_admin;
        let mut cmd = Command::new(&target_path);
        if !args.is_empty() {
            cmd.args(&args);
        }
        match cmd.spawn() {
            Ok(_) => {}
            Err(_) => {
                open::that(&target_path)
                    .map_err(|e| format!("Failed to launch {}: {}", target_path, e))?;
            }
        }
    }

    let elapsed_us = start.elapsed().as_micros() as u64;
    let elapsed_ms = (elapsed_us as f64) / 1000.0;

    if let Ok(db) = db_state.lock() {
        db.increment_launch_count(&target_path);
    }
    if let Ok(mut lat) = telemetry.last_launch_latency_ms.lock() {
        *lat = elapsed_ms;
    }

    Ok(elapsed_us)
}

#[tauri::command]
pub fn execute_shell_command(command_str: String) -> Result<String, String> {
    let trimmed = command_str.trim();
    if trimmed.is_empty() {
        return Err("Empty command".to_string());
    }

    #[cfg(target_os = "windows")]
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", trimmed])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh")
        .args(["-c", trimmed])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(if stdout.is_empty() {
            "Command executed successfully.".to_string()
        } else {
            stdout
        })
    } else {
        Err(if stderr.is_empty() {
            format!("Exit code: {:?}", output.status.code())
        } else {
            stderr
        })
    }
}

#[tauri::command]
pub fn open_folder_in_explorer(path: String) -> Result<(), String> {
    open::that(&path).map_err(|e| format!("Failed to open path {}: {}", path, e))
}

#[tauri::command]
pub fn reveal_file_location(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(format!("/select,{}", path))
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let target = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(path);
        open::that(target).map_err(|e| e.to_string())
    }
}
