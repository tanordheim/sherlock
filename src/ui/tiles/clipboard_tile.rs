use gtk4::{self, prelude::*, Box, Builder, Image, Label, ListBoxRow};
use regex::Regex;

use super::util::insert_attrs;
use super::Tile;

impl Tile {
    pub fn clipboard_tile(
        index: i32,
        clipboard_content: &String
    ) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;

        let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/tile.ui");
        let holder: ListBoxRow = builder.object("holder").unwrap();
        let icon_obj: Image = builder.object("icon-name").unwrap();
        let title_obj: Label = builder.object("app-name").unwrap();
        let attr_holder: Box = builder.object("attrs-holder").unwrap();
        let launcher_type: Label = builder.object("launcher-type").unwrap();

        let mut name = "Clipboard Action";
        let mut method = "";
        let mut icon = "";

        // Check if clipboard content is a url:
        let pattern = r"^(https?:\/\/)?([\da-z\.-]+)\.([a-z]{2,6})([\/\w\.-]*)*\/?$";
        let re = Regex::new(pattern).unwrap();
        if re.is_match(clipboard_content) {
            name = "Clipboard Web-Search";
            method = "web_launcher";
            icon = "google";
        }

        if name.is_empty() {
            launcher_type.set_visible(false);
        }

        launcher_type.set_text(name);
        title_obj.set_text(clipboard_content);
        icon_obj.set_icon_name(Some(&icon));

        let attrs: Vec<String> = vec![
            format!("{} | {}", "method", method),
            format!("{} | {}", "keyword", clipboard_content),
            format!("{} | {}", "engine", "plain"),
        ];

        insert_attrs(&attr_holder, attrs);
        index_ref += 1;
        results.push(holder);

        return (index_ref + 1, results);
    }
}
