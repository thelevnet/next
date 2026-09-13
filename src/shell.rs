use std::process::Command;

pub const REPO_PATH: &str = "/etc/nixos";

pub fn run(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "desktop".to_string())
}

pub fn username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "lev".to_string())
}

