use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

#[derive(Deserialize, Debug)]
pub struct RawLauncher {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub tag_start: Option<String>,
    pub tag_end: Option<String>,
    pub display_name: Option<String>,
    pub on_return: Option<String>,
    pub next_content: Option<String>,
    pub r#type: String,
    pub priority: f32,

    #[serde(default = "default_true")]
    pub shortcut: bool,
    #[serde(default = "default_true")]
    pub spawn_focus: bool,
    #[serde(default)]
    pub r#async: bool,
    #[serde(default)]
    pub home: bool,
    #[serde(default)]
    pub only_home: bool,
    #[serde(default)]
    pub args: serde_json::Value,
}
fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppData {
    #[serde(default)]
    pub name: String,
    pub icon: String,
    pub icon_class: Option<String>,
    pub exec: String,
    pub search_string: String,
    pub tag_start: Option<String>,
    pub tag_end: Option<String>,
    pub desktop_file: Option<PathBuf>,
    #[serde(default)]
    pub priority: f32,
}
impl Eq for AppData {}
impl Hash for AppData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Make more efficient and handle error using f32
        self.exec.hash(state);
        self.desktop_file.hash(state);
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct SherlockAlias {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub exec: Option<String>,
    pub keywords: Option<String>,
}
