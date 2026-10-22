use std::fs;
use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;

use crate::storage_layout::StorageLayout;

const PROJECT_CONFIG_SCHEMA: &str = "atelier.project_config";
const PROJECT_CONFIG_SCHEMA_VERSION: i64 = 1;
pub const DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS: u64 = 7;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectConfig {
    pub project_slug: String,
    pub issue_links: IssueLinkConfig,
    pub prune: PruneConfig,
    pub review: ReviewConfig,
}

/// How reviews are requested.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ReviewConfig {
    /// Repository-relative Markdown file whose text `atelier review request`
    /// adds to every reviewer brief as the repository's working rules.
    pub rules_file: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, expecting = "struct RawIssueLinkConfig")]
pub struct IssueLinkConfig {
    #[serde(default)]
    pub custom_context_types: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PruneConfig {
    pub canonical_retention_days: u64,
}

impl ProjectConfig {
    pub fn load(repo_root: &Path) -> Result<Self> {
        Self::load_file(&StorageLayout::new(repo_root).config_path())
    }

    /// Load the config stored in a canonical state directory (`.atelier`).
    pub fn load_from_state_dir(state_dir: &Path) -> Result<Self> {
        Self::load_file(&state_dir.join(crate::storage_layout::CONFIG_FILE))
    }

    fn load_file(config_path: &Path) -> Result<Self> {
        let text = fs::read_to_string(config_path)
            .with_context(|| format!("failed to read {}", config_path.display()))?;
        parse_project_config(&text, config_path)
    }
}

/// A record store that mints IDs from the project's configured slug.
pub fn record_store(state_dir: &Path) -> Result<atelier_records::RecordStore> {
    let config = ProjectConfig::load_from_state_dir(state_dir)?;
    Ok(atelier_records::RecordStore::for_project(
        state_dir,
        config.project_slug,
    ))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProjectConfig {
    schema: String,
    schema_version: i64,
    project_slug: String,
    paths: RawProjectPaths,
    #[serde(default)]
    issue_links: Option<IssueLinkConfig>,
    #[serde(default)]
    prune: Option<RawPruneConfig>,
    #[serde(default)]
    review: Option<RawReviewConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReviewConfig {
    rules_file: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProjectPaths {
    state_root: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPruneConfig {
    canonical_retention_days: Option<u64>,
}

pub fn parse_project_config(text: &str, config_path: &Path) -> Result<ProjectConfig> {
    let raw = parse_raw_project_config(text, config_path)?;
    validate_schema_fields(&raw, config_path)?;
    require_non_empty(&raw.project_slug, config_path, "project_slug")?;
    atelier_records::validate_project_slug(&raw.project_slug)
        .with_context(|| format!("invalid project_slug in {}", config_path.display()))?;
    require_non_empty(&raw.paths.state_root, config_path, "paths.state_root")?;
    let issue_links = parse_issue_link_config(raw.issue_links, config_path)?;
    let prune = parse_prune_config(raw.prune, config_path)?;
    let review = parse_review_config(raw.review, config_path)?;
    Ok(ProjectConfig {
        project_slug: raw.project_slug,
        issue_links,
        prune,
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

fn parse_prune_config(raw: Option<RawPruneConfig>, config_path: &Path) -> Result<PruneConfig> {
    let canonical_retention_days = raw
        .and_then(|prune| prune.canonical_retention_days)
        .unwrap_or(DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS);
    if canonical_retention_days > 3650 {
        bail!(
            "project_config_invalid: {} field 'prune.canonical_retention_days' must be at most 3650",
            config_path.display()
        );
    }
    Ok(PruneConfig {
        canonical_retention_days,
    })
}

fn parse_review_config(raw: Option<RawReviewConfig>, config_path: &Path) -> Result<ReviewConfig> {
    let rules_file = raw.and_then(|review| review.rules_file);
    if let Some(path) = &rules_file {
        let relative = Path::new(path);
        let confined = !path.trim().is_empty()
            && relative.is_relative()
            && relative
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)));
        if !confined {
            bail!(
                "project_config_invalid: {} field 'review.rules_file' must be a path inside the repository, such as docs/review-rules.md",
                config_path.display()
            );
        }
    }
    Ok(ReviewConfig { rules_file })
}

fn parse_issue_link_config(
    raw: Option<IssueLinkConfig>,
    config_path: &Path,
) -> Result<IssueLinkConfig> {
    let mut config = raw.unwrap_or_default();
    config.custom_context_types.sort();
    config.custom_context_types.dedup();
    for relation_type in &config.custom_context_types {
        validate_custom_issue_link_type(relation_type, config_path)?;
    }
    Ok(config)
}

fn validate_custom_issue_link_type(relation_type: &str, config_path: &Path) -> Result<()> {
    let mut chars = relation_type.chars();
    let Some(first) = chars.next() else {
        bail!(
            "project_config_invalid: {} issue_links.custom_context_types entries cannot be empty",
            config_path.display()
        );
    };
    if !first.is_ascii_lowercase() {
        bail!(
            "project_config_invalid: {} issue link type '{}' must start with a lowercase ASCII letter",
            config_path.display(),
            relation_type
        );
    }
    if !chars
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '_' | '-' | '.'))
    {
        bail!(
            "project_config_invalid: {} issue link type '{}' may contain only lowercase ASCII letters, digits, '_', '-', or '.'",
            config_path.display(),
            relation_type
        );
    }
    Ok(())
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

[paths]
state_root = ".atelier"
"#
    }

    #[test]
    fn issue_link_defaults_normalize_and_keep_strict_parse_diagnostics() {
        for suffix in [
            "",
            "\n[issue_links]\n",
            "\n[issue_links]\ncustom_context_types = []\n",
        ] {
            let config =
                parse_project_config(&format!("{}{suffix}", valid_config()), &path()).unwrap();
            assert!(config.issue_links.custom_context_types.is_empty());
        }
        let config = parse_project_config(
            &format!(
                "{}\n[issue_links]\ncustom_context_types = [\"zz\", \"aa\", \"zz\"]\n",
                valid_config()
            ),
            &path(),
        )
        .unwrap();
        assert_eq!(config.issue_links.custom_context_types, ["aa", "zz"]);
        let text = valid_config().replace("[paths]", "issue_links = \"bad\"\n\n[paths]");
        let error = parse_project_config(&text, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("expected struct RawIssueLinkConfig"));
        let text = format!("{}\n[issue_links]\nunknown = true\n", valid_config());
        let error = parse_project_config(&text, &path())
            .unwrap_err()
            .to_string();
        assert!(error.contains("unknown field `unknown`, expected `custom_context_types`"));
    }

    #[test]
    fn parses_valid_config_with_default_prune_retention() {
        let config = parse_project_config(valid_config(), &path()).unwrap();

        assert_eq!(config.project_slug, "atelier");
        assert_eq!(
            config.prune.canonical_retention_days,
            DEFAULT_CANONICAL_PRUNE_RETENTION_DAYS
        );
    }

    #[test]
    fn parses_prune_canonical_retention_days() {
        let config = parse_project_config(
            &format!(
                "{}\n[prune]\ncanonical_retention_days = 14\n",
                valid_config()
            ),
            &path(),
        )
        .unwrap();

        assert_eq!(config.prune.canonical_retention_days, 14);
    }

    #[test]
    fn rejects_unreasonable_prune_retention_days() {
        let config = format!(
            "{}\n[prune]\ncanonical_retention_days = 3651\n",
            valid_config()
        );
        let error = parse_project_config(&config, &path())
            .unwrap_err()
            .to_string();

        assert!(error.contains("prune.canonical_retention_days"));
        assert!(error.contains("at most 3650"));
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
}
