use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use procfs::process::{all_processes, Process};
use std::collections::HashMap;

use crate::loader::util::{SherlockError, SherlockErrorType};

#[derive(Clone, Debug)]
pub struct ProcessLauncher {
    pub icon: String,
    pub processes: HashMap<(i32, i32), String>,
}

impl ProcessLauncher {
    pub fn new(icon: &str) -> Option<Self> {
        if let Some(processes) = get_all_processes() {
            return Some(Self {
                processes,
                icon: icon.to_string(),
            });
        } else {
            return None;
        }
    }
    pub fn kill(pid: (i32, i32)) -> Result<(), SherlockError> {
        if pid.0 != pid.1 {
            let child = Pid::from_raw(pid.1);
            kill(child, Signal::SIGKILL).map_err(|e| SherlockError {
                error: SherlockErrorType::CommandExecutionError(format!(
                    "Kill process \"{}\"",
                    child
                )),
                traceback: e.to_string(),
            })?;
        };
        let parent = Pid::from_raw(pid.0);
        kill(parent, Signal::SIGKILL).map_err(|e| SherlockError {
            error: SherlockErrorType::CommandExecutionError(format!("Kill process \"{}\"", parent)),
            traceback: e.to_string(),
        })
    }
}

fn get_all_processes() -> Option<HashMap<(i32, i32), String>> {
    match all_processes() {
        Ok(procs) => {
            let user_processes: Vec<Process> = procs
                .flatten()
                .filter_map(|p| match p.uid() {
                    Ok(uid) if uid > 0 => Some(p),
                    _ => None,
                })
                .collect();
            let mut process_names: HashMap<i32, String> = user_processes
                .iter()
                .filter_map(|p| match p.exe() {
                    Ok(path) => path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|name| (p.pid, name.to_string())),
                    _ => None,
                })
                .collect();

            let stats = user_processes.iter().filter_map(|p| p.stat().ok());
            let mut collected: HashMap<(i32, i32), String> = HashMap::new();
            let mut tmp: HashMap<i32, i32> = HashMap::new();
            for item in stats.rev() {
                if item.ppid == 1 {
                    let named_id = tmp.get(&item.pid).copied().unwrap_or(item.pid);
                    if let Some(name) = process_names.remove(&named_id) {
                        collected.insert((item.pid, named_id), name);
                    }
                } else if item.tty_nr != 0 {
                    if let Some(r) = tmp.remove(&item.pid) {
                        tmp.insert(item.ppid, r);
                    } else if tmp.get(&item.ppid).is_none() {
                        tmp.insert(item.ppid, item.pid);
                    }
                } else if tmp.get(&item.ppid).is_none() {
                    tmp.insert(item.ppid, item.pid);
                }
            }

            Some(collected)
        }
        Err(_) => None,
    }
}
