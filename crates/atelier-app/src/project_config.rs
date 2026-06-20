use std::fs;
use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;

use crate::storage_layout::StorageLayout;
use crate::workflow_policy::{self, ActionParams, WorkflowForgejoRoleAuthors};

const PROJECT_CONFIG_SCHEMA: &str = "atelier.project_config";
const PROJECT_CONFIG_SCHEMA_VERSION: i64 = 1;
pub const FORGEJO_ROLES: &[&str] = &["worker", "reviewer", "validator", "manager"];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectConfig {
    pub project_slug: String,
    pub paths: ProjectPaths,
    pub review: ReviewConfig,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectPaths {
    pub state_root: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReviewConfig {
    Room,
    Provider(ReviewProviderConfig),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReviewProviderConfig {
    pub provider: ReviewProviderKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReviewProviderKind {
    Forgejo(ForgejoConfig),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForgejoConfig {
    pub host: String,
    pub owner: String,
    pub repo: String,
    pub admin_token_env: String,
    pub role_authors: Option<ForgejoRoleAuthors>,
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
        match &self.review {
            ReviewConfig::Provider(ReviewProviderConfig {
                provider: ReviewProviderKind::Forgejo(forgejo),
            }) => Ok(forgejo),
            ReviewConfig::Room => Err(anyhow!(
                "review_mode_invalid: {} uses review.mode = \"room\"; provider-only commands require review.mode = \"provider\" and provider = \"forgejo\"",
                config_path.display()
            )),
        }
    }
}

pub fn load_forgejo_with_workflow_role_authors(repo_root: &Path) -> Result<ForgejoConfig> {
    let config_path = StorageLayout::new(repo_root).config_path();
    let config = ProjectConfig::load(repo_root)?;
    let mut forgejo = config.require_forgejo(&config_path)?.clone();
    forgejo.role_authors = Some(workflow_forgejo_role_authors(repo_root)?);
    Ok(forgejo)
}

pub fn workflow_forgejo_role_authors(repo_root: &Path) -> Result<ForgejoRoleAuthors> {
    let policy = workflow_policy::load(repo_root)?;
    let mut found: Option<WorkflowForgejoRoleAuthors> = None;
    for workflow in policy.workflows.values() {
        for transition in workflow.transitions.values() {
            for action in &transition.actions {
                let Some(ActionParams::ReviewArtifact(params)) = action.params.as_ref() else {
                    continue;
                };
                if params.provider.as_deref() != Some("forgejo") {
                    continue;
                }
                let role_authors = params.role_authors.clone().ok_or_else(|| {
                    anyhow!(
                        "workflow_config_invalid_action: Forgejo review action '{}' is missing role_authors",
                        action.builtin
                    )
                })?;
                if let Some(existing) = &found {
                    if existing != &role_authors {
                        bail!(
                            "workflow_config_invalid_action: Forgejo review actions define conflicting role_authors"
                        );
                    }
                } else {
                    found = Some(role_authors);
                }
            }
        }
    }
    let role_authors = found.ok_or_else(|| {
        anyhow!(
            "workflow_config_missing_role_authors: no Forgejo review action role_authors found in .atelier/workflow.yaml"
        )
    })?;
    Ok(workflow_role_authors_to_project(&role_authors))
}

fn workflow_role_authors_to_project(
    role_authors: &WorkflowForgejoRoleAuthors,
) -> ForgejoRoleAuthors {
    ForgejoRoleAuthors {
        worker: role_authors.worker.clone(),
        reviewer: role_authors.reviewer.clone(),
        validator: role_authors.validator.clone(),
        manager: role_authors.manager.clone(),
    }
}

impl ForgejoConfig {
    pub fn role_author_for_role(&self, role: &str) -> Result<&str> {
        let role_authors = self.role_authors.as_ref().ok_or_else(|| {
            anyhow!(
                "forgejo_config_missing_role_authors: Forgejo role authors are required for role '{}'; configure them in workflow action params",
                role
            )
        })?;
        match role {
            "worker" => Ok(&role_authors.worker),
            "reviewer" => Ok(&role_authors.reviewer),
            "validator" => Ok(&role_authors.validator),
            "manager" => Ok(&role_authors.manager),
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
    review: Option<RawReviewConfig>,
    #[serde(default)]
    forgejo: Option<toml::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProjectPaths {
    state_root: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReviewConfig {
    mode: Option<String>,
    provider: Option<String>,
    providers: Option<RawReviewProviders>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReviewProviders {
    forgejo: Option<RawForgejoConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawForgejoConfig {
    host: Option<String>,
    owner: Option<String>,
    repo: Option<String>,
    admin_token_env: Option<String>,
}

pub fn load(repo_root: &Path) -> Result<ProjectConfig> {
    let layout = StorageLayout::new(repo_root);
    let config_path = layout.config_path();
    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    parse_project_config(&text, &config_path)
}

fn parse_project_config(text: &str, config_path: &Path) -> Result<ProjectConfig> {
    let raw = parse_raw_project_config(text, config_path)?;
    validate_schema_fields(&raw, config_path)?;
    reject_legacy_forgejo(&raw, config_path)?;
    require_non_empty(&raw.project_slug, config_path, "project_slug")?;
    let paths = ProjectPaths {
        state_root: require_owned(raw.paths.state_root, config_path, "paths.state_root")?,
    };
    let review = parse_review_config(raw.review, config_path)?;
    Ok(ProjectConfig {
        project_slug: raw.project_slug,
        paths,
        review,
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

fn reject_legacy_forgejo(raw: &RawProjectConfig, config_path: &Path) -> Result<()> {
    if raw.forgejo.is_some() {
        return Err(anyhow!(
            "review_config_legacy_forgejo: {} top-level [forgejo] is no longer accepted; move settings under [review] mode = \"provider\", provider = \"forgejo\", and [review.providers.forgejo]",
            config_path.display()
        ));
    }
    Ok(())
}

fn parse_review_config(raw: Option<RawReviewConfig>, config_path: &Path) -> Result<ReviewConfig> {
    let raw = raw.ok_or_else(|| {
        anyhow!(
            "review_config_missing: {} is missing [review]; configure review.mode = \"room\" or review.mode = \"provider\"",
            config_path.display()
        )
    })?;
    let mode = require_owned_option(raw.mode.clone(), config_path, "review.mode")?;
    match mode.as_str() {
        "room" => {
            if raw.provider.is_some() || raw.providers.is_some() {
                return Err(anyhow!(
                    "review_config_invalid: {} review.mode = \"room\" must not define provider settings",
                    config_path.display()
                ));
            }
            Ok(ReviewConfig::Room)
        }
        "provider" => {
            let raw_forgejo = provider_forgejo_config(raw, config_path)?;
            Ok(ReviewConfig::Provider(ReviewProviderConfig {
                provider: ReviewProviderKind::Forgejo(parse_forgejo_config(
                    raw_forgejo,
                    config_path,
                )?),
            }))
        }
        other => Err(anyhow!(
            "review_config_invalid: {} review.mode must be \"room\" or \"provider\", got '{}'",
            config_path.display(),
            other
        )),
    }
}

fn provider_forgejo_config(raw: RawReviewConfig, config_path: &Path) -> Result<RawForgejoConfig> {
    let provider = require_owned_option(raw.provider, config_path, "review.provider")?;
    if provider != "forgejo" {
        return Err(anyhow!(
            "review_config_invalid: {} review.provider must be \"forgejo\", got '{}'",
            config_path.display(),
            provider
        ));
    }
    let providers = raw.providers.ok_or_else(|| {
        anyhow!(
            "forgejo_config_missing: {} is missing [review.providers.forgejo]",
            config_path.display()
        )
    })?;
    providers.forgejo.ok_or_else(|| {
        anyhow!(
            "forgejo_config_missing: {} is missing [review.providers.forgejo]",
            config_path.display()
        )
    })
}

fn parse_forgejo_config(raw: RawForgejoConfig, config_path: &Path) -> Result<ForgejoConfig> {
    let config = ForgejoConfig {
        host: require_owned_option(raw.host, config_path, "review.providers.forgejo.host")?,
        owner: require_owned_option(raw.owner, config_path, "review.providers.forgejo.owner")?,
        repo: require_owned_option(raw.repo, config_path, "review.providers.forgejo.repo")?,
        admin_token_env: require_env_var_name(
            raw.admin_token_env,
            config_path,
            "review.providers.forgejo.admin_token_env",
        )?,
        role_authors: None,
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
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    fn path() -> PathBuf {
        PathBuf::from(".atelier/config.toml")
    }

    fn valid_config() -> &'static str {
        r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "atelier"

[paths]
state_root = ".atelier"

[review]
mode = "provider"
provider = "forgejo"

[review.providers.forgejo]
host = "forge.example.test"
owner = "tools"
repo = "atelier"
admin_token_env = "FORGEJO_ADMIN_TOKEN"
"#
    }

    fn valid_workflow(role_suffix: &str) -> String {
        format!(
            r#"schema: atelier.workflow
schema_version: 3
branch_policy:
  base_branch: master
  merge_strategy: squash
issue_types:
  epic: {{ label: Epic }}
statuses:
  todo:
    category: todo
  review:
    category: active
  done:
    category: done
workflows:
  epic_delivery:
    applies_to: [epic]
    initial_status: todo
    done_statuses: [done]
    transitions:
      request_review:
        from: [todo]
        to: review
        actions:
          - review.open:
              provider: forgejo
              role: worker
              role_authors:
                worker: forge-worker{role_suffix}
                reviewer: forge-reviewer{role_suffix}
                validator: forge-validator{role_suffix}
                manager: forge-manager{role_suffix}
"#
        )
    }

    fn write_repo_config(repo_root: &Path, workflow: &str) {
        let atelier_dir = repo_root.join(".atelier");
        fs::create_dir_all(&atelier_dir).unwrap();
        fs::write(atelier_dir.join("config.toml"), valid_config()).unwrap();
        fs::write(atelier_dir.join("workflow.yaml"), workflow).unwrap();
    }

    #[test]
    fn forgejo_loader_applies_workflow_role_authors() {
        let dir = tempdir().unwrap();
        write_repo_config(dir.path(), &valid_workflow(""));

        let forgejo = load_forgejo_with_workflow_role_authors(dir.path()).unwrap();

        assert_eq!(forgejo.host, "forge.example.test");
        assert_eq!(
            forgejo.role_author_for_role("worker").unwrap(),
            "forge-worker"
        );
        assert_eq!(
            forgejo.role_author_for_role("validator").unwrap(),
            "forge-validator"
        );
    }

    #[test]
    fn forgejo_loader_rejects_conflicting_workflow_role_authors() {
        let dir = tempdir().unwrap();
        let workflow = valid_workflow("").replace(
            "      request_review:",
            &format!(
                "      alternate_review:\n        from: [todo]\n        to: review\n        actions:\n          - review.open:\n              provider: forgejo\n              role: worker\n              role_authors:\n                worker: forge-worker-alt\n                reviewer: forge-reviewer-alt\n                validator: forge-validator-alt\n                manager: forge-manager-alt\n      request_review:"
            ),
        );
        write_repo_config(dir.path(), &workflow);

        let error = load_forgejo_with_workflow_role_authors(dir.path())
            .unwrap_err()
            .to_string();

        assert!(error.contains("conflicting role_authors"));
    }

    #[test]
    fn parses_valid_forgejo_config_without_role_authors() {
        let config = parse_project_config(valid_config(), &path()).unwrap();
        let forgejo = config.require_forgejo(&path()).unwrap();

        assert_eq!(forgejo.host, "forge.example.test");
        assert_eq!(forgejo.owner, "tools");
        assert_eq!(forgejo.repo, "atelier");
        assert_eq!(forgejo.admin_token_env, "FORGEJO_ADMIN_TOKEN");
        assert_eq!(forgejo.role_authors, None);
        assert!(forgejo
            .role_author_for_role("worker")
            .unwrap_err()
            .to_string()
            .contains("workflow action params"));
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

[review]
mode = "room"
"#,
            &path(),
        )
        .unwrap();

        let error = config.require_forgejo(&path()).unwrap_err().to_string();

        assert!(error.contains("review_mode_invalid"));
        assert!(error.contains("review.mode = \"room\""));
    }

    #[test]
    fn invalid_forgejo_config_names_and_legacy_role_authors() {
        let old_role_authors = format!(
            "{}\n[review.providers.forgejo.role_authors]\nworker = \"atelier-worker\"\nreviewer = \"atelier-reviewer\"\nvalidator = \"atelier-validator\"\nmanager = \"atelier-manager\"\n",
            valid_config()
        );
        let error = parse_project_config(&old_role_authors, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("unknown field `role_authors`"));

        let old_sudo_users = valid_config().replace(
            "[review.providers.forgejo]",
            "[review.providers.forgejo.sudo_users]",
        );
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
        assert!(error.contains("review.providers.forgejo.admin_token_env"));
        assert!(error.contains("FORGEJO_ADMIN_TOKEN"));
    }

    #[test]
    fn rejects_committed_runtime_and_compatibility_path_settings() {
        let runtime_path_config = valid_config().replace(
            "state_root = \".atelier\"",
            "state_root = \".atelier\"\nruntime_dir = \".atelier/runtime\"\nruntime_database = \".atelier/runtime/state.db\"\ncache_dir = \".atelier/cache\"",
        );
        let error = parse_project_config(&runtime_path_config, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("unknown field `runtime_dir`"));

        let compatibility_config = valid_config().replace(
            "project_slug = \"atelier\"\n",
            "project_slug = \"atelier\"\ncompatibility_state_root = \".atelier-state\"\n",
        );
        let error = parse_project_config(&compatibility_config, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("unknown field `compatibility_state_root`"));
    }

    #[test]
    fn rejects_legacy_top_level_forgejo_config() {
        let legacy = valid_config()
            .replace(
                "[review]\nmode = \"provider\"\nprovider = \"forgejo\"\n\n[review.providers.forgejo]",
                "[forgejo]",
            )
            .replace(
                "[review.providers.forgejo.role_authors]",
                "[forgejo.role_authors]",
            );

        let error = parse_project_config(&legacy, &path())
            .unwrap_err()
            .to_string();

        assert!(error.contains("review_config_legacy_forgejo"));
        assert!(error.contains("[review.providers.forgejo]"));
    }

    #[test]
    fn rejects_mixed_room_and_provider_config() {
        let mixed = valid_config().replace("mode = \"provider\"", "mode = \"room\"");
        let error = parse_project_config(&mixed, &path())
            .unwrap_err()
            .to_string();

        assert!(error.contains("review_config_invalid"));
        assert!(error.contains("must not define provider settings"));
    }
}
