#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
<<<<<<< HEAD
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
=======
    ferro_lib::run();
}
>>>>>>> 743870a (feat: initial FERRO launcher project structure)
