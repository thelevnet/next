use crate::config::*;
use crate::shell::run;

pub fn clean() -> bool {
    run(CLEAN_CMD)
}

pub fn rollback() -> bool {
    run(ROLLBACK_CMD)
}

pub fn list() -> bool {
    run("nixos-rebuild list-generations")
}
