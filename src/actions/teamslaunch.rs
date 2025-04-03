use crate::{actions::util::eval_exit, CONFIG};
use std::{
    os::unix::process::CommandExt,
    process::{exit, Command, Stdio},
};

pub fn teamslaunch(meeting_url: &str) {
    if let Some(c) = CONFIG.get() {
        let teams_command = c.default_apps.teams.clone();
        let exec = teams_command.replace("{meeting_url}", meeting_url);

        let parts: Vec<String> = exec.split_whitespace().map(String::from).collect();

        if parts.is_empty() {
            eprintln!("Error: Command is empty");
            eval_exit();
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
                .pre_exec(|| {
                    nix::unistd::setsid().ok();
                    Ok(())
                });
        }

        let _output = command.spawn().expect("Failed to start the application");
    }
}
