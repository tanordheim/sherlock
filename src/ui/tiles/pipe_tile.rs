use std::io::Cursor;

use crate::actions::execute_from_attrs;
use crate::actions::get_attrs_map;
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
        let mut results: Vec<SherlockRow> = Vec::with_capacity(lines.len());

        for item in lines {
            if item.fuzzy_match(keyword) || item.binary.is_some() {
                let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");

                if let Some(title) = &item.title {
                    builder.title.upgrade().map(|t| t.set_text(&title));
                }
                builder.category.upgrade().map(|category| {
                    if let Some(desc) = &item.description {
                        category.set_text(&desc);
                    } else {
                        category.set_visible(false);
                    }
                });
                builder.icon.upgrade().map(|ico| {
                    if let Some(icon) = &item.icon {
                        ico.set_icon_name(Some(&icon));
                    } else {
                        ico.set_visible(false);
                    }
                });
                // Custom Image Data
                if let Some(bin) = item.binary.clone() {
                    let cursor = Cursor::new(bin);
                    if let Some(pixbuf) = Pixbuf::from_read(cursor).ok() {
                        let image = Image::from_pixbuf(Some(&pixbuf));
                        builder.icon_holder.upgrade().map(|holder| holder.append(&image));
                        if let Some(size) = &item.icon_size {
                            image.set_pixel_size(*size);
                        }
                    }
                } else {
                    builder.icon.upgrade().map(|icon| icon.set_visible(false));
                }

                // Create attributes and enable action capability
                let method = item.method.as_deref().unwrap_or(method);
                let result = item.result.as_deref().or(item.title.as_deref());
                let mut constructor: Vec<(&str, &str)> =
                    item.hidden.as_ref().map_or_else(Vec::new, |a| {
                        a.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect()
                    });
                constructor.extend([("method", method), ("keyword", keyword)]);
                if let Some(result) = result {
                    constructor.push(("result", result))
                }
                if let Some(field) = &item.field {
                    constructor.push(("field", field));
                }
                let attrs = get_attrs_map(constructor);

                builder.object.set_spawn_focus(true);
                builder.object.connect("row-should-activate", false, move |row| {
                    let row = row.first().map(|f| f.get::<SherlockRow>().ok())??;
                    execute_from_attrs(&row, &attrs);
                    None
                });
                results.push(builder.object);
            }
        }
        return results;
    }
}
