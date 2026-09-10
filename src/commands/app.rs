use std::io::Write;
use std::process::{Command, Stdio};
use crate::commands::os;
use crate::config::Config;
use crate::nix_parser;
use crate::shell::{capture, confirm, run};

fn search_nixpkgs() -> Option<String> {
    let filter_path = "/tmp/next_search_filter.jq";
    let _ = std::fs::write(
        filter_path,
        r#".results[]? | "\(.package_attr_name)\t\(.package_description // "no description")""#,
    );

    let output = Command::new("fzf")
        .args([
            "--disabled",
            "--ansi",
            "--prompt=search> ",
            "--delimiter=\t",
            "--with-nth=1",
            "--preview=echo {2}",
            "--preview-window=right:50%:wrap",
            "--bind=change:reload:nh search {q} --json 2>/dev/null | jq -r -f /tmp/next_search_filter.jq || true",
            "--header=type to search nixpkgs live",
        ])
        .output()
        .ok()?;

    let line = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if line.is_empty() {
        return None;
    }
    let pkg = line.split('\t').next()?.to_string();

    if Config::load().app.show_description {
        let escaped = pkg.replace('\'', "'\\''");
        let cmd = format!(
            "nh search '{escaped}' --json 2>/dev/null | jq -r --arg name '{escaped}' '.results[] | select(.package_attr_name == $name) | \"\\(.package_pname) (\\(.package_pversion))\\n\\(.package_description // \"no description\")\\n\\(.package_homepage[0] // \"\")\"'"
        );
        let desc = capture(&cmd);
        if !desc.trim().is_empty() {
            println!("{desc}");
        }
    }

    Some(pkg)
}

pub fn install(package: Option<&str>) -> Option<String> {
    let pkg = match package {
        Some(p) => p.to_string(),
        None => {
            let p = search_nixpkgs()?;
            if !confirm(&format!("Install {p}?")) {
                return None;
            }
            p
        }
    };

    if let Err(e) = nix_parser::insert(&pkg) {
        eprintln!("Failed to update apps file: {e}");
        return None;
    }
    os::switch();
    Some(format!("Installed: {pkg}"))
}

pub fn remove(package: Option<&str>) -> Option<String> {
    let pkg = match package {
        Some(p) => p.to_string(),
        None => {
            let installed = match nix_parser::list() {
                Ok(list) if !list.is_empty() => list,
                _ => {
                    eprintln!("No installed packages found");
                    return None;
                }
            };

            let mut child = Command::new("fzf")
                .args(["--prompt=select> ", "--header=pick a package"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .ok()?;

            child.stdin.take()?.write_all(installed.join("\n").as_bytes()).ok()?;
            let output = child.wait_with_output().ok()?;
            let p = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if p.is_empty() {
                return None;
            }
            if !confirm(&format!("Remove {p}?")) {
                return None;
            }
            p
        }
    };

    match nix_parser::remove(&pkg) {
        Ok(true) => {
            os::switch();
            Some(format!("Removed: {pkg}"))
        }
        Ok(false) => {
            eprintln!("App not found");
            None
        }
        Err(e) => {
            eprintln!("Failed to remove app: {e}");
            None
        }
    }
}

pub fn r#try(package: Option<&str>) -> bool {
    let pkg = match package {
        Some(p) => p.to_string(),
        None => match search_nixpkgs() {
            Some(p) => p,
            None => return false,
        },
    };

    run(&format!("nix-shell -p '{}'", pkg.replace('\'', "'\\''")))
}

pub fn list() {
    match nix_parser::list() {
        Ok(entries) if !entries.is_empty() => {
            for e in entries {
                println!("{e}");
            }
        }
        _ => eprintln!("No apps found"),
    }
}
