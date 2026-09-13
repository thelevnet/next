use crate::shell::run;

pub fn enter() -> bool {
    run("cd /etc/nixos && exec nix develop -c \"${SHELL:-bash}\"")
}
