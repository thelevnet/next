use crate::shell::run;
use std::io::{self, BufRead, Write};

pub fn sync() -> bool {
    print!("Commit message: ");
    io::stdout().flush().ok();
    let mut msg = String::new();
    io::stdin().lock().read_line(&mut msg).ok();
    let msg = msg.trim();

    if msg.is_empty() {
        eprintln!("Empty commit message, aborting");
        return false;
    }

    run(&format!("git -C /etc/nixos add -A && git -C /etc/nixos commit -m {msg:?} && git -C /etc/nixos push"))
}


