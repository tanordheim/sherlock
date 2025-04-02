use gtk4::gdk::Display;
use gtk4::CssProvider;
use std::path::Path;

use super::util::{SherlockError, SherlockErrorType};
use super::Loader;
use crate::CONFIG;

impl Loader {
    pub fn load_css() -> Result<Vec<SherlockError>, SherlockError> {
        let mut non_breaking: Vec<SherlockError> = Vec::new();
        let provider = CssProvider::new();

        let config = CONFIG.get().ok_or_else(|| SherlockError {
            error: SherlockErrorType::ConfigError(None),
            traceback: String::new(),
        })?;

        // Load the custom css
        if Path::new(&config.files.css).exists() {
            provider.load_from_path(&config.files.css);
        } else {
            // Load the default css
            provider.load_from_resource("/dev/skxxtz/sherlock/main.css");
            non_breaking.push(SherlockError {
                error: SherlockErrorType::FileExistError(config.files.css.clone()),
                traceback: String::from("Using default css"),
            });
        }

        let display = Display::default().ok_or_else(|| SherlockError {
            error: SherlockErrorType::DisplayError,
            traceback: "No display available".to_string(),
        })?;

        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        drop(provider);
        Ok(non_breaking)
    }
}
