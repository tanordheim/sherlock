use std::process::{exit, Command};

pub fn applaunch(exec: &str) {
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
