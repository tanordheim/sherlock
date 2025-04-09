use crate::loader::util::AppData;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CategoryLauncher {
    pub categories: HashMap<String, AppData>,
}
