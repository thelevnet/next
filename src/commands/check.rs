use crate::messages::*;
use crate::config::REPO_PATH;
use crate::shell::run;

pub fn check() -> bool {
    println!("{PREFIX} {MSG_CHECKING_FLAKE}");
    let flake_ok = run(&format!("nix flake check {REPO_PATH}"));

    println!("{PREFIX} {MSG_CHECKING_GIT}");
    let git_clean = run(&format!("git -C {REPO_PATH} diff --quiet"));

    if flake_ok && git_clean {
        println!("{PREFIX} {MSG_ALL_CLEAR}");
        true
    } else {
        if !flake_ok { eprintln!("{PREFIX} {ERR_FLAKE_CHECK_FAILED}"); }
        if !git_clean { eprintln!("{PREFIX} {ERR_GIT_DIRTY}"); }
        false
    }
}
