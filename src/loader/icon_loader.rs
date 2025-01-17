use gtk4::{gdk::Display, IconTheme};
use std::env;
use super::Loader;
use super::util::SherlockError;

impl Loader {
    pub fn load_icon_theme(icon_paths: &Vec<String>)-> Vec<SherlockError>{
        let mut non_breaking: Vec<SherlockError> = Vec::new();
        let icon_theme = IconTheme::for_display(Display::default().as_ref().unwrap());
        let home_dir = env::var("HOME")
            .map_err(|e| {
                non_breaking.push(
                    SherlockError {
                        name:format!("Env Var not Found Error"),
                        message: format!("Failed to unpack home directory for user."),
                        traceback: e.to_string(),
                    }
                );
            })
        .ok();

        if let Some(h) = home_dir {
            icon_paths
                .iter()
                .map(|path| {
                    path.replace("~", &h)
                })
            .for_each(|path| icon_theme.add_search_path(path));
        }
        non_breaking
    }
}


