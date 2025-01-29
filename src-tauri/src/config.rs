use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub project_root: String,
    pub autosort_tags: Vec<AutosortTag>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutosortTag {
    pub extensions: Vec<String>,
    pub folder: String,
}
