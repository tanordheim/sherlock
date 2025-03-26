use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use crate::loader::util::{SherlockError, SherlockErrorType};

pub fn command_launch(exec: &str, keyword: &str) -> Result<(), SherlockError> {
    let exec = exec.replace("{keyword}", &keyword);
    let commands: Vec<&str> = exec.split("&").collect();

    for command in commands {
        if !command.is_empty() {
            let mut parts = command.split_whitespace();
            let execute = parts.next().expect("No command found");
            let args: Vec<&str> = parts.collect();
            asynchronous_execution(execute, args)?;
        }
    }
    Ok(())
}

fn asynchronous_execution(execute: &str, args: Vec<&str>) -> Result<(), SherlockError> {
    let mut command = Command::new(execute);
    unsafe {
        command
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .pre_exec(|| {
                nix::unistd::setsid().ok();
                Ok(())
            });
    }

    command.spawn().map_err(|e| SherlockError {
        error: SherlockErrorType::CommandExecutionError(execute.to_string()),
        traceback: e.to_string(),
    })?;

    Ok(())
}
