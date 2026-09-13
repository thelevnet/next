use std::process::Command;

pub fn run(command: &str) -> bool {
    Command::new("sh").arg("-c").arg(command).status()
        .map(|s| s.success()).unwrap_or(false)
}

