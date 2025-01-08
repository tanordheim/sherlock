use gtk4::gdk::{self, Rectangle};
use gtk4::{prelude::*, EventControllerKey, ListBoxRow, ScrolledWindow};
use gtk4::{ApplicationWindow, Box as HVBox, Builder, Entry, ListBox, Label};
use std::cell::RefCell;
use std::collections::HashMap;
use std::process::{exit, Command};
use std::rc::Rc;
use std::{env, path::Path};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::components::launchers;
use crate::helpers;
use helpers::{read_from_label, select_first_row};
use launchers::{get_launchers, launcher_loop, Launcher};


pub fn search(window: ApplicationWindow) -> ApplicationWindow {
    let launchers: Vec<Launcher> = get_launchers();

    // Collect Modes
    let mode = Rc::new(RefCell::new("all".to_string()));
    let mut modes: HashMap<String, String> = HashMap::new();
    for item in launchers.iter(){
        let alias = item.alias();
        if !alias.is_empty() {
            let name= item.name();
            modes.insert(format!("{} ", alias), name);
        }
    }

    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/com/skxxtz/sherlock/ui/search.ui");

    // Get the requred object references
    let vbox: HVBox = builder.object("vbox").unwrap();
    let search_bar: Entry = builder.object("search-bar").unwrap();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let mode_title: Label = builder.object("category-type-label").unwrap();
    let results: ListBox = builder.object("result-frame").unwrap();

    //RC cloning:
    let results = Rc::new(results);


    let mode_clone_ev_changed = Rc::clone(&mode);
    let mode_clone_ev_nav = Rc::clone(&mode);
    let mode_title_clone = mode_title.clone();

    let results_enter = Rc::clone(&results);
    let results_clone_ev_nav = Rc::clone(&results);

    let launchers_clone_ev_changed = launchers.clone();
    let launchers_clone_ev_nav = launchers.clone();

    // Initiallize the view to show all apps
    set_results("","all", &*results, &launchers);

    // Setting search window to active
    window.set_child(Some(&vbox));
    search_bar.grab_focus();

    // Eventhandling for text change inside search bar
    // EVENT: CHANGE
    search_bar.connect_changed(move |search_bar| {
        let current_text = search_bar.text().to_string();


        // Check if current text is present in modes
        if modes.contains_key(&current_text) {
            if let Some(mode_name) = modes.get(&current_text){
                set_mode(&mode_title_clone, &mode_clone_ev_changed, &current_text, mode_name);
                search_bar.set_text("");
            }
        } else {
            // Get the new widgets for the specific input text
            set_results(&current_text,&mode_clone_ev_changed.borrow(), &*results, &launchers_clone_ev_changed);
        }
    });


    // Eventhandling for navigation
    // EVENT: Navigate
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, modifiers| {
        match key {
            gdk::Key::Up => {
                let new_row = select_row(-1, &results_clone_ev_nav);

                let row_allocation = new_row.allocation();
                let row_rect = Rectangle::from(row_allocation);

                let row_start = (row_rect.y()) as f64;
                let vadjustment = result_viewport.vadjustment();

                let current_value = vadjustment.value();
                if current_value > row_start {
                    vadjustment.set_value(row_start);
                } 
            },
            gdk::Key::Down => {
                select_row(1, &results_clone_ev_nav);
                let allocation = result_viewport.allocation();
                let list_box_rect = Rectangle::from(allocation);

                let row_allocation = results_clone_ev_nav.selected_row().unwrap().allocation();
                let row_rect = Rectangle::from(row_allocation);

                let list_height = list_box_rect.height() as f64;
                let row_end = (row_rect.y() + row_rect.height() + 10) as f64;
                let vadjustment = result_viewport.vadjustment();

                let current_value = vadjustment.value();
                let list_end = list_height + current_value;
                if row_end > list_end {
                    let delta = row_end - list_end;
                    let new_value = current_value + delta;
                    vadjustment.set_value(new_value);
                }
            },
            gdk::Key::BackSpace => {
                let ctext = &search_bar.text();
                if ctext.is_empty(){
                    set_mode(&mode_title, &mode_clone_ev_nav, "all", &"All".to_string());
                    set_results(&ctext,&mode_clone_ev_nav.borrow(), &*results_clone_ev_nav, &launchers_clone_ev_nav);
                }
            },
            gdk::Key::Return => {
                if let Some(row) = results_enter.selected_row(){
                    let attrs: HashMap<String, String> = get_row_attrs(row);
                    execute_from_attrs(attrs);
                }
            },
            gdk::Key::_1 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK){
                    execute_by_index(&*results_clone_ev_nav, 1);
                }
            },
            gdk::Key::_2 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK){
                    execute_by_index(&*results_clone_ev_nav, 2);
                }
            },
            gdk::Key::_3 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK){
                    execute_by_index(&*results_clone_ev_nav, 3);
                }
            },
            gdk::Key::_4 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK){
                    execute_by_index(&*results_clone_ev_nav, 4);
                }
            },
            gdk::Key::_5 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK){
                    execute_by_index(&*results_clone_ev_nav, 5);
                }
            },

            _ => (),
        }
        false.into()
    });

    window.add_controller(event_controller);

    return window;
}

fn set_mode(mode_title:&Label, mode_c:&Rc<RefCell<String>>, ctext:&str, mode_name:&String){
    let new_mode = ctext.to_string();
    mode_title.set_text(mode_name);
    *mode_c.borrow_mut() = new_mode;

}

fn execute_by_index(results:&ListBox, index:i32){
    if let Some(row) = results.row_at_index(index-1){
        let attrs = get_row_attrs(row);
        execute_from_attrs(attrs);
    }

    
    
}
fn get_row_attrs(selected_row:ListBoxRow)->HashMap<String, String>{
    let mut attrs: HashMap<String, String> = Default::default();
    if let Some(main_holder) = selected_row.first_child() {
        if let Some(attrs_holder) = main_holder.first_child() {
            if let Some(first_label_obj) = attrs_holder.first_child() {
                if let Some(text) = read_from_label(&first_label_obj) {
                    attrs.insert(text.0, text.1);
                }
                let mut current_label_obj = first_label_obj;
                while let Some(next_label_obj) = current_label_obj.next_sibling() {
                    if let Some(text) = read_from_label(&next_label_obj) {
                        attrs.insert(text.0, text.1);
                    }
                    current_label_obj = next_label_obj;
                }
            }
        }
    }
    attrs
}

fn set_results(keyword:&str,mode:&str, results_frame:&ListBox, launchers:&Vec<Launcher>){
    // Remove all elements inside to avoid duplicates
    while let Some(row) = results_frame.last_child() {
        results_frame.remove(&row);
    }
    let widgets = launcher_loop(&keyword.to_string(), &launchers, &mode.to_string());
    for widget in widgets {
        results_frame.append(&widget);
    }
    select_first_row(&results_frame);


}

fn select_row(offset: i32, listbox:&Rc<ListBox>)->ListBoxRow{
    if let Some(row) = listbox.selected_row(){
        let new_index = row.index() + offset;
        if let Some(new_row) = listbox.row_at_index(new_index){
            listbox.select_row(Some(&new_row));
            return new_row
        };
    };
    return ListBoxRow::new();
}

fn execute_from_attrs(attrs: HashMap<String, String>) {
    if let Some(method) = attrs.get("method") {
        match method.as_str() {
            "app" => {
                let exec = attrs.get("exec").expect("Missing field: exec");
                applaunch(exec);
                exit(0);
            }
            "web" => {
                let query = attrs
                    .get("keyword")
                    .expect("Missing field: keyword (query)");
                let engine = attrs.get("engine").expect("Missing field: engine (query)");
                websearch(engine, query);
                exit(0);
            },
            "command" => {
                let exec = attrs.get("exec").expect("Missing field: exec");
                command_launch(exec);
                exit(0)
            },
            "calc" => {
                let string = attrs.get("result").expect("Missing field: result");
                copy_to_clipboard(string);
            }
            _ => {
                eprint!("Invalid method: {}", method)
            }
        }
    }
}

fn websearch(engine: &str, query: &str) {
    let mut engines: HashMap<&str, &str> = Default::default();
    engines.insert("google", "https://www.google.com/search?q={}");
    engines.insert("bing", "https://www.bing.com/search?q={}");
    engines.insert("duckduckgo", "https://duckduckgo.com/?q={}");
    if let Some(url_template) = engines.get(engine) {
        let url = url_template.replace("{}", query);
        let _ = open::that(url);
    }
}

fn applaunch(exec: &str) {
    let parts: Vec<String> = exec.split_whitespace().map(String::from).collect();

    if parts.is_empty() {
        eprintln!("Error: Command is empty");
        exit(1);
    }

    let mut command = Command::new(&parts[0]);
    for arg in &parts[1..] {
        if !arg.starts_with("%") {
            command.arg(arg);
        }
    }

    let _output = command.spawn().expect("Failed to start the application");
}

fn command_launch(exec: &str) {
    let mut parts = exec.split_whitespace();
    let command = parts.next().expect("No command found.");
    let args: Vec<&str> = parts.collect();

    let output = Command::new(command)
        .args(args)
        .output()
        .expect(format!("Failed to execute command: {:?}", command).as_str());

    if output.status.success() {
        println!("Command executed successfully!");
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
fn copy_to_clipboard(string:&String){
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(string.to_owned()).unwrap();


}
