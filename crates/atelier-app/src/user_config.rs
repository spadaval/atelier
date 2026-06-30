use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;

pub const USER_CONFIG_DISPLAY_PATH: &str = "~/.config/atelier.toml";

#[derive(Debug, Deserialize)]
struct RawUserConfig {
    forgejo: Option<RawForgejoUserConfig>,
}

#[derive(Debug, Deserialize)]
struct RawForgejoUserConfig {
    admin_token: Option<String>,
}

pub fn user_config_path() -> Result<PathBuf> {
    let home = std::env::var_os("HOME").ok_or_else(|| {
        anyhow!(
            "atelier_user_config_missing_home: HOME is required to locate {}",
            USER_CONFIG_DISPLAY_PATH
        )
    })?;
    Ok(PathBuf::from(home).join(".config/atelier.toml"))
}

pub fn forgejo_admin_token() -> Result<String> {
    let path = user_config_path()?;
    let text = fs::read_to_string(&path).with_context(|| {
        format!(
            "forgejo_config_missing_token: set forgejo.admin_token in {}",
            USER_CONFIG_DISPLAY_PATH
        )
    })?;
    let raw = toml::from_str::<RawUserConfig>(&text)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    let token = raw
        .forgejo
        .and_then(|forgejo| forgejo.admin_token)
        .ok_or_else(|| {
            anyhow!(
                "forgejo_config_missing_token: set forgejo.admin_token in {}",
                USER_CONFIG_DISPLAY_PATH
            )
        })?;
    if token.trim().is_empty() {
        bail!(
            "forgejo_config_missing_token: forgejo.admin_token in {} must not be empty",
            USER_CONFIG_DISPLAY_PATH
        );
    }
    Ok(token)
}

pub fn write_forgejo_admin_token(token: &str) -> Result<PathBuf> {
    if token.trim().is_empty() {
        bail!("forgejo_config_missing_token: Forgejo admin token must not be empty");
    }
    let path = user_config_path()?;
    let mut root = if path.exists() {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        match toml::from_str::<toml::Value>(&text)
            .with_context(|| format!("failed to parse {}", path.display()))?
        {
            toml::Value::Table(table) => table,
            _ => bail!(
                "atelier_user_config_invalid: {} must be a TOML table",
                path.display()
            ),
        }
    } else {
        toml::map::Map::new()
    };

    let forgejo = root
        .entry("forgejo".to_string())
        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
    let forgejo = forgejo.as_table_mut().ok_or_else(|| {
        anyhow!(
            "atelier_user_config_invalid: {} key 'forgejo' must be a TOML table",
            path.display()
        )
    })?;
    forgejo.insert(
        "admin_token".to_string(),
        toml::Value::String(token.to_string()),
    );

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let text = toml::to_string_pretty(&toml::Value::Table(root))
        .context("failed to render Atelier user config")?;
    fs::write(&path, text).with_context(|| format!("failed to write {}", path.display()))?;
    restrict_user_config_permissions(&path)?;
    Ok(path)
}

#[cfg(unix)]
fn restrict_user_config_permissions(path: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .with_context(|| format!("failed to inspect {}", path.display()))?
        .permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(path, permissions)
        .with_context(|| format!("failed to set permissions on {}", path.display()))
}

#[cfg(not(unix))]
fn restrict_user_config_permissions(_path: &std::path::Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_forgejo_admin_token_without_exposing_secret() {
        let raw: RawUserConfig =
            toml::from_str("[forgejo]\nadmin_token = \"secret-token\"\n").unwrap();
        assert_eq!(
            raw.forgejo
                .and_then(|forgejo| forgejo.admin_token)
                .as_deref(),
            Some("secret-token")
        );
    }
}
