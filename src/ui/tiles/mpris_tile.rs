use gtk4::prelude::WidgetExt;
use gtk4::{Box, Image, Label};

use super::util::TileBuilder;
use super::Tile;
use crate::launcher::audio_launcher::MusicPlayerLauncher;
use crate::launcher::{Launcher, ResultItem};

impl Tile {
    pub fn mpris_tile(
        launcher: &Launcher,
        mpris: &MusicPlayerLauncher,
    ) -> Option<(ResultItem, Option<Label>, Option<Label>, Option<Image>, Box)> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/mpris_tile.ui", 0, true);

        builder.category.set_text(&mpris.artist);
        builder.title.set_text(&mpris.title);
        builder.object.set_overflow(gtk4::Overflow::Hidden);
        builder.icon.set_pixel_size(60);
        builder.icon.set_margin_top(10);
        builder.icon.set_margin_bottom(10);



        builder.add_default_attrs(
            Some(&launcher.method),
            None,
            None,
            None,
            None,
        );

        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
        };

        return Some((
            res,
            None,
            None,
            Some(builder.icon),
            builder.attrs,
        ));

    }
}


