use crate::messages::*;
use crate::config::{DOTS_PATH, EDITOR};
use crate::shell::{run, confirm};
use crate::commands::home;
use std::path::Path;
use std::fs;

fn find_module(m: &str) -> Option<String> {
    let direct = format!("{DOTS_PATH}/{m}");
    if Path::new(&direct).exists() {
        return Some(direct);
    }
    let direct_nix = format!("{DOTS_PATH}/{m}.nix");
    if Path::new(&direct_nix).exists() {
        return Some(direct_nix);
    }

    if let Ok(categories) = fs::read_dir(DOTS_PATH) {
        for cat in categories.flatten() {
            let cat_path = cat.path();
            if cat_path.is_dir() {
                let candidate = cat_path.join(m);
                if candidate.exists() {
                    return Some(candidate.to_string_lossy().to_string());
                }
                let candidate_nix = cat_path.join(format!("{m}.nix"));
                if candidate_nix.exists() {
                    return Some(candidate_nix.to_string_lossy().to_string());
                }
            }
        }
    }

    None
}

pub fn edit(module: Option<&str>) -> Option<String> {
    let path = match module {
        Some(m) => match find_module(m) {
            Some(p) => p,
            None => {
                eprintln!("{PREFIX} {ERR_MODULE_NOT_FOUND} {DOTS_PATH}: \"{m}\"");
                return None;
            }
        },
        None => {
            list_modules();
            return None;
        }
    };

    run(&format!("{EDITOR} {path}"));

    if confirm(&format!("{PREFIX} {PROMPT_HOME_SWITCH}")) {
        home::switch();
        Some(format!("Update dots: {}", module.unwrap_or("module")))
    } else {
        None
    }
}

fn list_modules() {
    println!("{PREFIX} {MSG_AVAILABLE_DOTS}:");
    if let Ok(categories) = fs::read_dir(DOTS_PATH) {
        for cat in categories.flatten() {
            let cat_path = cat.path();
            let cat_name = cat.file_name().to_string_lossy().to_string();
            if cat_path.is_dir() {
                if let Ok(files) = fs::read_dir(&cat_path) {
                    for file in files.flatten() {
                        let f_name = file.file_name().to_string_lossy().to_string();
                        if f_name.ends_with(".nix") {
                            println!("  {cat_name}/{f_name}");
                        }
                    }
                }
            } else if cat_name.ends_with(".nix") {
                println!("  {cat_name}");
            }
        }
    }
}
