use crate::config::{Config, CLEAN_CMD, REPO_PATH};
use crate::shell::run;

pub fn switch() -> bool {
    let config = Config::load();
    let target = &config.general.hostname;
    run(&format!("nh os switch {REPO_PATH} --hostname {target}"))
}

pub fn dry() -> bool {
    let config = Config::load();
    let target = &config.general.hostname;
    run(&format!("nh os build {REPO_PATH} --hostname {target}"))
}

pub fn clean() -> bool {
    run(CLEAN_CMD)
}

