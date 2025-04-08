use std::collections::HashMap;
use crate::loader::util::AppData;

#[derive(Clone, Debug)]
pub struct CategoryLauncher {
    pub categories: HashMap<String, AppData>,
}
