use std::collections::HashSet;

use crate::loader::util::AppData;

#[derive(Clone, Debug)]
pub struct SystemCommand {
    pub commands: HashSet<AppData>,
}
