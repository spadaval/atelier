use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnoredTestInventory {
    pub tests: Vec<IgnoredTest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgnoredTest {
    pub path: PathBuf,
    pub line: usize,
    pub name: String,
    pub raw_reason: Option<String>,
    pub metadata: IgnoreMetadata,
    pub problems: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IgnoreMetadata {
    pub reason: Option<String>,
    pub owner: Option<String>,
    pub issue: Option<String>,
    pub product_behavior: Option<bool>,
    pub blocking: Option<bool>,
}

impl IgnoredTestInventory {
    pub fn scan_repo(repo_root: &Path) -> Result<Self> {
        let mut tests = Vec::new();
        for root_name in ["src", "tests"] {
            let root = repo_root.join(root_name);
            if root.exists() {
                scan_dir(repo_root, &root, &mut tests)?;
            }
        }
        tests.sort_by(|a, b| a.path.cmp(&b.path).then_with(|| a.line.cmp(&b.line)));
        Ok(Self { tests })
    }

    pub fn blocking_findings(&self) -> Vec<&IgnoredTest> {
        self.tests
            .iter()
            .filter(|test| test.is_invalid() || test.is_product_behavior_blocker())
            .collect()
    }

    pub fn non_blocking_count(&self) -> usize {
        self.tests
            .len()
            .saturating_sub(self.blocking_findings().len())
    }

    pub fn status_reason(&self) -> (bool, String) {
        let blockers = self.blocking_findings();
        if blockers.is_empty() {
            return (
                true,
                format!(
                    "ignored test inventory reviewed {} ignored test(s); {} non-blocking",
                    self.tests.len(),
                    self.non_blocking_count()
                ),
            );
        }

        let mut lines = vec![format!(
            "ignored test inventory found {} blocker(s) across {} ignored test(s)",
            blockers.len(),
            self.tests.len()
        )];
        for test in blockers {
            lines.push(format!("  - {}", test.summary_line()));
            for problem in &test.problems {
                lines.push(format!("      problem: {problem}"));
            }
            if test.is_product_behavior_blocker() {
                lines.push(
                    "      problem: ignored product-behavior test is still blocking closeout"
                        .to_string(),
                );
            }
        }
        (false, lines.join("\n"))
    }
}

impl IgnoredTest {
    pub fn is_invalid(&self) -> bool {
        !self.problems.is_empty()
    }

    pub fn is_product_behavior_blocker(&self) -> bool {
        self.metadata.product_behavior == Some(true) && self.metadata.blocking != Some(false)
    }

    fn summary_line(&self) -> String {
        format!(
            "{}:{} {} reason={} owner={} issue={} product={} blocking={}",
            self.path.display(),
            self.line,
            self.name,
            quoted_or_missing(self.metadata.reason.as_deref()),
            quoted_or_missing(self.metadata.owner.as_deref()),
            quoted_or_missing(self.metadata.issue.as_deref()),
            bool_or_missing(self.metadata.product_behavior),
            bool_or_missing(self.metadata.blocking),
        )
    }
}

fn scan_dir(repo_root: &Path, dir: &Path, tests: &mut Vec<IgnoredTest>) -> Result<()> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("failed to read {}", dir.display()))?
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("failed to read entries in {}", dir.display()))?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to read file type for {}", path.display()))?;
        if file_type.is_dir() {
            scan_dir(repo_root, &path, tests)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            let relative = path
                .strip_prefix(repo_root)
                .unwrap_or(path.as_path())
                .to_path_buf();
            tests.extend(scan_file(&relative, &content));
        }
    }
    Ok(())
}

pub fn scan_file(path: &Path, content: &str) -> Vec<IgnoredTest> {
    let mut tests = Vec::new();
    let mut pending_ignore: Option<(usize, Option<String>)> = None;

    for (index, line) in content.lines().enumerate() {
        let line_number = index + 1;
        if let Some(reason) = ignore_attribute_reason(line) {
            pending_ignore = Some((line_number, reason));
            continue;
        }
        let Some((ignore_line, raw_reason)) = pending_ignore.clone() else {
            continue;
        };
        if let Some(name) = test_function_name(line) {
            let metadata = parse_ignore_metadata(raw_reason.as_deref());
            let problems = metadata_problems(raw_reason.as_deref(), &metadata);
            tests.push(IgnoredTest {
                path: path.to_path_buf(),
                line: ignore_line,
                name,
                raw_reason,
                metadata,
                problems,
            });
            pending_ignore = None;
        }
    }

    tests
}

fn ignore_attribute_reason(line: &str) -> Option<Option<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with("#[ignore") {
        return None;
    }
    if trimmed == "#[ignore]" {
        return Some(None);
    }
    let value = trimmed
        .strip_prefix("#[ignore")
        .and_then(|value| value.strip_suffix(']'))?
        .trim();
    let value = value.strip_prefix('=')?.trim();
    let quoted = value.strip_prefix('"')?.strip_suffix('"')?;
    Some(Some(unescape_rust_string(quoted)))
}

fn unescape_rust_string(value: &str) -> String {
    let json = format!("\"{value}\"");
    serde_json::from_str::<String>(&json).unwrap_or_else(|_| value.to_string())
}

fn test_function_name(line: &str) -> Option<String> {
    let start = line.find("fn ")? + 3;
    let name = line[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect::<String>();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn parse_ignore_metadata(raw_reason: Option<&str>) -> IgnoreMetadata {
    let Some(raw_reason) = raw_reason.map(str::trim).filter(|value| !value.is_empty()) else {
        return IgnoreMetadata::default();
    };

    let mut metadata = IgnoreMetadata::default();
    let mut saw_key_value = false;
    for part in raw_reason.split(';') {
        let Some((key, value)) = part.split_once(':') else {
            continue;
        };
        saw_key_value = true;
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        match key.as_str() {
            "reason" => metadata.reason = Some(value.to_string()),
            "owner" => metadata.owner = Some(value.to_string()),
            "issue" => metadata.issue = Some(value.to_string()),
            "product" | "product_behavior" | "product-behavior" => {
                metadata.product_behavior = parse_bool(value)
            }
            "blocking" => metadata.blocking = parse_bool(value),
            _ => {}
        }
    }

    if !saw_key_value {
        metadata.reason = Some(raw_reason.to_string());
    }
    metadata
}

fn metadata_problems(raw_reason: Option<&str>, metadata: &IgnoreMetadata) -> Vec<String> {
    let mut problems = Vec::new();
    if raw_reason
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .is_none()
    {
        problems.push("missing ignore reason".to_string());
    }
    if metadata
        .reason
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .is_none()
    {
        problems.push("missing reason metadata".to_string());
    }
    if metadata.owner.is_none() && metadata.issue.is_none() {
        problems.push("missing owner or linked issue".to_string());
    }
    if metadata.product_behavior.is_none() {
        problems.push("missing product behavior classification".to_string());
    }
    if metadata
        .issue
        .as_deref()
        .is_some_and(|issue| !is_record_id(issue))
    {
        problems.push("linked issue must look like an Atelier issue ID".to_string());
    }
    if metadata
        .reason
        .as_deref()
        .is_some_and(is_stale_or_obsolete_reason)
        && metadata.issue.is_none()
    {
        problems.push("stale or obsolete ignored test must link a follow-up issue".to_string());
    }
    problems
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "yes" | "true" => Some(true),
        "no" | "false" => Some(false),
        _ => None,
    }
}

fn is_stale_or_obsolete_reason(reason: &str) -> bool {
    let reason = reason.to_ascii_lowercase();
    ["stale", "obsolete", "legacy", "removed"]
        .iter()
        .any(|word| reason.contains(word))
}

fn is_record_id(value: &str) -> bool {
    let Some((prefix, suffix)) = value.rsplit_once('-') else {
        return false;
    };
    prefix == "atelier"
        && suffix.len() == 4
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}

fn quoted_or_missing(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("\"{value}\""),
        None => "(missing)".to_string(),
    }
}

fn bool_or_missing(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "(missing)",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignored_test_without_reason_or_owner_is_reported() {
        let fixture = concat!("\n#[test]\n", "#[", "ignore]\n", "fn hides_work() {}\n");
        let tests = scan_file(Path::new("tests/example.rs"), fixture);

        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].name, "hides_work");
        assert!(tests[0]
            .problems
            .contains(&"missing ignore reason".to_string()));
        assert!(tests[0]
            .problems
            .contains(&"missing owner or linked issue".to_string()));
        assert!(tests[0]
            .problems
            .contains(&"missing product behavior classification".to_string()));
    }

    #[test]
    fn ignored_product_behavior_defaults_to_closeout_blocker() {
        let fixture = concat!(
            "\n#[test]\n",
            "#[",
            "ignore = \"reason: waiting on parser migration; issue: atelier-ab12; product: yes\"]\n",
            "fn product_behavior_gap() {}\n"
        );
        let tests = scan_file(Path::new("tests/example.rs"), fixture);

        assert_eq!(tests.len(), 1);
        assert!(tests[0].problems.is_empty());
        assert!(tests[0].is_product_behavior_blocker());
        let inventory = IgnoredTestInventory { tests };
        let (passed, reason) = inventory.status_reason();
        assert!(!passed);
        assert!(reason.contains("product_behavior_gap"));
        assert!(reason.contains("ignored product-behavior test"));
    }

    #[test]
    fn extended_product_behavior_can_be_explicitly_non_blocking() {
        let fixture = concat!(
            "\n#[test]\n",
            "#[",
            "ignore = \"reason: extended property test; owner: quality; product: yes; blocking: no\"]\n",
            "fn prop_extended_ready_list() {}\n"
        );
        let tests = scan_file(Path::new("src/db/proptest_tests.rs"), fixture);

        assert_eq!(tests.len(), 1);
        assert!(tests[0].problems.is_empty());
        assert!(!tests[0].is_product_behavior_blocker());
        let inventory = IgnoredTestInventory { tests };
        let (passed, reason) = inventory.status_reason();
        assert!(passed);
        assert!(reason.contains("1 ignored test(s); 1 non-blocking"));
    }

    #[test]
    fn obsolete_ignored_test_requires_follow_up_issue() {
        let fixture = concat!(
            "\n#[test]\n",
            "#[",
            "ignore = \"reason: obsolete command behavior; owner: cli; product: no; blocking: no\"]\n",
            "fn stale_assertion() {}\n"
        );
        let tests = scan_file(Path::new("tests/example.rs"), fixture);

        assert_eq!(tests.len(), 1);
        assert!(tests[0]
            .problems
            .contains(&"stale or obsolete ignored test must link a follow-up issue".to_string()));
    }
}
