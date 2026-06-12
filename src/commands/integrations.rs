use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

const SETTINGS_JSON: &str = include_str!("../../resources/claude/settings.json");
const PROMPT_GUARD_PY: &str = include_str!("../../resources/claude/hooks/prompt-guard.py");
const POST_EDIT_CHECK_PY: &str = include_str!("../../resources/claude/hooks/post-edit-check.py");
const SESSION_START_PY: &str = include_str!("../../resources/claude/hooks/session-start.py");
const PRE_WEB_CHECK_PY: &str = include_str!("../../resources/claude/hooks/pre-web-check.py");
const WORK_CHECK_PY: &str = include_str!("../../resources/claude/hooks/work-check.py");
const ATELIER_CONFIG_PY: &str = include_str!("../../resources/claude/hooks/atelier_config.py");
const SAFE_FETCH_SERVER_PY: &str = include_str!("../../resources/claude/mcp/safe-fetch-server.py");
const MCP_JSON: &str = include_str!("../../resources/mcp.json");
const HOOK_CONFIG_JSON: &str = include_str!("../../resources/atelier/hook-config.json");

pub fn install_claude(repo_root: &Path, force: bool) -> Result<()> {
    let layout = crate::storage_layout::StorageLayout::new(repo_root);
    let atelier_dir = layout.atelier_dir();
    let config_path = layout.config_path();
    let config = fs::read_to_string(&config_path).with_context(|| {
        format!(
            "Claude integration requires initialized Atelier config at {}. Run `atelier init` first.",
            config_path.display()
        )
    })?;
    ensure_project_config_uses_atelier_root(&config, &config_path)?;

    let claude_dir = repo_root.join(".claude");
    let hooks_dir = claude_dir.join("hooks");
    let mcp_dir = claude_dir.join("mcp");

    fs::create_dir_all(&hooks_dir).context("Failed to create .claude/hooks directory")?;
    fs::create_dir_all(&mcp_dir).context("Failed to create .claude/mcp directory")?;

    write_if_missing_or_forced(&claude_dir.join("settings.json"), SETTINGS_JSON, force)?;
    write_if_missing_or_forced(&hooks_dir.join("prompt-guard.py"), PROMPT_GUARD_PY, force)?;
    write_if_missing_or_forced(
        &hooks_dir.join("post-edit-check.py"),
        POST_EDIT_CHECK_PY,
        force,
    )?;
    write_if_missing_or_forced(&hooks_dir.join("session-start.py"), SESSION_START_PY, force)?;
    write_if_missing_or_forced(&hooks_dir.join("pre-web-check.py"), PRE_WEB_CHECK_PY, force)?;
    write_if_missing_or_forced(&hooks_dir.join("work-check.py"), WORK_CHECK_PY, force)?;
    write_if_missing_or_forced(
        &hooks_dir.join("atelier_config.py"),
        ATELIER_CONFIG_PY,
        force,
    )?;
    write_if_missing_or_forced(
        &mcp_dir.join("safe-fetch-server.py"),
        SAFE_FETCH_SERVER_PY,
        force,
    )?;
    write_if_missing_or_forced(
        &atelier_dir.join("hook-config.json"),
        HOOK_CONFIG_JSON,
        force,
    )?;

    let warnings =
        write_mcp_json_merged(&repo_root.join(".mcp.json")).context("Failed to write .mcp.json")?;
    for warning in warnings {
        println!("{}", warning);
    }

    println!("Claude integration installed.");
    println!("Installed .claude/hooks, .claude/mcp, .claude/settings.json, and .mcp.json.");
    println!("Atelier core tracker state remains in .atelier/ records.");
    Ok(())
}

fn ensure_project_config_uses_atelier_root(config: &str, path: &Path) -> Result<()> {
    if !config.contains("schema = \"atelier.project_config\"") {
        bail!(
            "{} is not an Atelier project config; run `atelier init` before installing integrations.",
            path.display()
        );
    }
    if !config.contains("state_root = \".atelier\"") {
        bail!(
            "{} does not configure state_root = \".atelier\"; Claude integration only supports the current Atelier layout.",
            path.display()
        );
    }
    Ok(())
}

fn write_if_missing_or_forced(path: &Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Ok(());
    }
    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn write_mcp_json_merged(mcp_path: &Path) -> Result<Vec<String>> {
    let embedded: serde_json::Value = serde_json::from_str(MCP_JSON)
        .context("embedded MCP_JSON is not valid JSON; this is a build defect")?;
    let src_servers = embedded
        .get("mcpServers")
        .and_then(|v| v.as_object())
        .context("embedded MCP_JSON missing mcpServers object; this is a build defect")?;

    let mut obj = match fs::read_to_string(mcp_path) {
        Ok(raw) => {
            let parsed: serde_json::Value = serde_json::from_str(&raw).with_context(|| {
                format!(
                    "Existing .mcp.json at {} contains invalid JSON; refusing to overwrite. Fix or remove it, then retry.",
                    mcp_path.display()
                )
            })?;
            match parsed {
                serde_json::Value::Object(map) => map,
                _ => bail!(
                    "Existing .mcp.json at {} is not a JSON object; refusing to overwrite. Fix or remove it, then retry.",
                    mcp_path.display()
                ),
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => serde_json::Map::new(),
        Err(e) => return Err(anyhow::Error::from(e).context("Failed to read existing .mcp.json")),
    };

    let mut dest_map = match obj.remove("mcpServers") {
        Some(serde_json::Value::Object(map)) => map,
        Some(_) => bail!(
            "Existing .mcp.json has a non-object mcpServers value; refusing to overwrite. Fix or remove it, then retry."
        ),
        None => serde_json::Map::new(),
    };

    let mut warnings = Vec::new();
    for (key, value) in src_servers {
        if dest_map.contains_key(key) {
            warnings.push(format!(
                "Warning: overwriting existing mcpServers entry \"{}\" with atelier default",
                key
            ));
        }
        dest_map.insert(key.clone(), value.clone());
    }

    obj.insert("mcpServers".into(), serde_json::Value::Object(dest_map));
    let mut output = serde_json::to_string_pretty(&serde_json::Value::Object(obj))
        .context("Failed to serialize .mcp.json")?;
    output.push('\n');
    fs::write(mcp_path, output).context("Failed to write .mcp.json")?;
    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn install_claude_requires_project_config() {
        let dir = tempdir().unwrap();

        let error = install_claude(dir.path(), false).unwrap_err();

        assert!(error
            .to_string()
            .contains("Claude integration requires initialized Atelier config"));
    }

    #[test]
    fn install_claude_preserves_existing_files_without_force() {
        let dir = tempdir().unwrap();
        crate::commands::init::run(dir.path(), false).unwrap();
        fs::create_dir_all(dir.path().join(".claude/hooks")).unwrap();
        let hook_path = dir.path().join(".claude/hooks/prompt-guard.py");
        fs::write(&hook_path, "# custom\n").unwrap();

        install_claude(dir.path(), false).unwrap();

        assert_eq!(fs::read_to_string(hook_path).unwrap(), "# custom\n");
    }

    #[test]
    fn install_claude_force_updates_files_and_preserves_custom_mcp_servers() {
        let dir = tempdir().unwrap();
        crate::commands::init::run(dir.path(), false).unwrap();
        fs::write(
            dir.path().join(".mcp.json"),
            r#"{"mcpServers":{"custom":{"command":"node","args":["server.js"]}}}"#,
        )
        .unwrap();

        install_claude(dir.path(), true).unwrap();

        let mcp: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(dir.path().join(".mcp.json")).unwrap())
                .unwrap();
        assert!(mcp["mcpServers"]["atelier-safe-fetch"].is_object());
        assert_eq!(mcp["mcpServers"]["custom"]["command"], "node");
        assert!(dir.path().join(".claude/hooks/prompt-guard.py").is_file());
        assert!(dir
            .path()
            .join(".claude/mcp/safe-fetch-server.py")
            .is_file());
        assert!(dir.path().join(".atelier/hook-config.json").is_file());
        assert!(!dir.path().join(".atelier/rules").exists());
    }
}
