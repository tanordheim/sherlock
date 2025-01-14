use std::collections::HashMap;
use crate::loader::util::AppData;

#[derive(Clone, Debug)]
pub struct App{
    pub apps: HashMap<String, AppData>,
}

