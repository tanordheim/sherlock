use std::cell::RefCell;
use std::rc::Rc;

use gtk4::Stack;
use gtk4::{prelude::*, ApplicationWindow};

use crate::loader::pipe_loader::deserialize_pipe;
use crate::loader::util::SherlockError;
use crate::loader::Loader;
use crate::{ui, CONFIG, FLAGS};

pub fn reload_content(
    window: &ApplicationWindow,
    stack: &Stack,
    stack_page: &Rc<RefCell<String>>,
) -> Option<()> {
    let mut startup_errors: Vec<SherlockError> = Vec::new();
    let mut non_breaking: Vec<SherlockError> = Vec::new();
    let app_config = CONFIG.get()?;
    let sherlock_flags = FLAGS.get()?;

    let (launchers, n) = Loader::load_launchers_sync()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();
    non_breaking.extend(n);

    while let Some(x) = stack.first_child() {
        stack.remove(&x);
    }

    let pipe = Loader::load_pipe_args();
    let search_stack = if pipe.is_empty() {
        ui::search::search(&launchers, &window, stack_page)
    } else {
        if sherlock_flags.display_raw {
            let pipe = String::from_utf8_lossy(&pipe);
            ui::user::display_raw(pipe, sherlock_flags.center_raw)
        } else {
            let parsed = deserialize_pipe(pipe);
            if let Some(c) = CONFIG.get() {
                let method: &str = c.pipe.method.as_deref().unwrap_or("print");
                ui::user::display_pipe(window, parsed, method)
            } else {
                return None;
            }
        }
    };
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
