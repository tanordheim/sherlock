use gtk4::{gdk::Display, IconTheme};
use super::Loader;

impl Loader {
    pub fn load_icon_theme(icons: &Vec<String>){
        let icon_theme = IconTheme::for_display(Display::default().as_ref().unwrap());
        for icon in icons.iter(){
            icon_theme.add_search_path(icon);
        }
        icon_theme.theme_name();
    }
}


