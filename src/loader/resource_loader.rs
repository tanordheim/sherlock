use gio::glib;

use super::Loader;

impl Loader {
    pub fn load_resources(){
        let res_bytes = include_bytes!("../../resources.gresources");
        let resource = gio::Resource::from_data(&glib::Bytes::from_static(res_bytes))
            .expect("Failed to load resources in helpers.");
        gio::resources_register(&resource);
    }
}

