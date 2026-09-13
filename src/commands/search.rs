use serde::Deserialize;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Deserialize)]
struct SearchResponse {
    #[serde(default)]
    results: Vec<PackageResult>,
}

#[derive(Deserialize)]
struct PackageResult {
    package_attr_name: Option<String>,
    package_description: Option<String>,
}

pub fn apps(initial_query: Option<String>) -> bool {
    let self_exe = std::env::current_exe().unwrap_or_else(|_| "next".into());
    let reload_cmd = format!("{} __search-query {{q}}", self_exe.display());

    let mut cmd = Command::new("fzf");
    cmd.args([
        "--disabled",
        "--layout=reverse",
        "--height=50%",
        "--border=rounded",
        "--prompt=󰍪 nixpkgs> ",
        "--pointer=▶",
        "--marker=✓",
        "--tabstop=32",
        "--header=Type to search nixpkgs | Enter: copy & print | Esc: exit",
        "--bind",
        &format!("start,change:reload:{reload_cmd}"),
    ]);

    if let Some(ref q) = initial_query {
        let trimmed = q.trim();
        if !trimmed.is_empty() {
            cmd.arg(format!("--query={trimmed}"));
        }
    }

    let output = match cmd.output() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Error launching fzf: {e}");
            return false;
        }
    };

    if !output.status.success() {
        return true;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let selected = stdout.trim();
    if selected.is_empty() {
        return true;
    }

    let (name, desc) = selected.split_once('\t').unwrap_or((selected, ""));
    let name = name.trim();
    let desc = desc.trim();

    copy_to_clipboard(name);

    println!("\x1b[32m✔\x1b[0m Copied '\x1b[1;36m{name}\x1b[0m' to clipboard!");
    println!("\x1b[1mName:\x1b[0m        {name}");
    println!("\x1b[1mDescription:\x1b[0m {desc}");

    true
}

pub fn query(words: &[String]) {
    let query_str = words.join(" ");
    let query_arg = query_str.trim();

    let output = Command::new("nh")
        .args(["search", "packages", query_arg, "-l", "40", "--json"])
        .stderr(Stdio::null())
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            if let Ok(resp) = serde_json::from_slice::<SearchResponse>(&out.stdout) {
                let mut stdout = std::io::stdout().lock();
                for item in resp.results {
                    if let Some(name) = item.package_attr_name {
                        let desc = item
                            .package_description
                            .as_deref()
                            .unwrap_or("No description")
                            .replace(['\n', '\r', '\t'], " ");
                        if writeln!(stdout, "{name}\t{desc}").is_err() {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn copy_to_clipboard(text: &str) {
    // 1. Wayland clipboard
    let _ = Command::new("wl-copy").arg(text).status();

    // 2. X11 clipboard fallback
    if let Ok(mut child) = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }

    // 3. Terminal OSC 52 sequence (Kitty, Tmux, SSH, etc.)
    let b64 = to_base64(text);
    let _ = write!(std::io::stderr(), "\x1b]52;c;{b64}\x07");
    let _ = std::io::stderr().flush();
}

fn to_base64(s: &str) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = s.as_bytes();
    let mut res = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        res.push(CHARSET[(b0 >> 2) as usize] as char);
        res.push(CHARSET[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            res.push(CHARSET[(((b1 & 15) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            res.push('=');
        }
        if chunk.len() > 2 {
            res.push(CHARSET[(b2 & 63) as usize] as char);
        } else {
            res.push('=');
        }
    }
    res
}
