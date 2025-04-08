use std::io::Cursor;

use crate::actions::execute_from_attrs;
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::loader::pipe_loader::PipeData;
use gdk_pixbuf::Pixbuf;
use gio::glib::object::ObjectExt;
use gtk4::prelude::BoxExt;
use gtk4::prelude::WidgetExt;
use gtk4::Image;

use super::util::SherlockSearch;
use super::util::TileBuilder;
use super::Tile;

impl Tile {
    pub fn pipe_data(lines: &Vec<PipeData>, method: &str, keyword: &str) -> Vec<SherlockRow> {
        let mut results: Vec<SherlockRow> = Default::default();

        for item in lines {
            if item.fuzzy_match(keyword) || item.binary.is_some() {
                let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
                builder.object.set_spawn_focus(true);

                if let Some(title) = &item.title {
                    builder.title.set_text(&title);
                }
                if let Some(desc) = &item.description {
                    builder.category.set_text(&desc);
                } else {
                    builder.category.set_visible(false);
                }
                if let Some(icon) = &item.icon {
                    builder.icon.set_icon_name(Some(&icon));
                } else {
                    builder.icon.set_visible(false);
                }
                if let Some(bin) = item.binary.clone() {
                    let cursor = Cursor::new(bin);
                    if let Some(pixbuf) = Pixbuf::from_read(cursor).ok() {
                        let image = Image::from_pixbuf(Some(&pixbuf));
                        builder.icon_holder.append(&image);
                    }
                } else {
                    builder.icon.set_visible(false);
                }
                let mut attrs: Vec<Option<(&str, &str)>> = match &item.hidden {
                    Some(a) => a.iter().map(|(k, v)|Some((k.as_str(), v.as_str()))).collect(),
                    None => Vec::new(),
                };

                if let Some(field) = &item.field {
                    attrs.push(Some(("field", field.as_str())));
                }

                let method = item.method.as_deref().or(Some(method));
                let result: Option<&str> = item.result.as_deref().or(item.title.as_deref());

                let attrs = builder.add_default_attrs(method, result, Some(keyword), None, attrs);
                builder.object.connect(
                    "row-should-activate",
                    false,
                    move |_row| {
                        execute_from_attrs(&attrs);
                        None
                    },
                );
                results.push(builder.object);
            }
        }

        return results;
    }
}
