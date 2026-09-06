pub const APPS_PATH: &str = "/etc/nixos/users/lev/packages.nix";
pub const DOTS_PATH: &str = "/etc/nixos/modules/home";
pub const ROLLBACK_CMD: &str = "nh os rollback";
pub const CLEAN_CMD: &str = "sudo nix-env --delete-generations +2 --profile /nix/var/nix/profiles/system && sudo nix-collect-garbage -d && sudo nix-store --optimise && sudo nix-collect-garbage --delete-old && sudo journalctl --vacuum-time=3d && sudo rm -rf /tmp/* && nix-env --delete-generations +2 && home-manager expire-generations '-1 days' && sudo nix-store --gc";
pub const EDITOR: &str = "nvim";
pub const REPO_PATH: &str = "/etc/nixos";

pub const OS_CONFIG: &str = "nvim /etc/nixos/hosts/desktop/default.nix";
pub const HOME_CONFIG: &str = "nvim /etc/nixos/users/lev/default.nix";

pub const GENS_LIST: &str = "nixos-rebuild list-generations";
