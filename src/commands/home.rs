use crate::messages::*;
use crate::config::*;
use crate::shell::{run, confirm};

fn get_config(user: Option<&str>) -> &str {
    user.unwrap_or("lev")
}

pub fn switch(user: Option<&str>) -> bool {
    let target = get_config(user);
    run(&format!("nh home switch {REPO_PATH} -c {target}"))
}

pub fn conf() -> Option<String> {
    run(HOME_CONFIG);
    if confirm("Commit and push?") {
        Some(HOME_COMMIT_MSG.to_string())
    } else {
        None
    }
}
