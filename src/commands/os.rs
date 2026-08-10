use crate::messages::*;
use crate::config::*;
use crate::shell::{run, confirm};

pub fn switch() -> bool { run(OS_SWITCH) }
pub fn boot() -> bool { run(OS_BOOT) }
pub fn dry() -> bool { run(OS_DRY) }

pub fn conf() -> Option<String> {
    run(OS_CONFIG);
    if confirm("Switch now?") {
        switch();
        Some(OS_COMMIT_MSG.to_string())
    } else {
        None
    }
}
