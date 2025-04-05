use std::{env, path::PathBuf};

use crate::actions::util::eval_exit;

use super::{
    util::{SherlockError, SherlockFlags},
    Loader,
};

impl Loader {
    pub fn load_flags() -> Result<SherlockFlags, SherlockError> {
        let args: Vec<String> = env::args().collect();
        if args.contains(&"--help".to_string()) {
            let _ = print_help();
            eval_exit();
        }
        if args.contains(&"-h".to_string()) {
            let _ = print_help();
            eval_exit();
        }
        if args.contains(&"--version".to_string()) {
            let _ = print_version();
            eval_exit();
        }

        SherlockFlags::new(args)
    }
}
impl SherlockFlags {
    fn new(args: Vec<String>) -> Result<Self, SherlockError> {
        // Helper closure to extract flag values
        let extract_path_value = |flag: &str| {
            args.iter()
                .position(|arg| arg == flag)
                .map_or(None, |index| args.get(index + 1))
                .map(|s| PathBuf::from(s))
        };
        let check_flag_existance = |flag: &str| {
            args.iter()
                .position(|arg| arg == flag)
                .map_or(false, |_| true)
        };
        let extract_flag_value = |flag: &str| {
            args.iter()
                .position(|arg| arg == flag)
                .map_or(None, |i| args.get(i + 1))
                .cloned()
        };

        Ok(SherlockFlags {
            config: extract_path_value("--config"),
            fallback: extract_path_value("--fallback"),
            style: extract_path_value("--style"),
            ignore: extract_path_value("--ignore"),
            alias: extract_path_value("--alias"),
            display_raw: check_flag_existance("--display-raw"),
            center_raw: check_flag_existance("--center"),
            cache: extract_path_value("--cache"),
            daemonize: check_flag_existance("--daemonize"),
            method: extract_flag_value("--method"),
            field: extract_flag_value("--field"),
        })
    }
}

pub fn print_version() -> Result<(), SherlockError> {
    let version = env!("CARGO_PKG_VERSION");
    println!("Sherlock v{}", version);
    println!("Developed by Skxxtz");

    Ok(())
}
pub fn print_help() -> Result<(), SherlockError> {
    let allowed_flags: Vec<(&str, &str)> = vec![
        ("--version", "Print the version of the application."),
        ("--help", "Show this help message with allowed flags."),
        ("--config", "Specify the configuration file to load."),
        ("--fallback", "Specify the fallback file to load."),
        ("--style", "Set the style configuration file."),
        ("--ignore", "Specify the sherlock ignore file"),
        ("--alias", "Specify the sherlock alias file (.json)."),
        (
            "--display-raw",
            "Force Sherlock to use a singular tile to display the piped content",
        ),
        ("--cache", "Specify the sherlock cache file (.json)."),
        (
            "--daemonize",
            "If this flag is set, sherlock will run in daemon mode.",
        ),
        (
            "--method",
            "For pipe mode only: Specifies what to do with the selected data row",
        ),
    ];

    // Print header
    println!("{:<15} {}", "Flag", "Description");

    for (flag, explanation) in allowed_flags {
        println!("{:<15} {}", flag, explanation);
    }

    println!(
        "\n\nFor more help:\nhttps://github.com/Skxxtz/sherlock/blob/main/README.md#Flags\n\n"
    );

    Ok(())
}
