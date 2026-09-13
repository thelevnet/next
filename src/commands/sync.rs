use crate::shell::run;
use std::io::{self, BufRead, Write};
use std::process::Command;

pub fn sync(message: Option<String>) -> bool {
    let has_changes = Command::new("git")
        .args(["-C", "/etc/nixos", "status", "--porcelain"])
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    if has_changes {
        let msg = match message {
            Some(m) if !m.trim().is_empty() => m.trim().to_string(),
            _ => {
                print!("Commit message: ");
                io::stdout().flush().ok();
                let mut input = String::new();
                io::stdin().lock().read_line(&mut input).ok();
                let trimmed = input.trim().to_string();
                if trimmed.is_empty() {
                    eprintln!("Empty commit message, aborting");
                    return false;
                }
                trimmed
            }
        };

        if !run(&format!("git -C /etc/nixos add -A && git -C /etc/nixos commit -m {msg:?}")) {
            return false;
        }
    }

    run("git -C /etc/nixos pull --no-rebase --no-edit && git -C /etc/nixos push")
}
