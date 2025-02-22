use gtk4::{self, prelude::*, Box, Builder, Image, Label, ListBoxRow};
use std::collections::HashMap;

use crate::loader::util::{AppData, Config};

use super::util::{ensure_icon_name, insert_attrs};
use super::Tile;

impl Tile {
    pub fn app_tile(
        index: i32,
        commands: HashMap<String, AppData>,
        name: &String,
        method: &String,
        keyword: &String,
        app_config: &Config,
    ) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;

        for (key, value) in commands.into_iter() {
            if value
                .search_string
                .to_lowercase()
                .contains(&keyword.to_lowercase())
            {
                //TODO Remoce the unwrap and make proper error handling
                let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/tile.ui");
                let holder: ListBoxRow = builder.object("holder").unwrap();
                let icon_obj: Image = builder.object("icon-name").unwrap();
                let title_obj: Label = builder.object("app-name").unwrap();
                let attr_holder: Box = builder.object("attrs-holder").unwrap();

                let tag_start: Label = builder.object("app-name-tag-start").unwrap();
                let tag_end: Label = builder.object("app-name-tag-end").unwrap();

                if index_ref < 5 {
                    let shortcut_holder: Box = builder.object("shortcut-holder").unwrap();
                    let shortcut: Label = builder.object("shortcut").unwrap();
                    shortcut_holder.set_visible(true);
                    shortcut.set_text(format!("ctrl + {}", index_ref + 1).as_str());
                }

                let icon = if app_config.appearance.recolor_icons {
                    ensure_icon_name(value.icon)
                } else {
                    value.icon
                };
            
                let tile_name = if key.contains("{keyword}"){
                    tag_start.set_text(keyword);
                    tag_start.set_visible(true);
                    &key.replace("{keyword}", "")
                } else { &key };


                let launcher_type: Label = builder.object("launcher-type").unwrap();
                if name.is_empty() {
                    launcher_type.set_visible(false);
                }
                launcher_type.set_text(name);
                icon_obj.set_icon_name(Some(&icon));
                title_obj.set_markup(tile_name);

                let attrs: Vec<String> = vec![
                    format!("{} | {}", "method", method),
                    format!("{} | {}", "app_name", &key),
                    format!("{} | {}", "exec", &value.exec),
                    format!("{} | {}", "keyword", keyword),
                ];

                insert_attrs(&attr_holder, attrs);
                index_ref += 1;
                results.push(holder);
            }
        }
        return (index_ref, results);
    }
}
