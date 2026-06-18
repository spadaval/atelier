use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::storage_layout::StorageLayout;

const PROJECT_CONFIG_SCHEMA: &str = "atelier.project_config";
const PROJECT_CONFIG_SCHEMA_VERSION: i64 = 1;
pub const FORGEJO_ROLES: &[&str] = &["worker", "reviewer", "validator", "manager"];

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
    pub role_authors: ForgejoRoleAuthors,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoRoleAuthors {
    pub worker: String,
    pub reviewer: String,
    pub validator: String,
    pub manager: String,
}

impl ProjectConfig {
    pub fn load(repo_root: &Path) -> Result<Self> {
        load(repo_root)
    }

    pub fn require_forgejo(&self, config_path: &Path) -> Result<&ForgejoConfig> {
        self.forgejo.as_ref().ok_or_else(|| {
            anyhow!(
                "forgejo_config_missing: {} is missing [forgejo]; configure host, owner, repo, admin_token_env, and [forgejo.role_authors] role mappings before running `atelier pr` or PR validators",
                config_path.display()
            )
        })
    }
}

impl ForgejoConfig {
    pub fn role_author_for_role(&self, role: &str) -> Result<&str> {
        match role {
            "worker" => Ok(&self.role_authors.worker),
            "reviewer" => Ok(&self.role_authors.reviewer),
            "validator" => Ok(&self.role_authors.validator),
            "manager" => Ok(&self.role_authors.manager),
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
    role_authors: Option<RawForgejoRoleAuthors>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawForgejoRoleAuthors {
    worker: Option<String>,
    reviewer: Option<String>,
    validator: Option<String>,
    manager: Option<String>,
}

pub fn load(repo_root: &Path) -> Result<ProjectConfig> {
    let layout = StorageLayout::new(repo_root);
    let config_path = layout.config_path();
    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    parse_project_config(&text, &config_path)
}

pub fn load_forgejo_with_default_role_authors(
    repo_root: &Path,
    role_authors: ForgejoRoleAuthors,
) -> Result<ForgejoConfig> {
    let layout = StorageLayout::new(repo_root);
    let config_path = layout.config_path();
    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    let raw = parse_raw_project_config(&text, &config_path)?;
    validate_schema_fields(&raw, &config_path)?;
    require_non_empty(&raw.project_slug, &config_path, "project_slug")?;
    let raw_forgejo = raw.forgejo.ok_or_else(|| {
        anyhow!(
            "forgejo_config_missing: {} is missing [forgejo]; configure host, owner, repo, and admin_token_env before provisioning Forgejo role authors",
            config_path.display()
        )
    })?;
    Ok(ForgejoConfig {
        host: require_owned_option(raw_forgejo.host, &config_path, "forgejo.host")?,
        owner: require_owned_option(raw_forgejo.owner, &config_path, "forgejo.owner")?,
        repo: require_owned_option(raw_forgejo.repo, &config_path, "forgejo.repo")?,
        admin_token_env: require_env_var_name(
            raw_forgejo.admin_token_env,
            &config_path,
            "forgejo.admin_token_env",
        )?,
        role_authors,
    })
}

fn parse_project_config(text: &str, config_path: &Path) -> Result<ProjectConfig> {
    let raw = parse_raw_project_config(text, config_path)?;
    validate_schema_fields(&raw, config_path)?;
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

fn parse_raw_project_config(text: &str, config_path: &Path) -> Result<RawProjectConfig> {
    toml::from_str::<RawProjectConfig>(text).map_err(|error| {
        anyhow!(
            "project_config_parse_error: {}: {}",
            config_path.display(),
            error
        )
    })
}

fn validate_schema_fields(raw: &RawProjectConfig, config_path: &Path) -> Result<()> {
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
    Ok(())
}

fn parse_forgejo_config(raw: RawForgejoConfig, config_path: &Path) -> Result<ForgejoConfig> {
    let role_authors = raw.role_authors.ok_or_else(|| {
        anyhow!(
            "forgejo_config_invalid: {} [forgejo.role_authors] is required; map {} to Forgejo role author users",
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
        role_authors: ForgejoRoleAuthors {
            worker: require_owned_option(
                role_authors.worker,
                config_path,
                "forgejo.role_authors.worker",
            )?,
            reviewer: require_owned_option(
                role_authors.reviewer,
                config_path,
                "forgejo.role_authors.reviewer",
            )?,
            validator: require_owned_option(
                role_authors.validator,
                config_path,
                "forgejo.role_authors.validator",
            )?,
            manager: require_owned_option(
                role_authors.manager,
                config_path,
                "forgejo.role_authors.manager",
            )?,
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

[forgejo.role_authors]
worker = "atelier-worker"
reviewer = "atelier-reviewer"
validator = "atelier-validator"
manager = "atelier-manager"
"#
    }

    #[test]
    fn parses_valid_forgejo_config_and_role_authors() {
        let config = parse_project_config(valid_config(), &path()).unwrap();
        let forgejo = config.require_forgejo(&path()).unwrap();

        assert_eq!(forgejo.host, "forge.example.test");
        assert_eq!(forgejo.owner, "tools");
        assert_eq!(forgejo.repo, "atelier");
        assert_eq!(forgejo.admin_token_env, "FORGEJO_ADMIN_TOKEN");
        assert_eq!(
            forgejo.role_author_for_role("worker").unwrap(),
            "atelier-worker"
        );
        assert_eq!(
            forgejo.role_author_for_role("validator").unwrap(),
            "atelier-validator"
        );
        assert!(forgejo.role_author_for_role("admin").is_err());
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
        assert!(error.contains("forgejo.role_authors.validator"));

        let old_sudo_users = valid_config()
            .replace("[forgejo.role_authors]", "[forgejo.sudo_users]")
            + "admin = \"atelier-admin\"\n";
        let error = parse_project_config(&old_sudo_users, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("unknown field `sudo_users`"));

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
