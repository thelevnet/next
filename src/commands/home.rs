use crate::messages::*;
use crate::config::*;
use crate::shell::{run, confirm};

pub fn switch() -> bool { run(HOME_SWITCH) }

pub fn conf() -> Option<String> {
    run(HOME_CONFIG);
    if confirm("Commit and push?") {
        Some(HOME_COMMIT_MSG.to_string())
    } else {
        None
    }
}
