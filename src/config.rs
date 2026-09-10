use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const ROLLBACK_CMD: &str = "nh os rollback";
pub const CLEAN_CMD: &str = "sudo nix-env --delete-generations +2 --profile /nix/var/nix/profiles/system && sudo nix-collect-garbage -d && sudo nix-store --optimise && sudo nix-collect-garbage --delete-old && sudo journalctl --vacuum-time=3d && sudo rm -rf /tmp/* && nix-env --delete-generations +2 && home-manager expire-generations '-1 days' && sudo nix-store --gc";
pub const REPO_PATH: &str = "/etc/nixos";
pub const GENS_LIST: &str = "nixos-rebuild list-generations";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub app: AppConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    #[serde(default = "default_user")]
    pub user: String,
    #[serde(default = "default_hostname")]
    pub hostname: String,
    #[serde(default = "default_confirm_action")]
    pub confirm_action: String,
    #[serde(default = "default_editor")]
    pub editor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitConfig {
    #[serde(default = "default_true")]
    pub enable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    #[serde(default = "default_true")]
    pub show_description: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UiConfig {
    #[serde(default = "default_true")]
    pub enable_color: bool,
    #[serde(default = "default_true")]
    pub enable_nerd_fonts: bool,
}

fn default_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| "lev".to_string())
}

fn default_hostname() -> String {
    "desktop".to_string()
}

fn default_confirm_action() -> String {
    "prompt".to_string()
}

fn default_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string())
}

const fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            git: GitConfig::default(),
            app: AppConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            user: default_user(),
            hostname: default_hostname(),
            confirm_action: default_confirm_action(),
            editor: default_editor(),
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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            show_description: default_true(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            enable_color: default_true(),
            enable_nerd_fonts: default_true(),
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

    /// Load or create default config file if it doesn't exist
    #[allow(dead_code)]
    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::default_path()
            .ok_or_else(|| "Could not determine user config directory ($HOME not set)")?;

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let default_cfg = Self::default();
            let toml_str = toml::to_string_pretty(&default_cfg)?;
            fs::write(&path, toml_str)?;
            return Ok(default_cfg);
        }

        Self::load_from_path(path)
    }

    /// Dynamic path to apps / packages.nix based on configured user
    pub fn apps_path(&self) -> String {
        format!("/etc/nixos/users/{}/packages.nix", self.general.user)
    }

    /// Dynamic path to user home config based on configured user
    #[allow(dead_code)]
    pub fn home_config(&self) -> String {
        format!("nvim /etc/nixos/users/{}/default.nix", self.general.user)
    }

    /// Dynamic path to os host config based on configured hostname
    #[allow(dead_code)]
    pub fn os_config(&self) -> String {
        format!("nvim /etc/nixos/hosts/{}/default.nix", self.general.hostname)
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
[app]
show_description = true
[ui]
enable_color = true
enable_nerd_fonts = true
"#;

        let config = Config::parse(toml_content).expect("Failed to parse config");
        assert_eq!(config.general.user, "lev");
        assert_eq!(config.general.hostname, "desktop");
        assert_eq!(config.general.confirm_action, "prompt");
        assert!(config.git.enable);
        assert!(config.app.show_description);
        assert!(config.ui.enable_color);
        assert!(config.ui.enable_nerd_fonts);
    }
}

