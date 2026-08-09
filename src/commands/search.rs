use crate::shell::capture;

/// fetch full description for a specific package attr name
pub fn describe(pkg: &str) -> Option<String> {
    let raw = capture(&format!(
        "nh search {} --json 2>/dev/null | jq -r --arg name {} '.results[] | select(.package_attr_name == $name) | \"\\(.package_pname) (\\(.package_pversion))\\n\\(.package_description // \"no description\")\\n\\(.package_homepage[0] // \"\")\"'",
        shell_escape(pkg),
        shell_escape(pkg)
    ));
    if raw.trim().is_empty() { None } else { Some(raw) }
}

/// interactive fuzzy search, returns the selected package name if any
pub fn search() -> Option<String> {
    let jq_filter = r#".results[]? | "\(.package_attr_name)\t\(.package_description // "no description")""#;
    let filter_path = "/tmp/next_search_filter.jq";
    std::fs::write(filter_path, jq_filter).ok()?;

    let fzf_cmd = format!(
        "fzf --disabled --ansi --prompt='search> ' \
         --delimiter='\t' --with-nth=1 \
         --preview 'echo {{2}}' --preview-window=right:50%:wrap \
         --bind 'change:reload:nh search {{q}} --json 2>/dev/null | jq -r -f {filter_path} || true' \
         --header 'type to search nixpkgs live'"
    );

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&fzf_cmd)
        .output()
        .ok()?;

    let selected_line = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if selected_line.is_empty() {
        return None;
    }
    selected_line.split('\t').next().map(String::from)
}
/// interactive fuzzy search over an arbitrary static list (no live nh reload)
pub fn search_over(items: &[String]) -> Option<String> {
    let input = items.join("\n");
    let mut child = std::process::Command::new("sh")
        .arg("-c")
        .arg("fzf --prompt='select> ' --header 'pick a package'")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .ok()?;

    use std::io::Write;
    child.stdin.take()?.write_all(input.as_bytes()).ok()?;

    let output = child.wait_with_output().ok()?;
    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if selected.is_empty() { None } else { Some(selected) }
}

pub fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}
