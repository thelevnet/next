use crate::config::{Config, REPO_PATH};
use crate::shell::run;

pub fn switch() -> bool {
    let config = Config::load();
    let target = &config.general.user;
    run(&format!("nh home switch {REPO_PATH} -c {target}"))
}



