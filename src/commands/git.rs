use crate::messages::*;
use crate::config::REPO_PATH;
use crate::shell::run;
use std::io::{self, Write, BufRead};

pub fn sync() {
    run(&format!("git -C {REPO_PATH} add -A"));

    print!("{PREFIX} {PROMPT_COMMIT_MSG}: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).ok();
    let msg = input.trim().to_string();

    if msg.is_empty() {
        eprintln!("{PREFIX} {ERR_EMPTY_COMMIT_MSG}");
        return;
    }

    run(&format!("git -C {REPO_PATH} commit -m \"{msg}\""));
    run(&format!("git -C {REPO_PATH} push"));
}

pub fn commit_push(msg: String) {
    println!("{PREFIX} {COMMIT_PUSH_MSG} \"\n{msg}\"\n");
    run(&format!("git -C {REPO_PATH} add -A"));
    run(&format!("git -C {REPO_PATH} commit -m \"{msg}\""));
    run(&format!("git -C {REPO_PATH} push"));

}
