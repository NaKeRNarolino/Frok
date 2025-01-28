use crate::dynamic_color::get_windows_accent_color;

mod dynamic_color;
mod path;
mod config;
mod autosort;

#[cfg_attr(mobile, tauri::mobile_enpub(crate))]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_windows_accent_color, path::select_folder, autosort::autosort_folders_and_gen_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
