use std::process::Command;

pub fn command_launch(exec: &str) {
    let mut parts = exec.split_whitespace();
    let command = parts.next().expect("No command found.");
    let args: Vec<&str> = parts.collect();

    let output = Command::new(command)
        .args(args)
        .output()
        .expect(format!("Failed to execute command: {:?}", command).as_str());

    if output.status.success() {
        println!("Command executed successfully!");
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

