use crate::messages::*;
use crate::config::{DOTS_PATH, EDITOR};
use crate::shell::{run, confirm};
use crate::commands::home;
use std::path::Path;
use std::fs;

pub fn edit(module: Option<&str>) {
    let path = match module {
        Some(m) => format!("{DOTS_PATH}/{m}"),
        None => {
            list_modules();
            return;
        }
    };

    if !Path::new(&path).exists() {
        eprintln!("{PREFIX} {ERR_MODULE_NOT_FOUND} {DOTS_PATH}: \"{}\"", module.unwrap());
        return;
    }

    run(&format!("{EDITOR} {path}"));

    if confirm(&format!("{PREFIX} {PROMPT_HOME_SWITCH}")) {
        home::switch();
    }
}

fn list_modules() {
    println!("{PREFIX} {MSG_AVAILABLE_DOTS}:");
    if let Ok(entries) = fs::read_dir(DOTS_PATH) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    }
}
