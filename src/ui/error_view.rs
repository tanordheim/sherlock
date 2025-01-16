use gtk4::{prelude::GtkWindowExt, ApplicationWindow, Box as HVBox, Builder, ListBox};
use std::rc::Rc;

use crate::{loader::util::SherlockError, ui::tiles::Tile};

pub fn errors(window: &ApplicationWindow, errors: &Vec<SherlockError>){
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_view.ui");

    let vbox: HVBox = builder.object("vbox").unwrap();
    let results: Rc<ListBox> = Rc::new(builder.object("result-frame").unwrap());

    let (_, error_tiles)= Tile::error_tile(0, errors);
    error_tiles.iter().for_each(|tile| results.append(tile));

    window.set_child(Some(&vbox));
}
