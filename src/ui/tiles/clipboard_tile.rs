use gio::glib::Bytes;
use gtk4::{gdk, prelude::*, Image};
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::launcher::calc_launcher::Calculator;
use crate::launcher::clipboard_launcher::ClipboardLauncher;
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
        clp: &ClipboardLauncher,
        calc: &Calculator,
        keyword: &str,
    ) -> Vec<ResultItem> {
        let mut results: Vec<ResultItem> = Default::default();
        let mut is_valid = false;
        let clipboard_content = &clp.clipboard_content;
        let capabilities: HashSet<&str> = match &clp.capabilities {
            Some(c) => c.iter().map(|s| s.as_str()).collect(),
            _ => HashSet::from(["url", "hex", "calc.math", "calc.length"]),
        };

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
            let url_raw = r"^(https?:\/\/)?(www\.)?([\da-z\.-]+)\.([a-z]{2,6})([\/\w\.-]*)*\/?$";
            let hex_raw = r"^#([A-Za-z0-9]{6,8})$";
            let url_re = Regex::new(url_raw).unwrap();
            let hex_re = Regex::new(hex_raw).unwrap();
            if capabilities.contains("url") {
                if let Some(captures) = url_re.captures(clipboard_content) {
                    name = "From Clipboard";
                    if let Some(main_domain) = captures.get(3) {
                        // setting up builder
                        builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
                        builder.object.set_spawn_focus(launcher.spawn_focus);
                        builder.object.set_shortcut(launcher.shortcut);

                        is_valid = true;
                        method = "web_launcher";
                        let main_domain = main_domain.as_str();
                        icon = known_pages.get(main_domain).map_or("sherlock-link", |m| m);
                    }
                };
            };
            if capabilities.contains("hex") && !is_valid {
                if let Some(captures) = hex_re.captures(clipboard_content) {
                    name = "From Clipboard";
                    if let Some(hex_color) = captures.get(1) {
                        builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
                        builder.object.set_spawn_focus(launcher.spawn_focus);
                        builder.object.set_shortcut(launcher.shortcut);
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
                    };
                }
            };
            // calc capabilities will be checked inside of calc file
            if !is_valid {
                name = "From Clipboard";
                results.extend(Tile::calc_tile(launcher, calc, clipboard_content));
            };

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
                    _ => None,
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
