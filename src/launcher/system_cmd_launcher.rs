use std::collections::HashMap;

use super::app_launcher::AppData;

#[derive(Clone, Debug)]
pub struct SystemCommand{
    pub alias: Option<String>,
    pub method: String,
    pub uuid: String,
    pub name: String,
    pub commands: HashMap<String, AppData>,
    pub r#async: bool,
    pub priority: u32,
}

