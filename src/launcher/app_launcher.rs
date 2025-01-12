use std::collections::HashMap;
use crate::loader::util::AppData;


#[derive(Clone, Debug)]
pub struct App{
    pub alias: Option<String>,
    pub method: String ,
    pub name: String,
    pub r#async: bool,
    pub priority: u32,
    pub apps: HashMap<String, AppData>,
}

