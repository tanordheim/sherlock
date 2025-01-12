use std::collections::HashMap;
use gtk4::{self, prelude::*, Builder, ListBoxRow, Image, Label, Box};

use crate::CONFIG;
use crate::launcher::app_launcher::AppData;

use super::Tile;
use super::util::{ensure_icon_name, insert_attrs};


impl Tile{
    pub fn app_tile(index:i32, commands:HashMap<String, AppData>, name:&String, method:&String, keyword:&String)->(i32, Vec<ListBoxRow>){
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;


        for (key, value) in commands.into_iter() {
            if value.search_string.to_lowercase().contains(&keyword.to_lowercase()){
                let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/tile.ui");
                let holder:ListBoxRow = builder.object("holder").unwrap();
                let icon_obj:Image = builder.object("icon-name").unwrap();
                let title_obj:Label = builder.object("app-name").unwrap();
                let attr_holder:Box = builder.object("attrs-holder").unwrap();

                if index_ref < 5 {
                    let shortcut_holder:Box = builder.object("shortcut-holder").unwrap();
                    let shortcut:Label = builder.object("shortcut").unwrap();
                    shortcut_holder.set_visible(true);
                    shortcut.set_text(format!("ctrl + {}", index_ref + 1).as_str());
                }
                
                let icon = if CONFIG.appearance.recolor_icons {
                    ensure_icon_name(value.icon)
                } else {
                    value.icon
                };

                let tile_name = key.replace("{keyword}", keyword);

                let launcher_type:Label = builder.object("launcher-type").unwrap();
                launcher_type.set_text(name);
                icon_obj.set_icon_name(Some(&icon));
                title_obj.set_text(&tile_name);

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
        return (index_ref, results)
    }
}
