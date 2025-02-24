use gtk4::ListBoxRow;

use super::util::{get_builder, insert_attrs};
use super::Tile;
use crate::launcher::web_launcher::Web;
use crate::launcher::Launcher;

impl Tile {
    pub fn web_tile(
        launcher: &Launcher,
        index: i32,
        keyword: &String,
        web: &Web,
    ) -> (i32, Vec<ListBoxRow>) {
        if !keyword.is_empty() {
            let builder = get_builder("/dev/skxxtz/sherlock/ui/tile.ui", index, true);

            builder.category.set_text(&launcher.name);
            builder.icon.set_icon_name(Some(&web.icon));
            
            let tile_name = if web.display_name.contains("{keyword}"){
                web.display_name.replace("{keyword}", keyword)
            }else {
                web.display_name.clone()
            };

            builder.title.set_text(&tile_name);
            builder.display_tag_start(&launcher.tag_start, keyword);
            builder.display_tag_end(&launcher.tag_end, keyword);

            let attrs: Vec<(&str, &str)> = vec![
                ("method", &launcher.method),
                ("engine", &web.engine),
                ("keyword", keyword),
                ("result", keyword),
            ];
            insert_attrs(&builder.attrs, attrs);

            return (index + 1, vec![builder.object]);
        }
        (index, vec![])
    }
}
