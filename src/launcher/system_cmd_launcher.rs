use std::collections::HashMap;

use crate::loader::util::AppData;

#[derive(Clone, Debug)]
pub struct SystemCommand{
    pub commands: HashMap<String, AppData>,
}

