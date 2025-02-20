use crate::loader::util::AppData;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct App {
    pub apps: HashMap<String, AppData>,
}
