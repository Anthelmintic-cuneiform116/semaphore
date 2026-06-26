use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ToolStatus {
    pub id: String,
    pub name: String,
    pub installed: bool,
    pub connected: bool,
}

const TOOLS: &[(&str, &str)] = &[
    ("cursor", "Cursor"),
    ("claude-code", "Claude Code"),
    ("codex", "Codex CLI"),
    ("gemini-cli", "Gemini CLI"),
    ("copilot-cli", "Copilot CLI"),
];

pub fn detect_tools() -> Vec<ToolStatus> {
    let home = home_dir();
    TOOLS
        .iter()
        .map(|(id, name)| {
            let config_path = tool_config_path(&home, id);
            ToolStatus {
                id: (*id).to_string(),
                name: (*name).to_string(),
                installed: is_tool_installed(&home, id),
                connected: hook_file_contains(&config_path),
            }
        })
        .collect()
}

pub fn tool_config_path(home: &Path, tool: &str) -> PathBuf {
    match tool {
        "cursor" => home.join(".cursor/hooks.json"),
        "claude-code" => home.join(".claude/settings.json"),
        "codex" => home.join(".codex/hooks.json"),
        "gemini-cli" => home.join(".gemini/settings.json"),
        "copilot-cli" => home.join(".copilot/hooks.json"),
        _ => home.join(format!(".{tool}/hooks.json")),
    }
}

pub fn is_tool_installed(home: &Path, tool: &str) -> bool {
    match tool {
        "cursor" => {
            home.join(".cursor").exists()
                || (cfg!(target_os = "macos")
                    && Path::new("/Applications/Cursor.app").exists())
        }
        "claude-code" => home.join(".claude").exists() || command_exists("claude"),
        "codex" => home.join(".codex").exists() || command_exists("codex"),
        "gemini-cli" => home.join(".gemini").exists() || command_exists("gemini"),
        "copilot-cli" => home.join(".copilot").exists() || command_exists("gh"),
        _ => false,
    }
}

pub fn hook_file_contains(path: &Path) -> bool {
    std::fs::read_to_string(path)
        .map(|s| s.contains("_semaphore"))
        .unwrap_or(false)
}

fn command_exists(name: &str) -> bool {
    let checker = if cfg!(windows) { "where" } else { "which" };
    std::process::Command::new(checker)
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn home_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home);
    }
    if let Ok(profile) = std::env::var("USERPROFILE") {
        return PathBuf::from(profile);
    }
    PathBuf::from(".")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn lock() -> std::sync::MutexGuard<'static, ()> {
        TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn detects_cursor_from_config_dir() {
        let _guard = lock();
        let tmp = tempfile::tempdir().unwrap();
        let cursor_dir = tmp.path().join(".cursor");
        fs::create_dir_all(&cursor_dir).unwrap();
        assert!(is_tool_installed(tmp.path(), "cursor"));
    }

    #[test]
    fn connected_when_marker_present() {
        let _guard = lock();
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("hooks.json");
        fs::write(&path, r#"{"hooks": {"_semaphore": true}}"#).unwrap();
        assert!(hook_file_contains(&path));
    }

    #[test]
    fn not_connected_without_marker() {
        let _guard = lock();
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("hooks.json");
        fs::write(&path, r#"{"hooks": {}}"#).unwrap();
        assert!(!hook_file_contains(&path));
    }

    #[test]
    fn detect_tools_returns_all_entries() {
        let tools = detect_tools();
        assert_eq!(tools.len(), 5);
        assert!(tools.iter().any(|t| t.id == "cursor"));
    }
}
