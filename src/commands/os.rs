use crate::shell::{hostname, run, REPO_PATH};

pub const CLEAN_CMD: &str = "sudo nix-env --delete-generations +2 --profile /nix/var/nix/profiles/system && sudo nix-collect-garbage -d && sudo nix-store --optimise && sudo nix-collect-garbage --delete-old && sudo journalctl --vacuum-time=3d && sudo rm -rf /tmp/* && nix-env --delete-generations +2 && home-manager expire-generations '-1 days' && sudo nix-store --gc";

pub fn switch() -> bool {
    let target = hostname();
    run(&format!("nh os switch {REPO_PATH} --hostname {target}"))
}

pub fn dry() -> bool {
    let target = hostname();
    run(&format!("nh os build {REPO_PATH} --hostname {target}"))
}

pub fn clean() -> bool {
    run(CLEAN_CMD)
}

