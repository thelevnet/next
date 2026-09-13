use crate::shell::run;

pub const CLEAN_CMD: &str = "sudo nix-env --delete-generations +2 --profile /nix/var/nix/profiles/system && sudo nix-collect-garbage -d && sudo nix-store --optimise && sudo nix-collect-garbage --delete-old && sudo journalctl --vacuum-time=3d && sudo rm -rf /tmp/* && nix-env --delete-generations +2 && home-manager expire-generations '-1 days' && sudo nix-store --gc";

pub fn switch() -> bool {
    run("nh os switch")
}

pub fn dry() -> bool {
    run("nh os build")
}

pub fn clean() -> bool {
    run(CLEAN_CMD)
}

