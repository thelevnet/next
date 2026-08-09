use crate::config::*;
use crate::shell::run;

pub fn switch() -> bool {
    run(HOME_SWITCH)
}

pub fn conf() -> bool {
    run(HOME_CONFIG)
}
