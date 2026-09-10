use crate::config::{Config, CLEAN_CMD, GENS_LIST, REPO_PATH, ROLLBACK_CMD};
use crate::shell::run;

/// Build and switch to the NixOS system configuration
pub fn switch() -> bool {
    let config = Config::load();
    let target = &config.general.hostname;
    run(&format!("nh os switch {REPO_PATH} --hostname {target}"))
}

/// Build and set configuration for the next boot without switching live
pub fn boot() -> bool {
    let config = Config::load();
    let target = &config.general.hostname;
    run(&format!("nh os boot {REPO_PATH} --hostname {target}"))
}

/// Dry-build the NixOS system configuration without switching
pub fn dry() -> bool {
    let config = Config::load();
    let target = &config.general.hostname;
    run(&format!("nh os build {REPO_PATH} --hostname {target}"))
}

/// Clean older generations and collect garbage
pub fn clean() -> bool {
    run(CLEAN_CMD)
}

/// Roll back to the previous NixOS generation
pub fn rollback() -> bool {
    run(ROLLBACK_CMD)
}

/// List all NixOS generations
pub fn list() -> bool {
    run(GENS_LIST)
}

