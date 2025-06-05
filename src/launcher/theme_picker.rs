use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Debug)]
pub struct ThemePicker {
    pub location: PathBuf,
    pub themes: HashMap<String, PathBuf>,
}
