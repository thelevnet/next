use crate::messages::*;
use crate::config::*;
use crate::shell::{run, confirm};

fn get_target(host: Option<&str>) -> &str {
    host.unwrap_or("desktop")
}

pub fn switch(host: Option<&str>) -> bool {
    let target = get_target(host);
    run(&format!("nh os switch {REPO_PATH} --hostname {target}"))
}

pub fn boot(host: Option<&str>) -> bool {
    let target = get_target(host);
    run(&format!("nh os boot {REPO_PATH} --hostname {target}"))
}

pub fn dry(host: Option<&str>) -> bool {
    let target = get_target(host);
    run(&format!("nh os build {REPO_PATH} --hostname {target}"))
}

pub fn conf() -> Option<String> {
    run(OS_CONFIG);
    if confirm("Switch now?") {
        switch(None);
        Some(OS_COMMIT_MSG.to_string())
    } else {
        None
    }
}
