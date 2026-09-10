use rnix::{ast, Root, SyntaxKind};
use rowan::ast::AstNode;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum NixError {
    Io(io::Error),
    SyntaxError(String),
    Custom(String),
}

impl fmt::Display for NixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NixError::Io(e) => write!(f, "I/O error: {e}"),
            NixError::SyntaxError(msg) => write!(f, "Nix syntax error: {msg}"),
            NixError::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for NixError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NixError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for NixError {
    fn from(err: io::Error) -> Self {
        NixError::Io(err)
    }
}

impl From<NixError> for io::Error {
    fn from(err: NixError) -> Self {
        match err {
            NixError::Io(e) => e,
            other => io::Error::new(io::ErrorKind::Other, other.to_string()),
        }
    }
}

/// AST-driven Nix file manager operating without comment markers
#[derive(Debug, Clone)]
pub struct NixFile {
    pub path: PathBuf,
    pub content: String,
}

impl NixFile {
    /// Read a Nix file from disk
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, NixError> {
        let path_buf = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path_buf)?;
        Ok(Self {
            path: path_buf,
            content,
        })
    }

    /// Construct from an in-memory string (useful for testing and manipulation)
    #[allow(dead_code)]
    pub fn from_str<P: AsRef<Path>>(path: P, content: impl Into<String>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            content: content.into(),
        }
    }

    // ==========================================
    // AST-BASED PACKAGE MANIPULATION (No markers needed)
    // ==========================================

    /// Find the package list AST node (e.g. in `home.packages` or `environment.systemPackages`).
    pub fn find_package_list(&self) -> Result<ast::List, NixError> {
        let parsed = Root::parse(&self.content);
        let root = parsed.syntax();

        for node in root.descendants() {
            if node.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
                if let Some(attr_val) = ast::AttrpathValue::cast(node) {
                    if let Some(attrpath) = attr_val.attrpath() {
                        let path_str = attrpath.to_string().replace(' ', "");
                        if path_str.ends_with("packages") || path_str.ends_with("systemPackages") {
                            if let Some(val) = attr_val.value() {
                                if let Some(list) = Self::extract_list_from_expr(&val) {
                                    return Ok(list);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Fallback: look for ANY list node
        for node in root.descendants() {
            if node.kind() == SyntaxKind::NODE_LIST {
                if let Some(list) = ast::List::cast(node) {
                    return Ok(list);
                }
            }
        }

        Err(NixError::Custom("Could not find a package list (e.g. home.packages or environment.systemPackages)".into()))
    }

    fn extract_list_from_expr(expr: &ast::Expr) -> Option<ast::List> {
        match expr {
            ast::Expr::With(w) => {
                let body = w.body()?;
                Self::extract_list_from_expr(&body)
            }
            ast::Expr::List(list) => Some(list.clone()),
            _ => None,
        }
    }

    /// List all packages directly from the Nix AST without requiring markers
    pub fn list_packages(&self) -> Result<Vec<String>, NixError> {
        let list = self.find_package_list()?;
        let mut packages = Vec::new();
        for item in list.items() {
            let s = item.to_string().trim().to_string();
            let clean = s.lines().next().unwrap_or("").trim().to_string();
            if !clean.is_empty() && !clean.starts_with('#') {
                packages.push(clean);
            }
        }
        Ok(packages)
    }

    /// Check if a package exists in the AST list
    pub fn contains_package(&self, pkg: &str) -> Result<bool, NixError> {
        let pkgs = self.list_packages()?;
        Ok(pkgs.iter().any(|p| p == pkg || p.split('.').last() == Some(pkg)))
    }

    /// Insert a package directly into the AST list without requiring markers
    pub fn insert_package(&mut self, pkg: &str) -> Result<bool, NixError> {
        let trimmed = pkg.trim();
        if trimmed.is_empty() || self.contains_package(trimmed)? {
            return Ok(false);
        }

        let list = self.find_package_list()?;
        let list_syntax = list.syntax();

        // Find closing bracket `]`
        let r_bracket = list_syntax
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find(|t| t.kind() == SyntaxKind::TOKEN_R_BRACK)
            .ok_or_else(|| NixError::Custom("Malformed Nix list: missing ']'".into()))?;

        let insert_offset: usize = r_bracket.text_range().start().into();

        // Determine indentation from existing items
        let indent = self.detect_list_indentation(&list, &r_bracket);

        let insertion = format!("{indent}{trimmed}\n");
        self.content.insert_str(insert_offset, &insertion);

        Ok(true)
    }

    /// Remove a package directly from the AST list without requiring markers
    pub fn remove_package(&mut self, pkg: &str) -> Result<bool, NixError> {
        let trimmed = pkg.trim();
        let list = self.find_package_list()?;

        for item in list.items() {
            let item_str = item.to_string().trim().to_string();
            if item_str == trimmed || item_str.split('.').last() == Some(trimmed) {
                let range = item.syntax().text_range();
                let start: usize = range.start().into();
                let end: usize = range.end().into();

                let line_start = self.content[..start]
                    .rfind('\n')
                    .map(|i| i + 1)
                    .unwrap_or(start);

                let line_end = self.content[end..]
                    .find('\n')
                    .map(|i| end + i + 1)
                    .unwrap_or(end);

                self.content.drain(line_start..line_end);
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn detect_list_indentation(
        &self,
        list: &ast::List,
        r_bracket: &rowan::SyntaxToken<rnix::NixLanguage>,
    ) -> String {
        for item in list.items() {
            let offset: usize = item.syntax().text_range().start().into();
            let line_start = self.content[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let ws: String = self.content[line_start..offset]
                .chars()
                .take_while(|c| c.is_whitespace() && *c != '\n')
                .collect();
            if !ws.is_empty() {
                return ws;
            }
        }

        let offset: usize = r_bracket.text_range().start().into();
        let line_start = self.content[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let ws: String = self.content[line_start..offset]
            .chars()
            .take_while(|c| c.is_whitespace() && *c != '\n')
            .collect();
        if ws.contains('\t') {
            format!("{ws}\t")
        } else {
            format!("{ws}    ")
        }
    }

    // ==========================================
    // PERSISTENCE & SAFE BACKUPS
    // ==========================================

    /// Create a backup in `/tmp/next_backups/` and `/tmp/{file_name}.bak`
    pub fn create_backup(&self) -> Result<PathBuf, NixError> {
        let file_name = self
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("config.nix");

        let compat_backup = PathBuf::from(format!("/tmp/{file_name}.bak"));
        if self.path.exists() {
            fs::copy(&self.path, &compat_backup)?;
        }

        let backup_dir = PathBuf::from("/tmp/next_backups");
        if !backup_dir.exists() {
            let _ = fs::create_dir_all(&backup_dir);
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let timestamped_path = backup_dir.join(format!("{file_name}_{timestamp}.bak"));

        if self.path.exists() {
            let _ = fs::copy(&self.path, &timestamped_path);
        }

        Ok(compat_backup)
    }

    /// Save the current content to disk after creating a backup
    pub fn save(&self) -> Result<(), NixError> {
        self.create_backup()?;

        if let Some(parent) = self.path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(&self.path, &self.content)?;
        Ok(())
    }
}

// =========================================================================
// NO-PATH PUBLIC API (Uses Config::load().apps_path() automatically)
// =========================================================================

/// Default path resolved from configuration
pub fn default_path() -> PathBuf {
    PathBuf::from(crate::config::Config::load().apps_path())
}

/// List packages from default configured Nix package file without markers
pub fn list() -> Result<Vec<String>, NixError> {
    list_from(&default_path())
}

/// Insert a package into default configured Nix file without markers
pub fn insert(pkg: &str) -> Result<bool, NixError> {
    insert_into(&default_path(), pkg)
}

/// Remove a package from default configured Nix file without markers
pub fn remove(pkg: &str) -> Result<bool, NixError> {
    remove_from(&default_path(), pkg)
}

// =========================================================================
// EXPLICIT PATH AST API (Pure AST, no markers needed)
// =========================================================================

/// List packages from specified path directly via AST
pub fn list_from<P: AsRef<Path>>(path: P) -> Result<Vec<String>, NixError> {
    let file = NixFile::open(path)?;
    file.list_packages()
}

/// Insert package into specified path directly via AST
pub fn insert_into<P: AsRef<Path>>(path: P, pkg: &str) -> Result<bool, NixError> {
    let mut file = NixFile::open(path)?;
    let inserted = file.insert_package(pkg)?;
    if inserted {
        file.save()?;
    }
    Ok(inserted)
}

/// Remove package from specified path directly via AST
pub fn remove_from<P: AsRef<Path>>(path: P, pkg: &str) -> Result<bool, NixError> {
    let mut file = NixFile::open(path)?;
    let removed = file.remove_package(pkg)?;
    if removed {
        file.save()?;
    }
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEAN_PACKAGES_NIX: &str = r#"{ pkgs, ... }:

{
  home.packages = with pkgs; [
    git
    neovim
    ripgrep
  ];
}
"#;

    #[test]
    fn test_ast_list_packages_no_markers() {
        let file = NixFile::from_str("packages.nix", CLEAN_PACKAGES_NIX);
        let packages = file.list_packages().unwrap();
        assert_eq!(packages, vec!["git", "neovim", "ripgrep"]);
    }

    #[test]
    fn test_ast_insert_no_markers() {
        let mut file = NixFile::from_str("packages.nix", CLEAN_PACKAGES_NIX);
        assert!(file.insert_package("tmux").unwrap());
        assert!(file.contains_package("tmux").unwrap());

        // Deduplication: should not insert again
        assert!(!file.insert_package("tmux").unwrap());
    }

    #[test]
    fn test_ast_remove_no_markers() {
        let mut file = NixFile::from_str("packages.nix", CLEAN_PACKAGES_NIX);
        assert!(file.remove_package("neovim").unwrap());
        assert!(!file.contains_package("neovim").unwrap());
        assert_eq!(file.list_packages().unwrap(), vec!["git", "ripgrep"]);
    }
}
