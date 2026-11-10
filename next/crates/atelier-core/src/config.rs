use crate::SchemaVersion;
use serde::{Deserialize, Serialize};

/// `.atelier/config.toml`. `merge` reads it from the base branch's tip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: SchemaVersion,
    /// The prefix of every item and mission ID.
    pub slug: String,
    pub base_branch: String,
    /// Commands `merge` runs on the merged tree, in order.
    #[serde(rename = "merge_check", default, skip_serializing_if = "Vec::is_empty")]
    pub merge_checks: Vec<MergeCheck>,
    pub forge: Option<Forge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MergeCheck {
    pub run: String,
}

/// Where `merge` pushes the branch, opens its pull request and pushes the
/// base.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Forge {
    pub provider: ForgeProvider,
    pub remote: String,
    /// The workflow file whose run on the pull request must pass; without
    /// one nothing waits for CI.
    pub ci_workflow: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForgeProvider {
    Github,
}
