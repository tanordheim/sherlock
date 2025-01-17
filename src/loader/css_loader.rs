use gtk4::gdk::Display;
use gtk4::CssProvider;
use std::path::Path;

use super::util::{SherlockError, SherlockFlags};
use super::Loader;


impl Loader{
    pub fn load_css(sherlock_flags: &SherlockFlags) -> Result<(), SherlockError>{
        let provider = CssProvider::new();
        // Load the default css
        provider.load_from_resource("/dev/skxxtz/sherlock/main.css");

        // Load the custom css
        if Path::new(&sherlock_flags.style).exists(){
            provider.load_from_path(&sherlock_flags.style);
        } 
        
        let display = Display::default().ok_or_else(|| SherlockError {
                name: "Display Error".to_string(),
                message: format!("Could not connect do a display."),
                traceback: "No display available".to_string(),
        })?;

        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        Ok(())
    }
}

