use std::{os::unix::process::CommandExt, process::{exit, Command, Stdio}};

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

    #[cfg(target_family = "unix")]
    unsafe {   
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .pre_exec(||{
                nix::unistd::setsid().ok();
                Ok(())
            });
    }

    let _output = command.spawn().expect("Failed to start the application");
}
