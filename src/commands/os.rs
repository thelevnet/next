use crate::messages::*;
use crate::commands::git;
use crate::config::*;
use crate::shell::{run, confirm};

pub fn switch() -> bool {
    run(OS_SWITCH);
    git::commit_push(format!("{OS_COMMIT_MSG}"));
    true
}

pub fn boot() -> bool {
    run(OS_BOOT);
    git::commit_push(format!("{OS_COMMIT_MSG}"));
    true
}

pub fn dry() -> bool {
    run(OS_DRY);
    git::commit_push(format!("{OS_COMMIT_MSG}"));
    true
}
pub fn conf() -> bool {
    run(OS_CONFIG);
    if confirm("Switch now?") {
        switch();
        git::commit_push(format!("{OS_COMMIT_MSG}"));
    }; 
    true
}
