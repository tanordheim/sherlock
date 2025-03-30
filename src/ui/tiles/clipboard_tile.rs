use gio::glib::Bytes;
use gtk4::{gdk, prelude::*, Image};
use meval::eval_str;
use regex::Regex;
use std::collections::HashMap;

use crate::launcher::{Launcher, ResultItem};

use super::util::TileBuilder;
use super::Tile;
fn hex_to_rgb(hex_color: &str) -> (u8, u8, u8) {
    let default = (0, 0, 0);
    if hex_color.len() >= 6 {
        let Ok(r) = u8::from_str_radix(&hex_color[0..2], 16) else {
            return default;
        };
        let Ok(g) = u8::from_str_radix(&hex_color[2..4], 16) else {
            return default;
        };
        let Ok(b) = u8::from_str_radix(&hex_color[4..6], 16) else {
            return default;
        };
        return (r, g, b);
    }
    default
}

impl Tile {
    pub fn clipboard_tile(
        launcher: &Launcher,
        clipboard_content: &str,
        keyword: &str,
    ) -> Vec<ResultItem> {
        let mut results: Vec<ResultItem> = Default::default();
        let mut is_valid = false;

        //TODO implement searchstring before clipboard content
        if !clipboard_content.is_empty() && clipboard_content.contains(keyword) {
            let mut builder = TileBuilder::default();
            let mut name = "";
            let mut method = "";
            let mut icon = "";

            let known_pages = HashMap::from([
                ("google", "google"),
                ("chatgpt", "chat-gpt"),
                ("youtube", "sherlock-youtube"),
            ]);

            // Check if clipboard content is a url:
            let checker = r"^(https?:\/\/)?(www\.)?([\da-z\.-]+)\.([a-z]{2,6})([\/\w\.-]*)*\/?$|^#([A-Za-z0-9]{6,8})$";
            let re = Regex::new(checker).unwrap();
            if let Some(captures) = re.captures(clipboard_content) {
                name = "From Clipboard";
                if let Some(main_domain) = captures.get(3) {
                    builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
                    builder.object.set_spawn_focus(launcher.spawn_focus);
                    is_valid = true;
                    method = "web_launcher";
                    let main_domain = main_domain.as_str();
                    icon = known_pages.get(main_domain).map_or("google", |m| m);
                } else if let Some(hex_color) = captures.get(6) {
                    builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
                    builder.object.set_spawn_focus(launcher.spawn_focus);
                    let (r, g, b) = hex_to_rgb(hex_color.as_str());
                    let pix_buf = vec![r, g, b];
                    let image_buf = gdk::gdk_pixbuf::Pixbuf::from_bytes(
                        &Bytes::from_owned(pix_buf),
                        gdk::gdk_pixbuf::Colorspace::Rgb,
                        false,
                        8,
                        1,
                        1,
                        3,
                    );
                    if let Some(image_buf) =
                        image_buf.scale_simple(30, 30, gdk::gdk_pixbuf::InterpType::Nearest)
                    {
                        let image = Image::from_pixbuf(Some(&image_buf));
                        builder.icon_holder.append(&image);
                        image.set_widget_name("icon");
                        builder.icon_holder.set_overflow(gtk4::Overflow::Hidden);
                        builder.icon_holder.set_widget_name("color-icon-holder");
                        image.set_pixel_size(22);
                        builder.icon.set_visible(false);

                        is_valid = true;
                    };

                    // Clipboard matches a hex color
                }
            } else if let Ok(result) = eval_str(clipboard_content) {
                return Tile::calc_tile(launcher, clipboard_content, Some(result));
            }

            if is_valid {
                if name.is_empty() {
                    builder.category.set_visible(false);
                }

                builder.category.set_text(name);
                builder.title.set_text(clipboard_content);
                builder.icon.set_icon_name(Some(&icon));

                let attrs: Vec<(&str, &str)> = vec![("engine", "plain")];
                builder.add_default_attrs(
                    Some(method),
                    None,
                    Some(clipboard_content),
                    None,
                    Some(attrs),
                );

                let shortcut_holder = match launcher.shortcut {
                    true => builder.shortcut_holder,
                    _ => None
                };
                results.push(ResultItem {
                    priority: launcher.priority as f32,
                    row_item: builder.object,
                    shortcut_holder,
                });
            }
        }

        return results;
    }
}
