use crate::messages::*;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

const START_MARKER: &str = "#next start";
const END_MARKER: &str = "#next end";
const DEPENDENCIES_MARKER: &str = "#next dependencies";

pub fn insert_entry(path: &str, entry: &str) -> io::Result<()> {
    backup(path)?;
    let reader = BufReader::new(File::open(path)?);
    let mut lines = Vec::new();
    let mut found = false;

    for line in reader.lines() {
        let line = line?;
        let is_marker = line.trim() == START_MARKER;
        lines.push(line);
        if is_marker {
            lines.push(format!("\t\t{entry}"));
            found = true;
        }
    }

    if !found {
        eprintln!("{PREFIX} {ERR_MARKER_NOT_FOUND} {path}: \"{START_MARKER}\"");
        return Ok(());
    }

    fs::write(path, lines.join("\n") + "\n")
}

pub fn remove_entry(path: &str, entry: &str) -> io::Result<bool> {
    backup(path)?;
    let reader = BufReader::new(File::open(path)?);
    let mut lines = Vec::new();
    let mut in_section = false;
    let mut found = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        match trimmed {
            _ if trimmed == START_MARKER => { in_section = true; lines.push(line); continue; }
            _ if trimmed == END_MARKER => { in_section = false; lines.push(line); continue; }
            _ if in_section && trimmed == entry => { found = true; continue; }
            _ => lines.push(line),
        }
    }

    fs::write(path, lines.join("\n") + "\n")?;
    Ok(found)
}

pub fn list_entries(path: &str) -> io::Result<Vec<String>> {
    let reader = BufReader::new(File::open(path)?);
    let mut entries = Vec::new();
    let mut in_section = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed == START_MARKER { in_section = true; continue; }
        if trimmed == END_MARKER { break; }
        if in_section && !trimmed.is_empty() && trimmed != DEPENDENCIES_MARKER {
            entries.push(trimmed.to_string());
        }
    }
    Ok(entries)
}

fn backup(path: &str) -> io::Result<()> {
    let file_name = std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("config.nix");
    let backup_path = format!("/tmp/{file_name}.bak");
    fs::copy(path, backup_path)?;
    Ok(())
}
