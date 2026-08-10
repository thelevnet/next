// ============ messages.rs — раздел apps ============
pub const PREFIX: &str = "\x1b[1;34m[next]\x1b[0m";
// ── apps errors ─────────────────────────────────────
pub const ERR_UPDATE_APPS_FILE: &str = "Failed to update apps file";
pub const ERR_REMOVE_APP: &str = "Failed to remove app";
pub const ERR_APP_NOT_FOUND: &str = "App not found";
pub const ERR_NO_INSTALLED_PACKAGES: &str = "No installed packages found";
pub const ERR_NO_APPS_FOUND: &str = "No apps found";

// ── apps prompts ────────────────────────────────────
pub const PROMPT_INSTALL: &str = "Install";
pub const PROMPT_REMOVE: &str = "Remove";

// ── apps status ─────────────────────────────────────
pub const MSG_NO_SELECTION: &str = "No app selected";
pub const APP_INSTALLED: &str = "Installed:";
pub const APP_REMOVED: &str = "Removed:";

// ============ messages.rs — раздел check ============

// ── check status ────────────────────────────────────
pub const MSG_CHECKING_FLAKE: &str = "Checking flake syntax";
pub const MSG_CHECKING_GIT: &str = "Checking git status";
pub const MSG_ALL_CLEAR: &str = "All clear";
pub const OS_COMMIT_MSG: &str = "Update to /etc/nixos/configuration.nix";
pub const HOME_COMMIT_MSG: &str = "Update to /etc/nixos/home.nix";

// ── check errors ────────────────────────────────────
pub const ERR_FLAKE_CHECK_FAILED: &str = "Flake check failed";
pub const ERR_GIT_DIRTY: &str = "Uncommitted changes present";

// ============ messages.rs — раздел conf ============

// ── conf status ─────────────────────────────────────
pub const MSG_AVAILABLE_DOTS: &str = "Available dots";

// ── conf errors ─────────────────────────────────────
pub const ERR_MODULE_NOT_FOUND: &str = "Module not found in";

// ── conf prompts ────────────────────────────────────
pub const PROMPT_HOME_SWITCH: &str = "Home switch now?";

// ============ messages.rs — раздел git ============

// ── git prompts ─────────────────────────────────────
pub const PROMPT_COMMIT_MSG: &str = "Commit message";
pub const COMMIT_PUSH_MSG: &str = "commit & push with message:";

// ── git errors ──────────────────────────────────────
pub const ERR_EMPTY_COMMIT_MSG: &str = "Empty commit message, aborting";

// ============ messages.rs — раздел search ============

// ── search fzf ui ───────────────────────────────────
pub const FZF_SEARCH_PROMPT: &str = "search> ";
pub const FZF_SEARCH_HEADER: &str = "type to search nixpkgs live";
pub const FZF_SELECT_PROMPT: &str = "select> ";
pub const FZF_SELECT_HEADER: &str = "pick a package";
pub const NO_DESCRIPTION: &str = "no description";

// ============ messages.rs — раздел nix_parser ============

// ── nix_parser errors ───────────────────────────────
pub const ERR_MARKER_NOT_FOUND: &str = "Marker not found in";
