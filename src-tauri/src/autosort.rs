use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use fs_extra::dir::{CopyOptions, DirOptions};
use crate::config::{AutosortTag, ProjectConfig};

#[tauri::command]
pub fn autosort_folders_and_gen_config(
    app: tauri::AppHandle,
    root: String,
    tags: Vec<String>
) -> String{
    println!("Autosorting started at {}", &root);

    dbg!(&tags);

    let tags = tags.iter().map(|x| {
        serde_json::from_str::<AutosortTag>(x).unwrap()
    }).collect::<Vec<AutosortTag>>();

    dbg!(&tags);

    let config = ProjectConfig {
        project_root: root.clone(),
        autosort_tags: tags.clone()
    };

    let mut extension_folder_associations: HashMap<String, String> = HashMap::new();

    for tag in tags {
        fs::create_dir_all(format!("{}/{}/", &root, &tag.folder)).expect("Cannot create the folder");
        for ext in tag.extensions {
            extension_folder_associations.insert(ext.clone(), tag.folder.clone());
        }
    }

    let reader = fs_extra::dir::get_dir_content(&root).expect("Unable to read the directory contents");

    fs::write(format!("{}/project.frok.yaml", &root), serde_yml::to_string(&config).unwrap()).expect("Unable to write the config");

    for path in reader.files {
        println!("{}", &path);
        let extension = PathBuf::from(&path).extension().unwrap().to_str().unwrap().to_string();
        let file_name = PathBuf::from(&path).file_name().unwrap().to_str().unwrap().to_string();

        let folder = extension_folder_associations.get(&extension);

        println!("{}", &extension);

        if let Some(folder) = folder {
            let res = format!("{}/{}/{}", &root, &folder, &file_name);

            // fs_extra::file::move_file(&path, &res, &fs_extra::file::CopyOptions::default()).unwrap();

            fs::copy(&path, &res).expect("Unable to copy the file");

            fs::remove_file(&path).expect("Unable to delete the file");
        }
    }

    String::from("OK")
}