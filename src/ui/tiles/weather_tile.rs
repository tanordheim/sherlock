use std::collections::HashMap;

use gtk4::prelude::*;

use crate::launcher::weather_launcher::WeatherLauncher;
use crate::launcher::{Launcher, ResultItem};

use super::util::{AsyncLauncherTile, WeatherTileBuilder, WeatherTileElements};
use super::Tile;

impl Tile {
    pub fn weather_tile_loader(
        launcher: Launcher,
        wtr: &WeatherLauncher,
    ) -> Option<AsyncLauncherTile>  {
        let title_case = |input_str:&str| {
            let mut result = String::with_capacity(input_str.len());
            let mut cap_next = true;
            for c in input_str.chars() {
                if c.is_whitespace() {
                    cap_next = true;
                    result.push(c);
                } else if cap_next {
                    for up in c.to_uppercase() {
                        result.push(up)
                    }
                    cap_next = false;
                } else {
                    result.push(c);
                }
            }
            result
        };

        let builder = WeatherTileBuilder::new("/dev/skxxtz/sherlock/ui/weather_tile.ui");
        builder.object.add_css_class("weather-tile");
        builder.object.set_spawn_focus(launcher.spawn_focus);
        builder.object.set_shortcut(launcher.shortcut);

        let loc = title_case(&wtr.location);
        // Add attrs and implement double click capabilities
        let attrs: HashMap<String, String> =
            vec![("method", &launcher.method), ("location", &loc)]
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

        builder.icon.set_pixel_size(40);

        let result_item = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
            shortcut_holder: None
        };
        let weather_tile = Some(WeatherTileElements {
            icon: builder.icon,
            temperature: builder.temperature,
            location: builder.location,
            spinner: builder.spinner,
        });
        return Some(AsyncLauncherTile{
            launcher,
            result_item,
            text_tile: None,
            image_replacement: None,
            weather_tile,
            attrs,
        });
    }
}
