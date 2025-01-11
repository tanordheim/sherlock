use gio::prelude::*;
use gtk4::{prelude::*, Application};
use once_cell::sync::Lazy;
use std::env;

mod launcher;
mod ui;
mod actions;
mod loader;

use loader::{Loader, util::Config};


static CONFIG: Lazy<Config> = Lazy::new(|| {
    Loader::load_config()
});



#[tokio::main]
async fn main() {
    Loader::load_resources();

    env::set_var("GSK_RENDERER", &CONFIG.appearance.gsk_renderer);
    let application = Application::new(Some("dev.skxxtz.sherlock"), Default::default());

    application.connect_activate(|app| {
        let launchers = Loader::load_launchers();
        Loader::load_icon_theme(&CONFIG.appearance.icon_paths);
        Loader::load_css();

        // Move the async block to GTK's main thread
        let app_clone = app.clone();
        let mut window = ui::window::window(&app_clone);
        window = ui::search::search(window, launchers);
        window.show();
    });

    application.run();
}

