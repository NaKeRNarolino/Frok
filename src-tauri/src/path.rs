use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn select_folder(app_handle: tauri::AppHandle) -> String {
    let mut folder =
        String::from("");
    if let Some(f) = app_handle.dialog().file().blocking_pick_folder() {
        folder = f.to_string()
    }
    folder
}