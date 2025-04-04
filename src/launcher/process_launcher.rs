use procfs::process::all_processes;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ProcessLauncher {
    pub processes: HashMap<i32, String>
}

impl ProcessLauncher {
    pub fn new()->Option<Self>{
         match all_processes() {
            Ok(procs) => {
                let processes: HashMap<i32, String> = procs
                    .flatten()
                    .into_iter()
                    .filter_map(|proc| proc.stat().ok())
                    .filter(|stat| stat.tpgid > 0)
                    .map(|stat| (stat.pid, stat.comm))
                    .collect();

                return Some(Self { processes })
            }
            Err(_) => return None
        }
    }

}

