use crate::messages::*;
use crate::config::*;
use crate::shell::{run, confirm};
use crate::commands::git;

pub fn switch() -> bool {
    run(HOME_SWITCH);
    git::commit_push(format!("{HOME_COMMIT_MSG}"));
    true
}

pub fn conf() -> bool {
    run(HOME_CONFIG);
    if confirm("Commit and push?") {
        git::commit_push(format!("{HOME_COMMIT_MSG}"));
    };
    true
}
