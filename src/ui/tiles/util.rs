use gtk4::{prelude::*, Label, Box};

pub fn insert_attrs(attr_holder:&Box, attrs:Vec<String>){
    for item in attrs{
        let label = Label::new(None);
        label.set_text(&item);
        attr_holder.append(&label);
    }
}


pub fn ensure_icon_name(name: String)->String{
    if name.ends_with("-symbolic"){
        name
    } else {
        format!("{}-symbolic", name)
    }
}

