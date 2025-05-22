use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use crate::CONFIG;
use crate::{
    sherlock_error,
    utils::errors::{SherlockError, SherlockErrorType},
};
pub fn command_launch(exec: &str, keyword: &str) -> Result<(), SherlockError> {
    let config = CONFIG
        .get()
        .ok_or(sherlock_error!(SherlockErrorType::ConfigError(None), ""))?;
    let prefix = config
        .behavior
        .global_prefix
        .as_ref()
        .map_or(String::new(), |p| format!("{} ", p));
    let flags = config
        .behavior
        .global_flags
        .as_ref()
        .map_or(String::new(), |f| format!(" {}", f));

    let exec = exec.replace("{keyword}", &keyword);
    let commands = exec.split("&").map(|s| s.trim()).filter(|s| !s.is_empty());

    for command in commands {
        asynchronous_execution(command, &prefix, &flags)?;
    }
    Ok(())
}

fn asynchronous_execution(cmd: &str, prefix: &str, flags: &str) -> Result<(), SherlockError> {
    let raw_command = format!("{}{}{}", prefix, cmd, flags).replace('"', "");

    let mut command = Command::new("sh");
    command.arg("-c").arg(raw_command);

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

    command.spawn().map_err(|e| {
        sherlock_error!(
            SherlockErrorType::CommandExecutionError(cmd.to_string()),
            e.to_string()
        )
    })?;

    Ok(())
}
