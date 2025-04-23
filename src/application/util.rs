use std::cell::RefCell;
use std::rc::Rc;

use gio::glib::WeakRef;
use gtk4::Stack;
use gtk4::{prelude::*, ApplicationWindow};

use crate::loader::util::SherlockError;
use crate::loader::Loader;
use crate::{ui, CONFIG};

pub fn reload_content(
    window: &ApplicationWindow,
    stack: &WeakRef<Stack>,
    stack_page: &Rc<RefCell<String>>,
) -> Option<()> {
    let mut startup_errors: Vec<SherlockError> = Vec::new();
    let mut non_breaking: Vec<SherlockError> = Vec::new();
    let app_config = CONFIG.get()?;
    let stack = stack.upgrade()?;

    let (launchers, n) = Loader::load_launchers_sync()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();
    non_breaking.extend(n);

    while let Some(x) = stack.first_child() {
        stack.remove(&x);
    }

    let search_stack = ui::search::search(&launchers, &window, stack_page);
    stack.add_named(&search_stack, Some("search-page"));

    // Logic for the Error-View
    let error_stack = ui::error_view::errors(&startup_errors, &non_breaking, stack_page);
    stack.add_named(&error_stack, Some("error-page"));
    if !app_config.debug.try_suppress_errors {
        let show_errors = !startup_errors.is_empty();
        let show_warnings = !app_config.debug.try_suppress_warnings && !non_breaking.is_empty();
        if show_errors || show_warnings {
            let _ = gtk4::prelude::WidgetExt::activate_action(
                window,
                "win.switch-page",
                Some(&String::from("error-page").to_variant()),
            );
        } else {
            let _ = gtk4::prelude::WidgetExt::activate_action(
                window,
                "win.switch-page",
                Some(&String::from("search-page").to_variant()),
            );
        }
    }
    None
}
