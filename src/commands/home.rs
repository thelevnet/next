use crate::config::{Config, REPO_PATH};
use crate::shell::{confirm, run};

/// Build and switch to Home Manager configuration for the configured user
pub fn switch() -> bool {
    let config = Config::load();
    let target = &config.general.user;
    run(&format!("nh home switch {REPO_PATH} -c {target}"))
}

/// Edit user home configuration with editor and optionally prompt to commit
#[allow(dead_code)]
pub fn conf() -> Option<String> {
    let config = Config::load();
    run(&config.home_config());
    if confirm("Commit and push?") {
        Some(format!("Update to {REPO_PATH}/users/{}", config.general.user))
    } else {
        None
    }
}



