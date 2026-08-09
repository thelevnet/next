use crate::config::REPO_PATH;
use crate::shell::run;

pub fn check() -> bool {
    println!("checking flake syntax...");
    let flake_ok = run(&format!("nix flake check {REPO_PATH}"));

    println!("checking git status...");
    let git_clean = run(&format!("git -C {REPO_PATH} diff --quiet"));

    if flake_ok && git_clean {
        println!("all clear.");
        true
    } else {
        if !flake_ok { eprintln!("flake check failed."); }
        if !git_clean { eprintln!("uncommitted changes present."); }
        false
    }
}
