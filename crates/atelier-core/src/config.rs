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
    /// Put before the name of every branch the convention names; a valid
    /// Git ref component ending in `/`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_prefix: Option<String>,
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

/// What a branch the convention names is for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Named {
    /// An item's or a mission's branch, named for its ID.
    Record(String),
    /// A `backlog/<slug>` branch, for record changes tied to no item or
    /// mission.
    Backlog(String),
}

impl Config {
    fn prefix(&self) -> &str {
        self.branch_prefix.as_deref().unwrap_or("")
    }

    /// The branch named for the item or mission `id`.
    pub fn record_branch(&self, id: &str) -> String {
        format!("{}{id}", self.prefix())
    }

    /// The backlog branch `backlog/<slug>`.
    pub fn backlog_branch(&self, slug: &str) -> String {
        format!("{}backlog/{slug}", self.prefix())
    }

    /// The branch that names `named`.
    pub fn branch(&self, named: &Named) -> String {
        match named {
            Named::Record(id) => self.record_branch(id),
            Named::Backlog(slug) => self.backlog_branch(slug),
        }
    }

    /// What the branch `branch` is for, when the convention names it: less
    /// the prefix, an ID of this repository's slug, or `backlog/<slug>`.
    /// The one reading of a branch name, which [`Config::branch`] inverts.
    pub fn named(&self, branch: &str) -> Option<Named> {
        let rest = branch.strip_prefix(self.prefix())?;
        if let Some(slug) = rest.strip_prefix("backlog/") {
            return validate_slug(slug)
                .is_ok()
                .then(|| Named::Backlog(slug.to_string()));
        }
        let suffix = rest.strip_prefix(&self.slug)?.strip_prefix('-')?;
        let id = !suffix.is_empty()
            && suffix
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
        id.then(|| Named::Record(rest.to_string()))
    }

    /// The ID of the item or mission whose branch is `branch`, if it is one.
    pub fn record_of(&self, branch: &str) -> Option<String> {
        match self.named(branch)? {
            Named::Record(id) => Some(id),
            Named::Backlog(_) => None,
        }
    }
}

/// Lowercase ASCII letters, digits and inner single hyphens: a
/// repository's slug, and a backlog branch's.
pub fn validate_slug(slug: &str) -> Result<(), String> {
    if slug.is_empty()
        || slug.starts_with('-')
        || slug.ends_with('-')
        || slug.contains("--")
        || !slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(format!(
            "`{slug}` is not a slug: lowercase letters, digits and single inner hyphens"
        ));
    }
    Ok(())
}

/// A branch prefix: one or more Git ref components, each ended by `/`.
pub fn validate_prefix(prefix: &str) -> Result<(), String> {
    let refused = || {
        Err(format!(
            "`{prefix}` is not a branch prefix: one or more parts of a Git branch name, of letters, digits, `-`, `_` and `.`, each ended by `/`, such as `x/`"
        ))
    };
    let Some(parts) = prefix.strip_suffix('/') else {
        return refused();
    };
    let valid = parts.split('/').all(|part| {
        !part.is_empty()
            && !part.starts_with('.')
            && !part.starts_with('-')
            && !part.ends_with(".lock")
            && !part.contains("..")
            && part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    });
    match valid {
        true => Ok(()),
        false => refused(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(prefix: Option<&str>) -> Config {
        Config {
            schema_version: crate::SchemaVersion,
            slug: "demo".into(),
            base_branch: "main".into(),
            branch_prefix: prefix.map(str::to_string),
            merge_checks: Vec::new(),
            forge: None,
        }
    }

    #[test]
    fn a_branch_name_reads_back_into_what_it_names_with_and_without_a_prefix() {
        for prefix in [None, Some("x/"), Some("team/sumeet/")] {
            let config = config(prefix);
            let p = prefix.unwrap_or("");
            for named in [
                Named::Record("demo-ab12".into()),
                Named::Backlog("file-bugs".into()),
            ] {
                let branch = config.branch(&named);
                assert_eq!(config.named(&branch), Some(named.clone()), "{branch}");
            }
            assert_eq!(config.record_branch("demo-ab12"), format!("{p}demo-ab12"));
            assert_eq!(config.backlog_branch("b"), format!("{p}backlog/b"));
            assert_eq!(
                config.record_of(&format!("{p}demo-ab12")),
                Some("demo-ab12".into())
            );
        }
    }

    #[test]
    fn a_branch_the_convention_does_not_name_reads_as_nothing() {
        let plain = config(None);
        for branch in [
            "main",
            "demo-",
            "other-ab12",
            "demo-AB12",
            "backlog/",
            "backlog/Bad",
            "feature/x",
        ] {
            assert_eq!(plain.named(branch), None, "{branch}");
        }
        let prefixed = config(Some("x/"));
        // Without its prefix a name is not the convention's.
        for branch in ["demo-ab12", "backlog/file", "y/demo-ab12"] {
            assert_eq!(prefixed.named(branch), None, "{branch}");
        }
    }

    #[test]
    fn a_prefix_is_ref_components_each_ended_by_a_slash() {
        for good in ["x/", "team/sumeet/", "a.b_c-d/"] {
            assert_eq!(validate_prefix(good), Ok(()), "{good}");
        }
        for bad in [
            "", "x", "/x/", "x//", ".x/", "-/", "a/-b/", "x..y/", "a b/", "x.lock/", "x:y/",
        ] {
            assert!(validate_prefix(bad).is_err(), "{bad}");
        }
    }
}
