use crate::shell::{run, username, REPO_PATH};

pub fn switch() -> bool {
    let target = username();
    run(&format!("nh home switch {REPO_PATH} -c {target}"))
}



