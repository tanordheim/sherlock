use std::env;
use super::{Loader, util::SherlockFlags};


impl Loader {
    pub fn load_flags()->SherlockFlags{
        let args: Vec<String> = env::args().collect();
        SherlockFlags::new(args)
    }
}

impl SherlockFlags {
    fn new(args: Vec<String>) -> Self {
        let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
        let defaults = SherlockFlags::default();

        // Helper closure to extract flag values
        let extract_flag = |flag: &str, default: String| {
            args.iter()
                .position(|arg| arg == flag)
                .and_then(|index| args.get(index + 1))
                .map_or(default,|f| f.replace("", &home_dir).to_string())
                .to_string()
        };

        SherlockFlags {
            config: extract_flag("--config", defaults.config),
            fallback: extract_flag("--fallback", defaults.fallback),
            style: extract_flag("--style", defaults.style),
            ignore: extract_flag("--ignore", defaults.ignore),
            alias: extract_flag("--alias", defaults.alias),
        }
    }

    fn default() -> Self {
        let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/user"));
        SherlockFlags {
            config: format!("{}/.config/sherlock/config.toml", home_dir),
            fallback: format!("{}/.config/sherlock/fallback.json", home_dir),
            style: format!("{}/.config/sherlock/main.css", home_dir),
            ignore: format!("{}/.config/sherlock/sherlockignore", home_dir),
            alias: format!("{}/.config/sherlock/sherlock_alias.json", home_dir),
        }
    }
}
