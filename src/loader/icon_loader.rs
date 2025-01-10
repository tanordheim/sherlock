use gtk4::IconTheme;
use super::Loader;

impl Loader {
    pub fn load_icon_theme(icons: &Vec<String>){
        let icon_theme = IconTheme::default();
        for icon in icons.iter(){
            icon_theme.add_search_path(icon);
        }
        println!("Search paths: {:?}", icon_theme.search_path())
    }
}
