use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

pub fn applaunch(exec: &str) -> Option<()> {
    let mut parts = exec
        .trim()
        .split_whitespace()
        .filter(|s| !s.starts_with("%"));

    let mut command = Command::new(parts.next()?);
    command.args(parts);

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
    let _ = command
        .spawn()
        .map_err(|e| eprintln!("Error executing command: {}", e));
    None
}
