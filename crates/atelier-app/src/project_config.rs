use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::storage_layout::StorageLayout;

const PROJECT_CONFIG_SCHEMA: &str = "atelier.project_config";
const PROJECT_CONFIG_SCHEMA_VERSION: i64 = 1;
pub const FORGEJO_ROLES: &[&str] = &["worker", "reviewer", "validator", "manager", "admin"];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectConfig {
    pub project_slug: String,
    pub paths: ProjectPaths,
    pub compatibility_state_root: Option<String>,
    pub forgejo: Option<ForgejoConfig>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectPaths {
    pub state_root: String,
    pub runtime_dir: String,
    pub runtime_database: String,
    pub cache_dir: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoConfig {
    pub host: String,
    pub owner: String,
    pub repo: String,
    pub admin_token_env: String,
    pub sudo_users: ForgejoSudoUsers,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoSudoUsers {
    pub worker: String,
    pub reviewer: String,
    pub validator: String,
    pub manager: String,
    pub admin: String,
}

impl ProjectConfig {
    pub fn load(repo_root: &Path) -> Result<Self> {
        load(repo_root)
    }

    pub fn require_forgejo(&self, config_path: &Path) -> Result<&ForgejoConfig> {
        self.forgejo.as_ref().ok_or_else(|| {
            anyhow!(
                "forgejo_config_missing: {} is missing [forgejo]; configure host, owner, repo, admin_token_env, and [forgejo.sudo_users] role mappings before running `atelier pr` or PR validators",
                config_path.display()
            )
        })
    }
}

impl ForgejoConfig {
    pub fn sudo_user_for_role(&self, role: &str) -> Result<&str> {
        match role {
            "worker" => Ok(&self.sudo_users.worker),
            "reviewer" => Ok(&self.sudo_users.reviewer),
            "validator" => Ok(&self.sudo_users.validator),
            "manager" => Ok(&self.sudo_users.manager),
            "admin" => Ok(&self.sudo_users.admin),
            other => Err(anyhow!(
                "forgejo_config_invalid_role: unsupported Atelier role '{}'; expected {}",
                other,
                FORGEJO_ROLES.join(", ")
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProjectConfig {
    schema: String,
    schema_version: i64,
    project_slug: String,
    paths: RawProjectPaths,
    #[serde(default)]
    compatibility_state_root: Option<String>,
    #[serde(default)]
    forgejo: Option<RawForgejoConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProjectPaths {
    state_root: String,
    runtime_dir: String,
    runtime_database: String,
    cache_dir: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawForgejoConfig {
    host: Option<String>,
    owner: Option<String>,
    repo: Option<String>,
    admin_token_env: Option<String>,
    sudo_users: Option<RawForgejoSudoUsers>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawForgejoSudoUsers {
    worker: Option<String>,
    reviewer: Option<String>,
    validator: Option<String>,
    manager: Option<String>,
    admin: Option<String>,
}

pub fn load(repo_root: &Path) -> Result<ProjectConfig> {
    let layout = StorageLayout::new(repo_root);
    let config_path = layout.config_path();
    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    parse_project_config(&text, &config_path)
}

fn parse_project_config(text: &str, config_path: &Path) -> Result<ProjectConfig> {
    let raw = toml::from_str::<RawProjectConfig>(text).map_err(|error| {
        anyhow!(
            "project_config_parse_error: {}: {}",
            config_path.display(),
            error
        )
    })?;
    if raw.schema != PROJECT_CONFIG_SCHEMA {
        return Err(anyhow!(
            "project_config_schema_unsupported: {} schema must be '{}'",
            config_path.display(),
            PROJECT_CONFIG_SCHEMA
        ));
    }
    if raw.schema_version != PROJECT_CONFIG_SCHEMA_VERSION {
        return Err(anyhow!(
            "project_config_schema_unsupported: {} schema_version must be {}",
            config_path.display(),
            PROJECT_CONFIG_SCHEMA_VERSION
        ));
    }
    require_non_empty(&raw.project_slug, config_path, "project_slug")?;
    let paths = ProjectPaths {
        state_root: require_owned(raw.paths.state_root, config_path, "paths.state_root")?,
        runtime_dir: require_owned(raw.paths.runtime_dir, config_path, "paths.runtime_dir")?,
        runtime_database: require_owned(
            raw.paths.runtime_database,
            config_path,
            "paths.runtime_database",
        )?,
        cache_dir: require_owned(raw.paths.cache_dir, config_path, "paths.cache_dir")?,
    };
    let forgejo = raw
        .forgejo
        .map(|forgejo| parse_forgejo_config(forgejo, config_path))
        .transpose()?;
    Ok(ProjectConfig {
        project_slug: raw.project_slug,
        paths,
        compatibility_state_root: raw.compatibility_state_root,
        forgejo,
    })
}

fn parse_forgejo_config(raw: RawForgejoConfig, config_path: &Path) -> Result<ForgejoConfig> {
    let sudo_users = raw.sudo_users.ok_or_else(|| {
        anyhow!(
            "forgejo_config_invalid: {} [forgejo.sudo_users] is required; map {} to Forgejo sudo users",
            config_path.display(),
            FORGEJO_ROLES.join(", ")
        )
    })?;
    let config = ForgejoConfig {
        host: require_owned_option(raw.host, config_path, "forgejo.host")?,
        owner: require_owned_option(raw.owner, config_path, "forgejo.owner")?,
        repo: require_owned_option(raw.repo, config_path, "forgejo.repo")?,
        admin_token_env: require_env_var_name(
            raw.admin_token_env,
            config_path,
            "forgejo.admin_token_env",
        )?,
        sudo_users: ForgejoSudoUsers {
            worker: require_owned_option(
                sudo_users.worker,
                config_path,
                "forgejo.sudo_users.worker",
            )?,
            reviewer: require_owned_option(
                sudo_users.reviewer,
                config_path,
                "forgejo.sudo_users.reviewer",
            )?,
            validator: require_owned_option(
                sudo_users.validator,
                config_path,
                "forgejo.sudo_users.validator",
            )?,
            manager: require_owned_option(
                sudo_users.manager,
                config_path,
                "forgejo.sudo_users.manager",
            )?,
            admin: require_owned_option(sudo_users.admin, config_path, "forgejo.sudo_users.admin")?,
        },
    };
    Ok(config)
}

fn require_owned(value: String, config_path: &Path, field: &str) -> Result<String> {
    require_non_empty(&value, config_path, field)?;
    Ok(value)
}

fn require_owned_option(value: Option<String>, config_path: &Path, field: &str) -> Result<String> {
    let value = value.ok_or_else(|| {
        anyhow!(
            "forgejo_config_invalid: {} field '{}' is required",
            config_path.display(),
            field
        )
    })?;
    require_owned(value, config_path, field)
}

fn require_env_var_name(value: Option<String>, config_path: &Path, field: &str) -> Result<String> {
    let value = require_owned_option(value, config_path, field)?;
    let valid = value.chars().enumerate().all(|(index, ch)| {
        ch == '_' || ch.is_ascii_uppercase() || (index > 0 && ch.is_ascii_digit())
    });
    if !valid || value.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        return Err(anyhow!(
            "forgejo_config_invalid: {} field '{}' must be an environment variable name such as FORGEJO_ADMIN_TOKEN",
            config_path.display(),
            field
        ));
    }
    Ok(value)
}

fn require_non_empty(value: &str, config_path: &Path, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!(
            "project_config_invalid: {} field '{}' must not be empty",
            config_path.display(),
            field
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn path() -> PathBuf {
        PathBuf::from(".atelier/config.toml")
    }

    fn valid_config() -> &'static str {
        r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"
compatibility_state_root = ".atelier-state"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/runtime"
runtime_database = ".atelier/runtime/state.db"
cache_dir = ".atelier/cache"

[forgejo]
host = "forge.example.test"
owner = "tools"
repo = "atelier"
admin_token_env = "FORGEJO_ADMIN_TOKEN"

[forgejo.sudo_users]
worker = "atelier-worker"
reviewer = "atelier-reviewer"
validator = "atelier-validator"
manager = "atelier-manager"
admin = "atelier-admin"
"#
    }

    #[test]
    fn parses_valid_forgejo_config_and_sudo_mapping() {
        let config = parse_project_config(valid_config(), &path()).unwrap();
        let forgejo = config.require_forgejo(&path()).unwrap();

        assert_eq!(forgejo.host, "forge.example.test");
        assert_eq!(forgejo.owner, "tools");
        assert_eq!(forgejo.repo, "atelier");
        assert_eq!(forgejo.admin_token_env, "FORGEJO_ADMIN_TOKEN");
        assert_eq!(
            forgejo.sudo_user_for_role("worker").unwrap(),
            "atelier-worker"
        );
        assert_eq!(
            forgejo.sudo_user_for_role("validator").unwrap(),
            "atelier-validator"
        );
    }

    #[test]
    fn missing_forgejo_config_is_actionable() {
        let config = parse_project_config(
            r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"

[paths]
state_root = ".atelier"
runtime_dir = ".atelier/runtime"
runtime_database = ".atelier/runtime/state.db"
cache_dir = ".atelier/cache"
"#,
            &path(),
        )
        .unwrap();

        let error = config.require_forgejo(&path()).unwrap_err().to_string();

        assert!(error.contains("forgejo_config_missing"));
        assert!(error.contains("before running `atelier pr` or PR validators"));
    }

    #[test]
    fn invalid_forgejo_config_names_missing_role_and_token() {
        let missing_role = valid_config().replace("validator = \"atelier-validator\"\n", "");
        let error = parse_project_config(&missing_role, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("forgejo.sudo_users.validator"));

        let bad_token = valid_config().replace(
            "admin_token_env = \"FORGEJO_ADMIN_TOKEN\"",
            "admin_token_env = \"forgejo-token\"",
        );
        let error = parse_project_config(&bad_token, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("forgejo.admin_token_env"));
        assert!(error.contains("FORGEJO_ADMIN_TOKEN"));
    }
}
