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

// Убираем импорт git отсюда, он больше не нужен
pub fn install(pkg: &str) -> Option<String> {
    if let Err(e) = nix_parser::insert_entry(APPS_PATH, pkg) {
        eprintln!("{ERR_UPDATE_APPS_FILE}: {e}");
        return None;
    }
    os::switch(None);
    Some(format!("{APP_INSTALLED} {pkg}"))
}

pub fn install_interactive() -> Option<String> {
    match pick_and_describe() {
        Some(pkg) => {
            if confirm(&format!("{PROMPT_INSTALL} {pkg}?")) {
                install(&pkg)
            } else {
                None
            }
        }
        None => { println!("{PREFIX} {MSG_NO_SELECTION}"); None }
    }
}

pub fn remove(pkg: &str) -> Option<String> {
    match nix_parser::remove_entry(APPS_PATH, pkg) {
        Ok(true) => {
            os::switch(None);
            Some(format!("{APP_REMOVED} {pkg}"))
        },
        Ok(false) => { eprintln!("{PREFIX} {ERR_APP_NOT_FOUND}"); None }
        Err(e) => { eprintln!("{PREFIX} {ERR_REMOVE_APP}: {e}"); None }
    }
}

pub fn remove_interactive() -> Option<String> {
    let installed = match nix_parser::list_entries(APPS_PATH) {
        Ok(list) if !list.is_empty() => list,
        _ => { eprintln!("{PREFIX} {ERR_NO_INSTALLED_PACKAGES}"); return None; }
    };

    match search::search_over(&installed) {
        Some(pkg) => {
            if confirm(&format!("{PREFIX} {PROMPT_REMOVE} {pkg}?")) {
                remove(&pkg)
            } else {
                None
            }
        }
        None => { println!("{PREFIX} {MSG_NO_SELECTION}"); None }
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
