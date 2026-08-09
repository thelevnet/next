use std::process::Command;

pub fn run(command: &str) -> bool {
    Command::new("sh").arg("-c").arg(command).status()
        .map(|s| s.success()).unwrap_or(false)
}

pub fn capture(command: &str) -> String {
    Command::new("sh").arg("-c").arg(command).output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

pub fn confirm(prompt: &str) -> bool {
    use std::io::{self, Write, BufRead};
    print!("{prompt} [y/N] ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).ok();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
