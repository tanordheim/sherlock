use gtk4::gdk::Display;
use gtk4::CssProvider;

use super::Loader;


impl Loader{
    pub fn load_css() {
        let provider = CssProvider::new();
        provider.load_from_resource("/dev/skxxtz/sherlock/main.css");
        gtk4::style_context_add_provider_for_display(
            &Display::default().expect("Cound not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

