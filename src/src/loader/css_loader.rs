use gtk4::gdk::Display;
use gtk4::CssProvider;
use std::path::Path;

use super::util::SherlockFlags;
use super::Loader;


impl Loader{
    pub fn load_css(sherlock_flags: &SherlockFlags) {
        let provider = CssProvider::new();
        // Load the default css
        provider.load_from_resource("/dev/skxxtz/sherlock/main.css");

        // Load the custom css
        if Path::new(&sherlock_flags.style).exists(){
            provider.load_from_path(&sherlock_flags.style);
        } 
        

        gtk4::style_context_add_provider_for_display(
            &Display::default().expect("Cound not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

