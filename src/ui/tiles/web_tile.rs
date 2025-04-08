use gio::glib::object::ObjectExt;

use super::util::TileBuilder;
use super::Tile;
use crate::actions::execute_from_attrs;
use crate::launcher::web_launcher::Web;
use crate::launcher::{Launcher, ResultItem};

impl Tile {
    pub fn web_tile(launcher: &Launcher, keyword: &str, web: &Web) -> Vec<ResultItem> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
        builder.object.set_spawn_focus(launcher.spawn_focus);
        builder.object.set_shortcut(launcher.shortcut);

        builder.category.set_text(&launcher.name);
        builder.icon.set_icon_name(Some(&web.icon));

        let tile_name = if web.display_name.contains("{keyword}") {
            web.display_name.replace("{keyword}", keyword)
        } else {
            web.display_name.clone()
        };

        builder.title.set_text(&tile_name);
        builder.display_tag_start(&launcher.tag_start, keyword);
        builder.display_tag_end(&launcher.tag_end, keyword);

        let mut attrs: Vec<Option<(&str, &str)>> = vec![Some(("engine", &web.engine))];
        if let Some(next) = launcher.next_content.as_deref() {
            attrs.push(Some(("next_content", next)));
        }

        let attrs = builder.add_default_attrs(
            Some(&launcher.method),
            Some(keyword),
            Some(keyword),
            None,
            attrs,
        );
        builder.object.connect(
            "row-should-activate",
            false,
            move |_row| {
                execute_from_attrs(&attrs);
                None
            },
        );

        let shortcut_holder = match launcher.shortcut {
            true => builder.shortcut_holder,
            _ => None,
        };
        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
            shortcut_holder,
        };

        return vec![res];
    }
}
