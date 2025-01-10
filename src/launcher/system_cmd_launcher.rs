use std::collections::HashMap;

use super::app_launcher::AppData;

#[derive(Clone, Debug)]
pub struct SystemCommand{
    pub alias: Option<String>,
    pub method: String,
    pub name: String,
    pub commands: HashMap<String, AppData>,
    pub priority: u32,
}

