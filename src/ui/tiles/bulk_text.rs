use gtk4::{self, Box, Builder, Image, Label, ListBoxRow, TextView};

use super::Tile;
use super::util::insert_attrs;

impl Tile{
    pub fn bulk_text_tile_loader(name: &String, method: &String, icon: &String, keyword:&String)->Option<(ListBoxRow, Label, TextView, Box)>{
        if !keyword.is_empty(){
            let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/bulk_text_tile.ui");
            let holder:ListBoxRow = builder.object("holder").unwrap();
            let launcher_type:Label = builder.object("launcher-type").unwrap();
            let icon_obj:Image = builder.object("icon-name").unwrap();
            let content_title:Label = builder.object("content-title").unwrap();
            let content_body:TextView = builder.object("content-body").unwrap();
            let attr_holder:Box = builder.object("attrs-holder").unwrap();
            let loader_holder:Box = builder.object("loader-holder").unwrap();

            launcher_type.set_text(name);
            icon_obj.set_icon_name(Some(icon));
            content_title.set_text(keyword);

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", method),
                format!("{} | {}", "keyword", keyword),
            ];
            insert_attrs(&attr_holder, attrs);

            return Some((holder, content_title, content_body, loader_holder))
        }
        return None
    

    }
    pub fn bulk_text_tile(name: &String, method: &String, icon: &String, index:i32, keyword:&String)->(i32, Vec<ListBoxRow>){
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
            ];
            insert_attrs(&attr_holder, attrs);

            return (index + 1, vec![holder])
        }
        (index, vec![])

    }
}

