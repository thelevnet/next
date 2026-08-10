use crate::messages::*;
use crate::config::APPS_PATH;
use crate::nix_parser;
use crate::commands::os;
use crate::commands::search;
use crate::shell::{run, confirm};

fn pick_and_describe() -> Option<String> {
    let pkg = search::search()?;
    if let Some(desc) = search::describe(&pkg) {
        println!("{desc}");
    }
    Some(pkg)
}

pub fn install(pkg: &str) -> bool {
    if let Err(e) = nix_parser::insert_entry(APPS_PATH, pkg) {
        eprintln!("{PREFIX} {ERR_UPDATE_APPS_FILE}: {e}");
        return false;
    }
    os::switch()
}

pub fn install_interactive() -> bool {
    match pick_and_describe() {
        Some(pkg) => {
            if confirm(&format!("{PREFIX} {PROMPT_INSTALL} {pkg}?")) {
                install(&pkg)
            } else {
                true
            }
        }
        None => { println!("{PREFIX} {MSG_NO_SELECTION}"); true }
    }
}

pub fn remove(pkg: &str) -> bool {
    match nix_parser::remove_entry(APPS_PATH, pkg) {
        Ok(true) => os::switch(),
        Ok(false) => { eprintln!("{PREFIX} {ERR_APP_NOT_FOUND}"); false }
        Err(e) => { eprintln!("{PREFIX} {ERR_REMOVE_APP}: {e}"); false }
    }
}

pub fn remove_interactive() -> bool {
    let installed = match nix_parser::list_entries(APPS_PATH) {
        Ok(list) if !list.is_empty() => list,
        _ => { eprintln!("{PREFIX} {ERR_NO_INSTALLED_PACKAGES}"); return false; }
    };

    match search::search_over(&installed) {
        Some(pkg) => {
            if confirm(&format!("{PREFIX} {PROMPT_REMOVE} {pkg}?")) {
                remove(&pkg)
            } else {
                true
            }
        }
        None => { println!("{PREFIX} {MSG_NO_SELECTION}"); true }
    }
}

pub fn temp(pkg: &str) -> bool {
    run(&format!("nix-shell -p {}", search::shell_escape(pkg)))
}

pub fn temp_interactive() -> bool {
    match pick_and_describe() {
        Some(pkg) => temp(&pkg),
        None => { println!("{PREFIX} {MSG_NO_SELECTION}"); true }
    }
}

pub fn list() {
    match nix_parser::list_entries(APPS_PATH) {
        Ok(entries) if !entries.is_empty() => {
            for e in entries { println!("{e}"); }
        }
        _ => eprintln!("{PREFIX} {ERR_NO_APPS_FOUND}"),
    }
}
