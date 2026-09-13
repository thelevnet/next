use crate::shell::{run, REPO_PATH};

pub fn enter() -> bool {
    run(&format!("cd {REPO_PATH} && exec nix develop -c \"${{SHELL:-bash}}\""))
}

