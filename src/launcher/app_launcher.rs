use crate::loader::util::AppData;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct App {
    pub apps: HashSet<AppData>,
}
