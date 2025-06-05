use std::collections::HashSet;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::create_dir_all;

use crate::loader::util::AppData;
use crate::sherlock_error;
use crate::utils::errors::{SherlockError, SherlockErrorType};
use crate::utils::files::home_dir;

use super::LauncherType;

#[derive(Clone, Debug)]
pub struct ThemePicker {
    pub location: PathBuf,
    pub themes: HashSet<AppData>,
}
impl ThemePicker {
    pub fn new<T: AsRef<Path>>(loc: T, prio: f32) -> LauncherType {
        let absolute = loc.as_ref();
        if !absolute.is_dir() {
            return LauncherType::Empty;
        }
        let themes: HashSet<AppData> = absolute
            .read_dir()
            .ok()
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| path.is_file() || path.is_symlink())
                    .filter_map(|path| {
                        if path.extension()?.to_str()? == "css" {
                            let name = path.file_name()?.to_str()?.to_string();
                            let mut data = AppData::new();
                            data.name = name.clone();
                            data.exec = path.to_str().map(|s| s.to_string());
                            data.search_string = name;
                            data.priority = prio;
                            data.icon = Some(String::from("sherlock-devtools"));
                            Some(data)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        if themes.is_empty() {
            return LauncherType::Empty;
        }

        LauncherType::Theme(ThemePicker {
            location: absolute.to_path_buf(),
            themes,
        })
    }
    pub fn select_theme<T: AsRef<str>>(theme: T) -> Result<(), SherlockError> {
        let absolute = Self::get_cached()?;
        let theme = theme.as_ref();

        write(&absolute, theme).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::FileWriteError(absolute.clone()),
                e.to_string()
            )
        })?;
        Ok(())
    }

    pub fn get_cached() -> Result<PathBuf, SherlockError> {
        let home = home_dir()?;
        let absolute = home.join(".sherlock/theme.txt");
        if let Some(parents) = absolute.parent() {
            let _ = create_dir_all(parents);
        }
        Ok(absolute)
    }
}
