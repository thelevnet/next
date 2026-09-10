use crate::config::{Config, REPO_PATH};
use crate::shell::run;
use std::io::{self, BufRead, Write};

pub fn sync() {
    let config = Config::load();
    if !config.git.enable {
        eprintln!("Git integration is disabled in config");
        return;
    }
    run(&format!("git -C {REPO_PATH} add -A"));

    print!("Commit message: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).ok();
    let msg = input.trim().to_string();

    if msg.is_empty() {
        eprintln!("Empty commit message, aborting");
        return;
    }

    run(&format!("git -C {REPO_PATH} commit -m \"{msg}\""));
    run(&format!("git -C {REPO_PATH} push"));
}

#[allow(dead_code)]
pub fn commit_push(msg: String) {
    let config = Config::load();
    if !config.git.enable {
        eprintln!("Git integration is disabled in config");
        return;
    }
    println!("Commit & push with message: \"{msg}\"");
    run(&format!("git -C {REPO_PATH} add -A"));
    run(&format!("git -C {REPO_PATH} commit -m \"{msg}\""));
    run(&format!("git -C {REPO_PATH} push"));
}


