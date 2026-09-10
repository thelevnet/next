use std::env;
use std::path::Path;
use std::process::Command;
use crate::config::REPO_PATH;

const INLINE_DEV_SHELL: &str = r#"
let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    name = "nixos-config-dev";
    packages = with pkgs; [
      sops
    ];
  }
"#;

/// Enter the NixOS configuration directory (/etc/nixos) with a nix develop shell
/// containing all essential tools for editing and managing NixOS configurations.
pub fn enter() -> bool {
    let repo = REPO_PATH;
    if !Path::new(repo).exists() {
        eprintln!("Error: NixOS configuration directory '{repo}' does not exist.");
        return false;
    }

    let shell = env::var("SHELL").unwrap_or_else(|_| "bash".to_string());

    // 1. Try entering via the repository's flake devShell
    let status = Command::new("nix")
        .args(["develop", "-c", &shell])
        .current_dir(repo)
        .status();

    if let Ok(s) = status {
        if s.success() {
            return true;
        }
    }

    // 2. Fallback: run nix develop with inline tools expression
    let fallback = Command::new("nix")
        .args(["develop", "--impure", "--expr", INLINE_DEV_SHELL, "-c", &shell])
        .current_dir(repo)
        .status();

    match fallback {
        Ok(s) => s.success(),
        Err(e) => {
            eprintln!("Failed to launch nix develop shell: {e}");
            false
        }
    }
}
