use crate::config::ProjectConfig;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn select_folder(app_handle: tauri::AppHandle) -> String {
    let mut folder = String::from("");
    if let Some(f) = app_handle.dialog().file().blocking_pick_folder() {
        folder = f.to_string()
    }
    folder
}

#[tauri::command]
pub fn select_config(app_handle: tauri::AppHandle) -> String {
    let mut file = String::from("");
    if let Some(f) = app_handle
        .dialog()
        .file()
        .add_filter("Frok Projects", &["yml", "yaml"])
        .blocking_pick_file()
    {
        file = f.to_string()
    }
    file
}

#[tauri::command]
pub fn get_config(app_handle: tauri::AppHandle, path: String) -> ProjectConfig {
    dbg!(&path);
    serde_yml::from_str::<ProjectConfig>(&fs::read_to_string(&path).unwrap()).unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileData {
    name: String,
    path: String,
    depth: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FolderData {
    name: String,
    path: String,
    folders: Vec<FolderData>,
    files: Vec<FileData>,
}

#[tauri::command]
pub fn get_folder_data(path: String) -> FolderData {
    _get_folder_data(path, 0)
}

fn _get_folder_data(path: String, depth: u128) -> FolderData {
    let pb = PathBuf::from(&path);

    let mut folder_data = FolderData {
        name: pb.file_name().unwrap().to_str().unwrap().to_string(),
        path: path.clone(),
        folders: vec![],
        files: vec![],
    };

    dbg!(&path);

    let read = fs::read_dir(pb).unwrap();

    for dir_entry in read.map(|x| x.unwrap()) {
        if dir_entry.path().is_dir() {
            folder_data.folders.push(
                FolderData {
                    name: dir_entry.path().file_name().unwrap().to_str().unwrap().to_string(),
                    path: dir_entry.path().to_str().unwrap().to_string(),
                    folders: vec![],
                    files: vec![],
                }
            );
        } else {
            let file = FileData {
                path: dir_entry.path().to_str().unwrap().to_string(),
                name: dir_entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                depth,
            };

            folder_data.files.push(file);
        }
    }

    folder_data
}
