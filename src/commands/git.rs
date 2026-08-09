use crate::config::REPO_PATH;
use crate::shell::run;
use std::io::{self, Write, BufRead};

pub fn sync() {
    run(&format!("git -C {REPO_PATH} add -A"));

    print!("commit message: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).ok();
    let msg = input.trim().to_string();

    if msg.is_empty() {
        eprintln!("empty commit message, aborting");
        return;
    }

    run(&format!("git -C {REPO_PATH} commit -m \"{msg}\""));
    run(&format!("git -C {REPO_PATH} push"));
}
