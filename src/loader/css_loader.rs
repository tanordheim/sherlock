use gtk4::gdk::Display;
use gtk4::CssProvider;
use std::path::Path;

use super::Loader;
use crate::utils::errors::{SherlockError, SherlockErrorType};
use crate::{sherlock_error, CONFIG};

impl Loader {
    pub fn load_css() -> Result<Vec<SherlockError>, SherlockError> {
        let mut non_breaking: Vec<SherlockError> = Vec::new();
        let provider = CssProvider::new();

        let config = CONFIG.get().ok_or_else(|| sherlock_error!(
            SherlockErrorType::ConfigError(None),
            ""
        ))?;
        let display = Display::default().ok_or_else(|| sherlock_error!(
            SherlockErrorType::DisplayError,
            "No display available"
        ))?;

        // Load the base line css
        if config.appearance.use_base_css {
            provider.load_from_resource("/dev/skxxtz/sherlock/main.css");
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Load the user css
        if Path::new(&config.files.css).exists() {
            let usr_provider = CssProvider::new();
            usr_provider.load_from_path(&config.files.css);
            gtk4::style_context_add_provider_for_display(
                &display,
                &usr_provider,
                gtk4::STYLE_PROVIDER_PRIORITY_USER,
            );
        } else {
            non_breaking.push(sherlock_error!(
                SherlockErrorType::FileExistError(config.files.css.clone()),
                "Using default css"
            ));
        }

        drop(provider);
        Ok(non_breaking)
    }
}
