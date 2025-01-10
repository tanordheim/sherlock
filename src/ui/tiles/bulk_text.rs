use gtk4::{self, Builder, ListBoxRow, Image, Label, Box, glib};

use super::Tile;
use super::util::insert_attrs;

impl Tile{
    pub async fn bulk_text_tile_async(name: &String, method: &String, icon: &String, path: &String, key: &String, index:i32, keyword:&String)->(i32, Vec<ListBoxRow>){
        if !keyword.is_empty(){
            glib::timeout_future_seconds(2).await;
            let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/bulk_text_tile.ui");
            let holder:ListBoxRow = builder.object("holder").unwrap();
            let launcher_type:Label = builder.object("launcher-type").unwrap();
            let icon_obj:Image = builder.object("icon-name").unwrap();
            let title_obj:Label = builder.object("content-title").unwrap();
            let attr_holder:Box = builder.object("attrs-holder").unwrap();

            launcher_type.set_text(name);
            icon_obj.set_icon_name(Some(icon));
            title_obj.set_text(keyword);

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", method),
                format!("{} | {}", "keyword", keyword),
                format!("{} | {}", "path", path),
                format!("{} | {}", "key", key),
            ];
            insert_attrs(&attr_holder, attrs);

            return (index + 1, vec![holder])
        }
        (index, vec![])

    }
    pub fn bulk_text_tile(name: &String, method: &String, icon: &String, path: &String, key: &String, index:i32, keyword:&String)->(i32, Vec<ListBoxRow>){
        if !keyword.is_empty(){
            let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/bulk_text_tile.ui");
            let holder:ListBoxRow = builder.object("holder").unwrap();
            let launcher_type:Label = builder.object("launcher-type").unwrap();
            let icon_obj:Image = builder.object("icon-name").unwrap();
            let title_obj:Label = builder.object("content-title").unwrap();
            let attr_holder:Box = builder.object("attrs-holder").unwrap();

            launcher_type.set_text(name);
            icon_obj.set_icon_name(Some(icon));
            title_obj.set_text(keyword);

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", method),
                format!("{} | {}", "keyword", keyword),
                format!("{} | {}", "path", path),
                format!("{} | {}", "key", key),
            ];
            insert_attrs(&attr_holder, attrs);

            return (index + 1, vec![holder])
        }
        (index, vec![])

    }
}

