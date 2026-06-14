use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const GUIDANCE_FILES: &[&str] = &[
    "AGENTS.md",
    "AGENTFACTORY.md",
    "docs/product/cli-surface.md",
    "docs/product/work-model.md",
    "docs/architecture/quality/validation.md",
];

const CLI_SURFACE_DOC: &str = "docs/product/cli-surface.md";

const COMMAND_GROUP_ROOTS: &[&str] = &[
    "diagnostics",
    "evidence",
    "graph",
    "issue",
    "maintenance",
    "mission",
    "note",
    "plan",
    "workflow",
    "worktree",
];

const REMOVED_ROOTS: &[&str] = &[
    "agent",
    "archive",
    "cpitd",
    "daemon",
    "integrations",
    "link",
    "locks",
    "milestone",
    "session",
    "sync",
    "timer",
    "usage",
    "work",
];

const REMOVED_COMMAND_PATHS: &[&[&str]] = &[
    &["evidence", "add"],
    &["evidence", "capture"],
    &["mission", "view"],
    &["workflow", "init"],
    &["work", "worktree"],
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftFinding {
    pub path: PathBuf,
    pub line: usize,
    pub subject: String,
    pub message: String,
}

impl DriftFinding {
    fn summary_line(&self) -> String {
        let location = if self.line == 0 {
            self.path.display().to_string()
        } else {
            format!("{}:{}", self.path.display(), self.line)
        };
        format!("{location} {} - {}", self.subject, self.message)
    }
}

#[derive(Debug, Default)]
pub struct CommandSurfaceReport {
    pub findings: Vec<DriftFinding>,
    pub checked_command_refs: usize,
    pub checked_test_refs: usize,
    pub checked_help_roots: usize,
}

impl CommandSurfaceReport {
    pub fn status_reason(&self) -> (bool, String) {
        if self.findings.is_empty() {
            return (
                true,
                format!(
                    "docs/help/Agent Factory command surface reviewed: {} guidance command reference(s), {} visible help command(s), {} obsolete-test command reference(s)",
                    self.checked_command_refs, self.checked_help_roots, self.checked_test_refs
                ),
            );
        }

        let mut lines = vec![format!(
            "command-surface drift found {} finding(s)",
            self.findings.len()
        )];
        for finding in self.findings.iter().take(12) {
            lines.push(format!("  - {}", finding.summary_line()));
        }
        if self.findings.len() > 12 {
            lines.push(format!(
                "  - ... and {} more finding(s)",
                self.findings.len() - 12
            ));
        }
        (false, lines.join("\n"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CommandUse {
    display: String,
    path: Vec<String>,
    options: Vec<String>,
}

struct HelpCatalog {
    exe: PathBuf,
    cache: BTreeMap<Vec<String>, Option<String>>,
    visible_roots: BTreeSet<String>,
}

impl HelpCatalog {
    fn new() -> Result<Self> {
        let exe = std::env::current_exe().context("failed to locate current atelier binary")?;
        let root_help = run_help(&exe, &[]).context("failed to run `atelier --help`")?;
        let visible_roots = parse_root_help_commands(&root_help);
        let mut cache = BTreeMap::new();
        cache.insert(Vec::new(), Some(root_help));
        Ok(Self {
            exe,
            cache,
            visible_roots,
        })
    }

    fn help_for(&mut self, path: &[String]) -> Result<Option<String>> {
        if let Some(help) = self.cache.get(path) {
            return Ok(help.clone());
        }

        let help = run_help(&self.exe, path).ok();
        self.cache.insert(path.to_vec(), help.clone());
        Ok(help)
    }

    fn command_exists(&mut self, path: &[String]) -> Result<bool> {
        if path.len() == 1 && path[0] == "help" {
            return Ok(true);
        }
        Ok(self.help_for(path)?.is_some())
    }

    fn option_exists(&mut self, path: &[String], option: &str) -> Result<bool> {
        if path.len() == 1 && path[0] == "help" {
            return Ok(true);
        }
        Ok(self
            .help_for(path)?
            .is_some_and(|help| help.contains(option)))
    }
}

pub fn status_reason(repo_root: &Path) -> Result<(bool, String)> {
    Ok(scan_repo(repo_root)?.status_reason())
}

pub fn scan_repo(repo_root: &Path) -> Result<CommandSurfaceReport> {
    let mut catalog = HelpCatalog::new()?;
    let mut report = CommandSurfaceReport {
        checked_help_roots: catalog.visible_roots.len(),
        ..CommandSurfaceReport::default()
    };

    for relative in GUIDANCE_FILES {
        let path = repo_root.join(relative);
        if path.exists() {
            scan_guidance_file(repo_root, &path, &mut catalog, &mut report)?;
        }
    }

    compare_cli_surface_to_help(repo_root, &mut catalog, &mut report)?;
    scan_rust_tests(repo_root, &mut report)?;
    Ok(report)
}

fn scan_guidance_file(
    repo_root: &Path,
    path: &Path,
    catalog: &mut HelpCatalog,
    report: &mut CommandSurfaceReport,
) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let relative = relative_path(repo_root, path);
    let mut section = String::new();

    for (index, line) in content.lines().enumerate() {
        let line_number = index + 1;
        if let Some(title) = markdown_heading(line) {
            section = title;
        }
        for raw in command_spans(line) {
            if removed_or_deferred_context(&section, line) {
                continue;
            }
            for command in expand_command_reference(&raw) {
                report.checked_command_refs += 1;
                if !catalog.command_exists(&command.path)? {
                    report.findings.push(DriftFinding {
                        path: relative.clone(),
                        line: line_number,
                        subject: format!("command `{}`", command.display),
                        message: format!(
                            "guidance references a command path that is not implemented; update {} or the command surface",
                            relative.display()
                        ),
                    });
                    continue;
                }
                for option in &command.options {
                    if !catalog.option_exists(&command.path, option)? {
                        report.findings.push(DriftFinding {
                            path: relative.clone(),
                            line: line_number,
                            subject: format!("command `{}`", command.display),
                            message: format!(
                                "guidance references option `{option}` that is not present in help for `atelier {}`",
                                command.path.join(" ")
                            ),
                        });
                    }
                }
            }
        }
    }
    Ok(())
}

fn compare_cli_surface_to_help(
    repo_root: &Path,
    catalog: &mut HelpCatalog,
    report: &mut CommandSurfaceReport,
) -> Result<()> {
    let path = repo_root.join(CLI_SURFACE_DOC);
    if !path.exists() {
        return Ok(());
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let documented = documented_visible_roots(&content);
    let relative = PathBuf::from(CLI_SURFACE_DOC);

    for root in &catalog.visible_roots {
        if !documented.contains_key(root) {
            report.findings.push(DriftFinding {
                path: relative.clone(),
                line: 0,
                subject: format!("help command `atelier {root}`"),
                message: "root help lists this visible command, but the CLI surface doc does not document it".to_string(),
            });
        }
    }

    for (root, line) in documented {
        if !catalog.visible_roots.contains(&root) {
            report.findings.push(DriftFinding {
                path: relative.clone(),
                line,
                subject: format!("documented command `atelier {root}`"),
                message: "CLI surface doc lists this as visible, but `atelier --help` does not"
                    .to_string(),
            });
        }
    }

    Ok(())
}

fn scan_rust_tests(repo_root: &Path, report: &mut CommandSurfaceReport) -> Result<()> {
    for root_name in ["src", "tests"] {
        let root = repo_root.join(root_name);
        if root.exists() {
            scan_rust_test_dir(repo_root, &root, report)?;
        }
    }
    Ok(())
}

fn scan_rust_test_dir(
    repo_root: &Path,
    dir: &Path,
    report: &mut CommandSurfaceReport,
) -> Result<()> {
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
            scan_rust_test_dir(repo_root, &path, report)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            scan_rust_test_file(repo_root, &path, report)?;
        }
    }
    Ok(())
}

fn scan_rust_test_file(
    repo_root: &Path,
    path: &Path,
    report: &mut CommandSurfaceReport,
) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let relative = relative_path(repo_root, path);
    for test in rust_tests(&content) {
        let obsolete_refs = obsolete_run_atelier_commands(&test.body);
        if obsolete_refs.is_empty() {
            continue;
        }
        report.checked_test_refs += obsolete_refs.len();
        if test.has_explicit_migration_window() || test.is_negative_or_replacement_test() {
            continue;
        }
        for (offset_line, command) in obsolete_refs {
            report.findings.push(DriftFinding {
                path: relative.clone(),
                line: test.start_line + offset_line,
                subject: format!("test `{}`", test.name),
                message: format!(
                    "asserts obsolete command `{command}` as normal behavior; migrate the test or add explicit owner/issue/blocking:false migration metadata"
                ),
            });
        }
    }
    Ok(())
}

fn documented_visible_roots(content: &str) -> BTreeMap<String, usize> {
    let mut roots = BTreeMap::new();
    let mut section = String::new();
    for (index, line) in content.lines().enumerate() {
        if let Some(title) = markdown_heading(line) {
            section = title;
        }
        if removed_or_deferred_context(&section, line) || hidden_context(line) {
            continue;
        }
        for raw in command_spans(line) {
            for command in expand_command_reference(&raw) {
                let Some(root) = command.path.first() else {
                    continue;
                };
                if root == "help" || root == "workflow" {
                    continue;
                }
                roots.entry(root.clone()).or_insert(index + 1);
            }
        }
    }
    roots
}

fn run_help(exe: &Path, path: &[String]) -> Result<String> {
    let output = Command::new(exe)
        .args(path)
        .arg("--help")
        .output()
        .with_context(|| format!("failed to run help for `atelier {}`", path.join(" ")))?;
    if !output.status.success() {
        anyhow::bail!("help failed for `atelier {}`", path.join(" "));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_root_help_commands(help: &str) -> BTreeSet<String> {
    let visible_sections = BTreeSet::from([
        "Setup",
        "Orientation",
        "Issues",
        "Missions and planning",
        "Records",
        "Advanced work",
        "State management",
        "Maintenance",
    ]);
    let mut current_visible_section = false;
    let mut commands = BTreeSet::new();
    for line in help.lines() {
        if !line.starts_with(' ') && line.ends_with(':') {
            let heading = line.trim_end_matches(':');
            current_visible_section = visible_sections.contains(heading);
            continue;
        }
        if !current_visible_section {
            continue;
        }
        let trimmed = line.trim_start();
        let Some(command) = trimmed.split_whitespace().next() else {
            continue;
        };
        if is_command_word(command) {
            commands.insert(command.to_string());
        }
    }
    commands
}

fn markdown_heading(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return None;
    }
    Some(trimmed.trim_start_matches('#').trim().to_string())
}

fn command_spans(line: &str) -> Vec<String> {
    let mut spans = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('`') else {
            break;
        };
        let span = &rest[..end];
        if span.trim_start().starts_with("atelier ") {
            spans.push(span.trim().to_string());
        }
        rest = &rest[end + 1..];
    }
    spans
}

fn expand_command_reference(raw: &str) -> Vec<CommandUse> {
    let Some((_, after_atelier)) = raw.split_once("atelier") else {
        return Vec::new();
    };
    let tokens = tokenize_command(after_atelier);
    if tokens.is_empty() {
        return Vec::new();
    }
    if tokens
        .first()
        .is_some_and(|token| token.starts_with("--") || token.starts_with('<'))
    {
        return Vec::new();
    }

    let mut uses = BTreeSet::new();
    for expanded in expand_slash_tokens(&tokens) {
        let path_len = intended_command_path_len(&expanded);
        if path_len == 0 {
            continue;
        }
        let path = expanded[..path_len].to_vec();
        let options = expanded
            .iter()
            .filter_map(|token| option_name(token))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let display = if options.is_empty() {
            format!("atelier {}", path.join(" "))
        } else {
            format!("atelier {} {}", path.join(" "), options.join(" "))
        };
        uses.insert(CommandUse {
            display,
            path,
            options,
        });
    }
    uses.into_iter().collect()
}

fn tokenize_command(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .map(clean_token)
        .filter(|token| !token.is_empty())
        .take_while(|token| token != "and")
        .collect()
}

fn clean_token(token: &str) -> String {
    token
        .trim_matches(|ch: char| {
            matches!(
                ch,
                '`' | '"' | '\'' | '(' | ')' | '[' | ']' | ',' | '.' | ';' | ':'
            )
        })
        .to_string()
}

fn expand_slash_tokens(tokens: &[String]) -> Vec<Vec<String>> {
    let mut expanded = vec![Vec::new()];
    for token in tokens {
        let parts = slash_alternatives(token);
        let mut next = Vec::new();
        for prefix in &expanded {
            for part in &parts {
                let mut candidate = prefix.clone();
                candidate.push(part.clone());
                next.push(candidate);
            }
        }
        expanded = next;
    }
    expanded
}

fn slash_alternatives(token: &str) -> Vec<String> {
    if token.contains('/')
        && token
            .split('/')
            .all(|part| !part.is_empty() && is_command_word(part))
    {
        token.split('/').map(ToOwned::to_owned).collect()
    } else {
        vec![token.to_string()]
    }
}

fn intended_command_path_len(tokens: &[String]) -> usize {
    let Some(root) = tokens.first() else {
        return 0;
    };
    if root == "help" {
        return 1;
    }
    if COMMAND_GROUP_ROOTS.contains(&root.as_str())
        && tokens.get(1).is_some_and(|token| is_command_word(token))
    {
        return 2;
    }
    1
}

fn option_name(token: &str) -> Option<String> {
    if !token.starts_with("--") || token == "--" {
        return None;
    }
    Some(
        token
            .split_once('=')
            .map(|(name, _)| name)
            .unwrap_or(token)
            .to_string(),
    )
}

fn is_command_word(token: &str) -> bool {
    !token.is_empty()
        && token != "..."
        && !token.starts_with('-')
        && !token.starts_with('<')
        && token
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
}

fn removed_or_deferred_context(section: &str, line: &str) -> bool {
    let section = section.to_ascii_lowercase();
    let line = line.to_ascii_lowercase();
    section.contains("removed")
        || section.contains("deferred")
        || line.contains("there is no `atelier")
        || line.contains("no `atelier")
        || line.contains("not part of the normal")
        || line.contains("not part of the target")
        || line.contains("should not appear")
        || line.contains("removed command")
}

fn hidden_context(line: &str) -> bool {
    let line = line.to_ascii_lowercase();
    line.contains("hidden `atelier") || line.contains("not need the hidden")
}

#[derive(Debug)]
struct RustTest {
    name: String,
    start_line: usize,
    ignore_reason: Option<String>,
    body: String,
}

impl RustTest {
    fn has_explicit_migration_window(&self) -> bool {
        let Some(reason) = self.ignore_reason.as_deref() else {
            return false;
        };
        let reason = reason.to_ascii_lowercase();
        (reason.contains("obsolete")
            || reason.contains("legacy")
            || reason.contains("removed")
            || reason.contains("stale"))
            && reason.contains("issue:")
            && (reason.contains("owner:") || reason.contains("blocking: no"))
    }

    fn is_negative_or_replacement_test(&self) -> bool {
        let text = format!(
            "{}\n{}",
            self.name.to_ascii_lowercase(),
            self.body.to_ascii_lowercase()
        );
        text.contains("replacement")
            || text.contains("removed")
            || text.contains("unknown")
            || text.contains("reject")
            || text.contains("fail")
            || text.contains("hides")
            || text.contains("assert!(!success")
            || text.contains("unexpectedly succeeded")
    }
}

fn rust_tests(content: &str) -> Vec<RustTest> {
    let lines = content.lines().collect::<Vec<_>>();
    let mut starts = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if lines[index].trim() != "#[test]" {
            index += 1;
            continue;
        }
        let attr_line = index + 1;
        let mut cursor = index + 1;
        let mut ignore_reason = None;
        while cursor < lines.len() {
            let trimmed = lines[cursor].trim();
            if trimmed.starts_with("#[ignore") {
                ignore_reason = ignore_attribute_reason(trimmed);
                cursor += 1;
                continue;
            }
            if let Some(name) = test_function_name(trimmed) {
                starts.push((attr_line, cursor + 1, name, ignore_reason));
                break;
            }
            if !trimmed.starts_with("#[") && !trimmed.is_empty() {
                break;
            }
            cursor += 1;
        }
        index += 1;
    }

    let mut tests = Vec::new();
    for (position, (_attr_line, fn_line, name, ignore_reason)) in starts.iter().enumerate() {
        let next_attr_line = starts
            .get(position + 1)
            .map(|(line, _, _, _)| *line)
            .unwrap_or(lines.len() + 1);
        let start_index = fn_line.saturating_sub(1);
        let end_index = next_attr_line.saturating_sub(1).min(lines.len());
        tests.push(RustTest {
            name: name.clone(),
            start_line: *fn_line,
            ignore_reason: ignore_reason.clone(),
            body: lines[start_index..end_index].join("\n"),
        });
    }
    tests
}

fn ignore_attribute_reason(line: &str) -> Option<String> {
    let first = line.find('"')?;
    let last = line.rfind('"')?;
    if last <= first {
        return None;
    }
    Some(line[first + 1..last].to_string())
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

fn obsolete_run_atelier_commands(body: &str) -> Vec<(usize, String)> {
    let mut refs = Vec::new();
    let mut offset = 0;
    while let Some(found) = body[offset..].find("run_atelier") {
        let call_start = offset + found;
        let Some(array_start_relative) = body[call_start..].find("&[") else {
            offset = call_start + "run_atelier".len();
            continue;
        };
        let array_start = call_start + array_start_relative;
        let Some(array_end_relative) = body[array_start..].find(']') else {
            offset = call_start + "run_atelier".len();
            continue;
        };
        let array_end = array_start + array_end_relative + 1;
        let strings = string_literals(&body[array_start..array_end]);
        if let Some(command) = obsolete_command(&strings) {
            let line = body[..call_start].lines().count();
            refs.push((line, command));
        }
        offset = array_end;
    }
    refs
}

fn string_literals(value: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'"' {
            index += 1;
            continue;
        }
        index += 1;
        let start = index;
        let mut escaped = false;
        while index < bytes.len() {
            let byte = bytes[index];
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                break;
            }
            index += 1;
        }
        if index <= bytes.len() {
            literals.push(value[start..index].to_string());
        }
        index += 1;
    }
    literals
}

fn obsolete_command(tokens: &[String]) -> Option<String> {
    let root = tokens.first()?;
    if REMOVED_ROOTS.contains(&root.as_str()) {
        return Some(format!(
            "atelier {}",
            tokens.iter().take(2).cloned().collect::<Vec<_>>().join(" ")
        ));
    }
    for path in REMOVED_COMMAND_PATHS {
        if tokens.len() >= path.len()
            && path
                .iter()
                .enumerate()
                .all(|(index, expected)| tokens[index] == *expected)
        {
            return Some(format!("atelier {}", path.join(" ")));
        }
    }
    None
}

fn relative_path(repo_root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(repo_root).unwrap_or(path).to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_slash_command_references() {
        let uses = expand_command_reference("atelier mission create/show/list/status/update");
        let displays = uses
            .into_iter()
            .map(|command| command.display)
            .collect::<Vec<_>>();

        assert!(displays.contains(&"atelier mission create".to_string()));
        assert!(displays.contains(&"atelier mission show".to_string()));
        assert!(displays.contains(&"atelier mission update".to_string()));
    }

    #[test]
    fn extracts_visible_roots_without_removed_or_hidden_sections() {
        let doc = "# CLI\n\n## Core\n\n- `atelier status`\n- `atelier mission show`\n\n## Removed Behavior\n\nThere is no `atelier work status`.\nThere is no `atelier mission close`.\n";
        let roots = documented_visible_roots(doc);

        assert!(roots.contains_key("status"));
        assert!(roots.contains_key("mission"));
        assert!(!roots.contains_key("work"));
    }

    #[test]
    fn obsolete_test_command_requires_metadata_or_negative_intent() {
        let content = concat!(
            "#[test]\n",
            "fn legacy_still_works() {\n",
            "    let (success, _, _) = run_atelier(dir.path(), &[\"session\", \"start\"]);\n",
            "    assert!(success);\n",
            "}\n",
            "#[test]\n",
            "#[ignore = \"reason: obsolete command migration; owner: cli; issue: atelier-ab12; product: no; blocking: no\"]\n",
            "fn legacy_deferred() {\n",
            "    let (success, _, _) = run_atelier(dir.path(), &[\"session\", \"start\"]);\n",
            "    assert!(success);\n",
            "}\n",
            "#[test]\n",
            "fn removed_command_fails() {\n",
            "    let (success, _, _) = run_atelier(dir.path(), &[\"session\", \"start\"]);\n",
            "    assert!(!success);\n",
            "}\n"
        );
        let tests = rust_tests(content);

        let first_refs = obsolete_run_atelier_commands(&tests[0].body);
        assert_eq!(first_refs[0].1, "atelier session start");
        assert!(!tests[0].has_explicit_migration_window());
        assert!(!tests[0].is_negative_or_replacement_test());
        assert!(tests[1].has_explicit_migration_window());
        assert!(tests[2].is_negative_or_replacement_test());
    }
}
