use crate::config::{Config, REPO_PATH};
use std::io::{self, BufRead, Write};
use std::process::Command;

pub fn sync() {
    let config = Config::load();
    if !config.git.enable {
        eprintln!("Git integration is disabled in config");
        return;
    }

    let _ = Command::new("git").args(["-C", REPO_PATH, "add", "-A"]).status();

    print!("Commit message: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).ok();
    let msg = input.trim().to_string();

    if msg.is_empty() {
        eprintln!("Empty commit message, aborting");
        return;
    }

    let status = Command::new("git")
        .args(["-C", REPO_PATH, "commit", "-m", &msg])
        .status();

    if let Ok(s) = status {
        if s.success() {
            let _ = Command::new("git").args(["-C", REPO_PATH, "push"]).status();
        }
    }
}


