use crate::config::*;
use crate::shell::run;

pub fn switch() -> bool {
    run(OS_SWITCH)
}

pub fn boot() -> bool {
    run(OS_BOOT)
}

pub fn dry() -> bool {
    run(OS_DRY)
}
pub fn conf() -> bool {
    run(OS_CONFIG)
}
