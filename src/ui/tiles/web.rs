use gtk4::{self, prelude::*, Builder, ListBoxRow, Image, Label, Box};

use super::Tile;
use super::util::insert_attrs;

impl Tile{
    pub fn web_tile(name:&String, method: &String, icon: &String, engine: &String ,index:i32, keyword:&String)->(i32, Vec<ListBoxRow>){
        if !keyword.is_empty(){
            let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/tile.ui");
            let holder:ListBoxRow = builder.object("holder").unwrap();
            let icon_obj:Image = builder.object("icon-name").unwrap();
            let title_obj:Label = builder.object("app-name").unwrap();
            let launcher_type:Label = builder.object("launcher-type").unwrap();
            let attr_holder:Box = builder.object("attrs-holder").unwrap();

            if index < 5 {
                let shortcut_holder:Box = builder.object("shortcut-holder").unwrap();
                let shortcut:Label = builder.object("shortcut").unwrap();
                shortcut_holder.set_visible(true);
                shortcut.set_text(format!("ctrl + {}", index + 1).as_str());
            }

            launcher_type.set_text(name);
            icon_obj.set_icon_name(Some(icon));
            title_obj.set_text(keyword);

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", method),
                format!("{} | {}", "engine", engine),
                format!("{} | {}", "keyword", keyword),
            ];
            insert_attrs(&attr_holder, attrs);

            return (index + 1, vec![holder])
        }
        (index, vec![])
    }
}
