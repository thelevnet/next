pub const APPS_PATH: &str = "/etc/nixos/apps.nix";
pub const DOTS_PATH: &str = "/etc/nixos/dots";
pub const ROLLBACK_CMD: &str = "nh os rollback";
pub const CLEAN_CMD: &str = "sudo nix-env --delete-generations +2 --profile /nix/var/nix/profiles/system && sudo nix-collect-garbage -d && sudo nix-store --optimise && sudo nix-collect-garbage --delete-old && sudo journalctl --vacuum-time=3d && sudo rm -rf /tmp/* && nix-env --delete-generations +2 && home-manager expire-generations '-1 days' && sudo nix-store --gc";
pub const EDITOR: &str = "nvim";
pub const REPO_PATH: &str = "/etc/nixos";

pub const OS_SWITCH: &str = "nh os switch /etc/nixos --hostname nix";
pub const OS_BOOT: &str = "nh os boot /etc/nixos --hostname nix";
pub const OS_DRY: &str = "nh os build /etc/nixos --hostname nix";
pub const OS_CONFIG: &str = "nvim /etc/nixos/configuration.nix";

pub const HOME_SWITCH: &str = "nh home switch /etc/nixos -c lev";
pub const HOME_CONFIG: &str = "nvim /etc/nixos/configuration.nix";

pub const GENS_LIST: &str = "nixos-rebuild list-generations";

