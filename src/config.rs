use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const CLEAN_CMD: &str = "sudo nix-env --delete-generations +2 --profile /nix/var/nix/profiles/system && sudo nix-collect-garbage -d && sudo nix-store --optimise && sudo nix-collect-garbage --delete-old && sudo journalctl --vacuum-time=3d && sudo rm -rf /tmp/* && nix-env --delete-generations +2 && home-manager expire-generations '-1 days' && sudo nix-store --gc";
pub const REPO_PATH: &str = "/etc/nixos";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub git: GitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    #[serde(default = "default_user")]
    pub user: String,
    #[serde(default = "default_hostname")]
    pub hostname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
}

fn default_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| "lev".to_string())
}

fn default_hostname() -> String {
    "desktop".to_string()
}

const fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            git: GitConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            user: default_user(),
            hostname: default_hostname(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            enable: default_true(),
        }
    }
}

impl Config {
    /// Parse TOML string directly into a Config struct
    pub fn parse(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    /// Load config from a file path
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config = Self::parse(&content)?;
        Ok(config)
    }

    /// Standard config file path (~/.config/next/config.toml)
    pub fn default_path() -> Option<PathBuf> {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                return Some(PathBuf::from(xdg).join("next").join("config.toml"));
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            return Some(PathBuf::from(home).join(".config").join("next").join("config.toml"));
        }
        None
    }

    /// Load config from default path, falling back to Default::default() if file doesn't exist
    pub fn load() -> Self {
        if let Some(path) = Self::default_path() {
            if path.exists() {
                if let Ok(cfg) = Self::load_from_path(&path) {
                    return cfg;
                }
            }
        }
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example_config() {
        let toml_content = r#"
[general]
user = "lev"
hostname = "desktop"
confirm_action = "prompt" 
[git]
enable = true
"#;

        let config = Config::parse(toml_content).expect("Failed to parse config");
        assert_eq!(config.general.user, "lev");
        assert_eq!(config.general.hostname, "desktop");
        assert!(config.git.enable);
    }
}

