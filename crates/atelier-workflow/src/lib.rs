use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_yaml::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use atelier_core::{Issue, IssueStrategy, ISSUE_STRATEGY_FIELD};

pub const STARTER_POLICY_YAML: &str = r#"schema: atelier.workflow
schema_version: 3
branch_policy:
  base_branch: main
  merge_strategy: fast_forward_only
issue_types:
  bug:
    label: Bug
  decision:
    label: Decision
  feature:
    label: Feature
  mission:
    label: Mission
  spike:
    label: Spike
  task:
    label: Task
  validation:
    label: Validation
statuses:
  draft:
    category: todo
  backlog:
    category: todo
  todo:
    category: todo
  in_progress:
    category: active
  blocked:
    category: blocked
  review:
    category: active
  done:
    category: done
  superseded:
    category: done
workflows:
  objective:
    applies_to:
    - mission
    initial_status: draft
    done_statuses:
    - done
    - superseded
    transitions:
      ready:
        from: [draft]
        to: todo
        description: The outcome, immediate work, and proof route are clear enough to execute.
        validators:
        - issue.sections_parseable
      start:
        from: [todo, blocked]
        to: in_progress
        description: Start work.
        validators:
        - blockers.transitive_none_open
      block:
        from: [todo, in_progress, review]
        to: blocked
        description: Record a blocker.
      request_review:
        from: [in_progress]
        to: review
        description: Ready for independent review.
      revise:
        from: [review]
        to: in_progress
        description: Return to active work after review findings.
      close:
        from: [in_progress, review]
        to: done
        required_fields: [close_reason]
        description: Done claim; commit it on the work branch before review.
        validators:
        - blockers.none_open
      supersede:
        from: [draft, todo, blocked, in_progress]
        to: superseded
        required_fields: [close_reason]
        description: Retire work that evidence made obsolete.
  work:
    applies_to:
    - bug
    - feature
    - spike
    - task
    - validation
    initial_status: todo
    done_statuses:
    - done
    - superseded
    transitions:
      plan:
        from: [backlog]
        to: todo
        description: Pull someday work into the near-term plan.
      defer:
        from: [todo, blocked]
        to: backlog
        description: Park work that is recorded but not near-term.
      start:
        from: [todo, blocked]
        to: in_progress
        description: Start work.
        validators:
        - blockers.transitive_none_open
      block:
        from: [todo, in_progress, review]
        to: blocked
        description: Record a blocker.
      request_review:
        from: [in_progress]
        to: review
        description: Ready for independent review.
      revise:
        from: [review]
        to: in_progress
        description: Return to active work after review findings.
      close:
        from: [in_progress, review]
        to: done
        required_fields: [close_reason]
        description: Done claim; commit it on the work branch before review. A parent closes after its children.
        validators:
        - blockers.none_open
        - work.terminal
      supersede:
        from: [backlog, todo, blocked, in_progress]
        to: superseded
        required_fields: [close_reason]
        description: Retire work that is no longer needed.
  decision:
    applies_to:
    - decision
    initial_status: todo
    done_statuses:
    - done
    - superseded
    transitions:
      decide:
        from: [todo, blocked]
        to: done
        required_fields: [close_reason]
        description: Record the decision. Decisions need no branch and may close on the base branch as a planning change.
        validators:
        - decision.recorded
        - blockers.none_open
        - work.terminal
      block:
        from: [todo]
        to: blocked
        description: Record a blocker.
      supersede:
        from: [todo, blocked]
        to: superseded
        required_fields: [close_reason]
        description: Retire a question that no longer needs deciding.
"#;

pub const WORKFLOW_POLICY_PATH: &str = ".atelier/workflow.yaml";
const WORKFLOW_SCHEMA: &str = "atelier.workflow";
const WORKFLOW_SCHEMA_VERSION: i64 = 3;
const STATUS_CATEGORIES: &[&str] = &["todo", "active", "blocked", "done"];
const BUILTIN_VALIDATORS: &[&str] = &[
    "issue.sections_parseable",
    "blockers.none_open",
    "blockers.transitive_none_open",
    "evidence.attached",
    "lint.none_blocking",
    "git.worktree_clean",
    "work.terminal",
    "decision.recorded",
    "shell.check",
];
const ALLOWED_REQUIRED_FIELDS: &[&str] = &["close_reason"];
const DEFERRED_TOP_LEVEL_FIELDS: &[&str] = &[
    "hooks",
    "triggers",
    "post_functions",
    "post-functions",
    "waivers",
];
const TOP_LEVEL_FIELDS: &[&str] = &[
    "schema",
    "schema_version",
    "branch_policy",
    "issue_types",
    "statuses",
    "workflows",
    "merge_checks",
];
#[derive(Debug, Clone)]
pub struct WorkflowPolicy {
    pub merge_checks: Vec<ValidatorDefinition>,
    pub schema_version: i64,
    pub branch_policy: BranchLifecycleConfig,
    pub issue_types: BTreeMap<String, IssueTypeDefinition>,
    pub workflow_by_issue_type: BTreeMap<String, String>,
    pub statuses: BTreeMap<String, StatusDefinition>,
    pub workflows: BTreeMap<String, WorkflowDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchLifecycleConfig {
    pub base_branch: String,
    pub merge_strategy: MergeStrategy,
}

impl Default for BranchLifecycleConfig {
    fn default() -> Self {
        Self {
            base_branch: "main".to_string(),
            merge_strategy: MergeStrategy::FastForwardOnly,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    Squash,
    MergeCommit,
    FastForwardOnly,
}

/// Issue field recording the branch an owner (a parent or standalone issue) is
/// worked on and the branch it was started from. Written by `atelier start`.
pub const WORKFLOW_BRANCH_FIELD: &str = "workflow_branch";

#[derive(Debug, Clone)]
pub struct StatusDefinition {
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct IssueTypeDefinition {
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct ValidatorDefinition {
    pub builtin: String,
    pub params: Option<ValidatorParams>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorParams {
    ShellCheck {
        command: String,
    },
    EvidenceAttached {
        min_count: i64,
        kind: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct WorkflowDefinition {
    pub applies_to: Vec<String>,
    pub initial_status: String,
    pub done_statuses: Vec<String>,
    pub transitions: BTreeMap<String, TransitionDefinition>,
}

#[derive(Debug, Clone)]
pub struct TransitionDefinition {
    pub from: Vec<String>,
    pub to: String,
    pub required_fields: Vec<String>,
    pub description: Option<String>,
    pub validators: Vec<ValidatorDefinition>,
}

impl WorkflowPolicy {
    pub fn workflow_name_for_issue_type(&self, issue_type: &str) -> Result<&str> {
        self.workflow_by_issue_type
            .get(issue_type)
            .map(String::as_str)
            .ok_or_else(|| {
                policy_error(
                    "workflow_issue_type_unmapped",
                    WORKFLOW_POLICY_PATH,
                    format!("issue_type '{}' is not mapped to a workflow", issue_type),
                )
            })
    }

    pub fn workflow_for_issue_type(&self, issue_type: &str) -> Result<&WorkflowDefinition> {
        let workflow_name = self.workflow_name_for_issue_type(issue_type)?;
        self.workflows.get(workflow_name).ok_or_else(|| {
            policy_error(
                "workflow_issue_type_unmapped",
                WORKFLOW_POLICY_PATH,
                format!(
                    "workflow '{}' selected by issue_type '{}' is not defined",
                    workflow_name, issue_type
                ),
            )
        })
    }

    pub fn status_category(&self, status: &str) -> Option<&str> {
        self.statuses
            .get(status)
            .map(|status| status.category.as_str())
    }

    pub fn issue_status_is_terminal(&self, issue_type: &str, status: &str) -> Result<bool> {
        Ok(self
            .workflow_for_issue_type(issue_type)?
            .done_statuses
            .iter()
            .any(|done_status| done_status == status))
    }

    pub fn workflow_allows_status(&self, issue_type: &str, status: &str) -> Result<bool> {
        Ok(workflow_statuses(self.workflow_for_issue_type(issue_type)?).contains(status))
    }

    pub fn transitions_from_status(
        &self,
        issue_type: &str,
        status: &str,
    ) -> Result<Vec<(&str, &TransitionDefinition)>> {
        Ok(self
            .workflow_for_issue_type(issue_type)?
            .transitions
            .iter()
            .filter(|(_, transition)| transition.from.iter().any(|from| from == status))
            .map(|(name, transition)| (name.as_str(), transition))
            .collect())
    }

    /// Returns whether work in this state can move directly into active execution.
    ///
    /// Status categories alone are not enough to answer this question: a planning
    /// state such as `draft` can intentionally share the `todo` category with an
    /// execution-ready state while requiring a separate refinement transition.
    pub fn can_enter_active(&self, issue_type: &str, status: &str) -> Result<bool> {
        Ok(self
            .transitions_from_status(issue_type, status)?
            .into_iter()
            .any(|(_, transition)| self.status_category(&transition.to) == Some("active")))
    }

    pub fn transition_for_issue_type(
        &self,
        issue_type: &str,
        transition_name: &str,
    ) -> Result<&TransitionDefinition> {
        self.workflow_for_issue_type(issue_type)?
            .transitions
            .get(transition_name)
            .ok_or_else(|| {
                policy_error(
                    "workflow_transition_unknown",
                    WORKFLOW_POLICY_PATH,
                    format!(
                        "transition '{}' is not defined for issue_type '{}'",
                        transition_name, issue_type
                    ),
                )
            })
    }

    /// Canonical branch for a branch owner (a parent or a standalone issue):
    /// `<issue_type>/<id>`.
    pub fn branch_name_for_owner(&self, owner: &Issue) -> Result<String> {
        let branch = format!("{}/{}", owner.issue_type, owner.id);
        validate_branch_value(
            &branch,
            WORKFLOW_POLICY_PATH,
            "branch_policy",
            "canonical branch name",
        )?;
        Ok(branch)
    }
}

#[derive(Debug, Clone)]
struct WorkflowPolicyError {
    code: &'static str,
    path: String,
    message: String,
    field: Option<String>,
    reference: Option<String>,
    line: Option<usize>,
    column: Option<usize>,
}

impl fmt::Display for WorkflowPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "error: {}", self.code)?;
        writeln!(f, "path: {}", self.path)?;
        if let Some(field) = &self.field {
            writeln!(f, "field: {}", field)?;
        }
        if let Some(reference) = &self.reference {
            writeln!(f, "reference: {}", reference)?;
        }
        if let Some(line) = self.line {
            writeln!(f, "line: {}", line)?;
        }
        if let Some(column) = self.column {
            writeln!(f, "column: {}", column)?;
        }
        write!(f, "message: {}", self.message)
    }
}

impl std::error::Error for WorkflowPolicyError {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatusDefinitionRaw {
    category: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct IssueTypeDefinitionRaw {
    label: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkflowDefinitionRaw {
    applies_to: Vec<String>,
    initial_status: String,
    done_statuses: Vec<String>,
    transitions: Mapping,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TransitionDefinitionRaw {
    from: StatusSelectorRaw,
    to: String,
    #[serde(default)]
    required_fields: Vec<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    validators: Vec<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StatusSelectorRaw {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BranchLifecycleRaw {
    base_branch: Option<String>,
    #[serde(default)]
    merge_strategy: Option<String>,
}

pub fn load(repo_root: &Path) -> Result<WorkflowPolicy> {
    let policy_path = repo_root.join(WORKFLOW_POLICY_PATH);
    let display_path = WORKFLOW_POLICY_PATH.to_string();
    if !policy_path.exists() {
        return Err(policy_error(
            "workflow_config_missing",
            &display_path,
            "workflow policy file is required at .atelier/workflow.yaml",
        ));
    }
    if !policy_path.is_file() {
        return Err(policy_error(
            "workflow_config_not_file",
            &display_path,
            "workflow policy path exists but is not a regular file",
        ));
    }
    let text = fs::read_to_string(&policy_path)
        .map_err(|error| anyhow!("failed to read {}: {}", policy_path.display(), error))?;
    parse_policy_text(&text, &display_path)
}

pub fn configured_initial_status(repo_root: &Path, issue_type: &str) -> Result<Option<String>> {
    let policy_path = repo_root.join(WORKFLOW_POLICY_PATH);
    if !policy_path.exists() {
        return Ok(None);
    }
    let policy = load(repo_root)?;
    let workflow = policy.workflow_for_issue_type(issue_type)?;
    Ok(Some(workflow.initial_status.clone()))
}

pub fn parse_policy_text(text: &str, display_path: &str) -> Result<WorkflowPolicy> {
    let root = parse_yaml(text, display_path)?;
    let root = root.as_mapping().ok_or_else(|| {
        policy_error(
            "workflow_config_parse_error",
            display_path,
            "workflow policy must be a YAML mapping at the top level",
        )
    })?;

    let schema_version = validate_schema(root, display_path)?;
    check_top_level_fields(root, display_path, schema_version)?;

    let statuses = parse_statuses(
        require_mapping(
            root,
            display_path,
            "statuses",
            "workflow_config_invalid_status",
            "statuses must be a mapping of status names to status definitions",
        )?,
        display_path,
    )?;
    let branch_policy = parse_branch_policy(root.get("branch_policy"), display_path)?;
    let issue_types = parse_issue_types(
        require_mapping(
            root,
            display_path,
            "issue_types",
            "workflow_config_invalid_issue_type",
            "issue_types must be a mapping of issue type names to issue type definitions",
        )?,
        display_path,
    )?;
    let workflows = parse_workflows(
        require_mapping(
            root,
            display_path,
            "workflows",
            "workflow_config_invalid_workflow",
            "workflows must be a mapping of workflow names to workflow definitions",
        )?,
        display_path,
    )?;
    let workflow_by_issue_type =
        issue_type_mappings_from_workflows(&issue_types, &workflows, display_path)?;

    let merge_checks = match root.get("merge_checks") {
        None => Vec::new(),
        Some(value) => parse_transition_validators(
            ValidatorContext::MergeChecks,
            "merge",
            "merge",
            value
                .as_sequence()
                .ok_or_else(|| anyhow!("merge_checks must be a sequence"))?,
            display_path,
        )?,
    };
    let policy = WorkflowPolicy {
        merge_checks,
        schema_version,
        branch_policy,
        issue_types,
        workflow_by_issue_type,
        statuses,
        workflows,
    };
    validate_workflows(&policy, display_path)?;
    Ok(policy)
}

fn parse_branch_policy(value: Option<&Value>, display_path: &str) -> Result<BranchLifecycleConfig> {
    let Some(value) = value else {
        return Err(policy_error_with_field(
            "workflow_config_invalid_branch_policy",
            display_path,
            "branch_policy",
            "branch_policy is required in workflow schema_version 3",
        ));
    };
    let raw = deserialize_entry::<BranchLifecycleRaw>(
        value,
        display_path,
        "branch_policy",
        "workflow_config_invalid_branch_policy",
    )?;
    let Some(base_branch) = raw.base_branch else {
        return Err(policy_error_with_field(
            "workflow_config_invalid_branch_policy",
            display_path,
            "branch_policy.base_branch",
            "branch_policy.base_branch is required",
        ));
    };
    validate_branch_value(
        &base_branch,
        display_path,
        "branch_policy.base_branch",
        "base branch",
    )?;
    let merge_strategy = parse_merge_strategy(raw.merge_strategy.as_deref(), display_path)?;
    Ok(BranchLifecycleConfig {
        base_branch,
        merge_strategy,
    })
}

fn parse_merge_strategy(value: Option<&str>, display_path: &str) -> Result<MergeStrategy> {
    match value.unwrap_or("squash") {
        "squash" => Ok(MergeStrategy::Squash),
        "merge_commit" => Ok(MergeStrategy::MergeCommit),
        "fast_forward_only" => Ok(MergeStrategy::FastForwardOnly),
        other => Err(policy_error_with_field(
            "workflow_config_invalid_branch_policy",
            display_path,
            "branch_policy.merge_strategy",
            format!(
                "unsupported merge strategy '{}'; expected squash, merge_commit, or fast_forward_only",
                other
            ),
        )),
    }
}

fn parse_yaml(text: &str, display_path: &str) -> Result<Value> {
    serde_yaml::from_str::<Value>(text).map_err(|error| {
        let mut workflow_error = WorkflowPolicyError {
            code: "workflow_config_parse_error",
            path: display_path.to_string(),
            message: error.to_string(),
            field: None,
            reference: None,
            line: None,
            column: None,
        };
        if let Some(location) = error.location() {
            workflow_error.line = Some(location.line());
            workflow_error.column = Some(location.column());
        }
        workflow_error.into()
    })
}

fn check_top_level_fields(root: &Mapping, display_path: &str, _schema_version: i64) -> Result<()> {
    for key in root.keys() {
        let Some(key) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_parse_error",
                display_path,
                "workflow policy keys must be strings",
            ));
        };
        if DEFERRED_TOP_LEVEL_FIELDS.contains(&key) {
            return Err(policy_error_with_field(
                "workflow_config_deferred_feature",
                display_path,
                key,
                format!(
                    "field '{}' is a deferred version 1 workflow feature and is not supported",
                    key
                ),
            ));
        }
        if !TOP_LEVEL_FIELDS.contains(&key) {
            return Err(policy_error_with_field(
                "workflow_config_unknown_field",
                display_path,
                key,
                format!("unknown top-level workflow field '{}'", key),
            ));
        }
    }
    Ok(())
}

fn validate_schema(root: &Mapping, display_path: &str) -> Result<i64> {
    let Some(schema_value) = root.get("schema") else {
        return Err(policy_error_with_field(
            "workflow_config_schema_missing",
            display_path,
            "schema",
            "workflow policy is missing required field 'schema'",
        ));
    };
    let Some(schema_version_value) = root.get("schema_version") else {
        return Err(policy_error_with_field(
            "workflow_config_schema_missing",
            display_path,
            "schema_version",
            "workflow policy is missing required field 'schema_version'",
        ));
    };

    let Some(schema) = schema_value.as_str() else {
        return Err(policy_error_with_field(
            "workflow_config_schema_unsupported",
            display_path,
            "schema",
            "workflow policy schema must be the string 'atelier.workflow'",
        ));
    };
    if schema != WORKFLOW_SCHEMA {
        return Err(policy_error_with_field(
            "workflow_config_schema_unsupported",
            display_path,
            "schema",
            format!(
                "unsupported workflow schema '{}'; expected '{}'",
                schema, WORKFLOW_SCHEMA
            ),
        ));
    }

    let Some(schema_version) = schema_version_value.as_i64() else {
        return Err(policy_error_with_field(
            "workflow_config_schema_unsupported",
            display_path,
            "schema_version",
            format!(
                "unsupported workflow schema_version '{:?}'; expected {}",
                schema_version_value, WORKFLOW_SCHEMA_VERSION
            ),
        ));
    };
    if schema_version != WORKFLOW_SCHEMA_VERSION {
        return Err(policy_error_with_field(
            "workflow_config_schema_unsupported",
            display_path,
            "schema_version",
            format!(
                "unsupported workflow schema_version {}; expected {}",
                schema_version, WORKFLOW_SCHEMA_VERSION
            ),
        ));
    }
    Ok(schema_version)
}

fn issue_type_mappings_from_workflows(
    issue_types: &BTreeMap<String, IssueTypeDefinition>,
    workflows: &BTreeMap<String, WorkflowDefinition>,
    display_path: &str,
) -> Result<BTreeMap<String, String>> {
    let mut mappings = BTreeMap::new();
    for (workflow_name, workflow) in workflows {
        for issue_type in &workflow.applies_to {
            if !issue_types.contains_key(issue_type) {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_issue_type",
                    display_path,
                    format!("workflows.{}.applies_to", workflow_name),
                    format!(
                        "workflow '{}' applies to unregistered issue type '{}'",
                        workflow_name, issue_type
                    ),
                ));
            }
            if let Some(existing_workflow) =
                mappings.insert(issue_type.clone(), workflow_name.clone())
            {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_issue_type",
                    display_path,
                    format!("workflows.{}.applies_to", workflow_name),
                    format!(
                        "issue type '{}' is assigned to both workflow '{}' and workflow '{}'",
                        issue_type, existing_workflow, workflow_name
                    ),
                ));
            }
        }
    }
    for issue_type in issue_types.keys() {
        if !mappings.contains_key(issue_type) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_issue_type",
                display_path,
                "workflows.*.applies_to",
                format!(
                    "missing workflow applies_to entry for registered issue type '{}'",
                    issue_type
                ),
            ));
        }
    }
    Ok(mappings)
}

fn parse_issue_types(
    mapping: &Mapping,
    display_path: &str,
) -> Result<BTreeMap<String, IssueTypeDefinition>> {
    if mapping.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_issue_type",
            display_path,
            "issue_types",
            "issue_types must define at least one issue type",
        ));
    }
    let mut issue_types = BTreeMap::new();
    for (key, value) in mapping {
        let Some(name) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_issue_type",
                display_path,
                "issue_types keys must be strings",
            ));
        };
        ensure_identifier(
            name,
            display_path,
            &format!("issue_types.{}", name),
            "workflow_config_invalid_issue_type",
            "issue type names",
        )?;
        let raw = deserialize_entry::<IssueTypeDefinitionRaw>(
            value,
            display_path,
            &format!("issue_types.{}", name),
            "workflow_config_invalid_issue_type",
        )?;
        let label = raw.label.ok_or_else(|| {
            policy_error_with_field(
                "workflow_config_invalid_issue_type",
                display_path,
                format!("issue_types.{}.label", name),
                "issue type label is required",
            )
        })?;
        if label.trim().is_empty() {
            return Err(policy_error_with_field(
                "workflow_config_invalid_issue_type",
                display_path,
                format!("issue_types.{}.label", name),
                "issue type label must not be empty",
            ));
        }
        issue_types.insert(name.to_string(), IssueTypeDefinition { label });
    }
    Ok(issue_types)
}

fn parse_statuses(
    mapping: &Mapping,
    display_path: &str,
) -> Result<BTreeMap<String, StatusDefinition>> {
    if mapping.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_status",
            display_path,
            "statuses",
            "statuses must define at least one status",
        ));
    }
    let mut statuses = BTreeMap::new();
    for (key, value) in mapping {
        let Some(name) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_status",
                display_path,
                "statuses keys must be strings",
            ));
        };
        ensure_identifier(
            name,
            display_path,
            &format!("statuses.{}", name),
            "workflow_config_invalid_status",
            "status names",
        )?;
        let raw = deserialize_entry::<StatusDefinitionRaw>(
            value,
            display_path,
            &format!("statuses.{}", name),
            "workflow_config_invalid_status",
        )?;
        if !STATUS_CATEGORIES.contains(&raw.category.as_str()) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_status",
                display_path,
                format!("statuses.{}.category", name),
                format!(
                    "status category '{}' is not supported; expected {}",
                    raw.category,
                    STATUS_CATEGORIES.join(", ")
                ),
            ));
        }
        statuses.insert(
            name.to_string(),
            StatusDefinition {
                category: raw.category,
            },
        );
    }
    Ok(statuses)
}

fn parse_validator_params(
    validator_name: &str,
    builtin: &str,
    params: Option<Value>,
    display_path: &str,
) -> Result<Option<ValidatorParams>> {
    match builtin {
        "shell.check" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct ShellParams {
                command: String,
            }
            let parsed: ShellParams = serde_yaml::from_value(
                params.ok_or_else(|| anyhow!("shell.check requires command"))?,
            )?;
            if parsed.command.trim().is_empty() {
                return Err(anyhow!("shell.check command cannot be empty"));
            }
            Ok(Some(ValidatorParams::ShellCheck {
                command: parsed.command,
            }))
        }
        "evidence.attached" => {
            let Some(params) = params else {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!("validators.{}.params", validator_name),
                    "evidence.attached requires params.min_count >= 1",
                ));
            };
            let mapping = params.as_mapping().ok_or_else(|| {
                policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!("validators.{}.params", validator_name),
                    "validator params must be a mapping",
                )
            })?;
            for key in mapping.keys() {
                let Some(key) = key.as_str() else {
                    return Err(policy_error_with_field(
                        "workflow_config_invalid_validator",
                        display_path,
                        format!("validators.{}.params", validator_name),
                        "validator param keys must be strings",
                    ));
                };
                if key != "min_count" && key != "kind" {
                    return Err(policy_error_with_field(
                        "workflow_config_invalid_validator",
                        display_path,
                        format!("validators.{}.params.{}", validator_name, key),
                        format!("unexpected validator param '{}'", key),
                    ));
                }
            }
            let Some(min_count_value) = mapping.get("min_count") else {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!("validators.{}.params.min_count", validator_name),
                    "evidence.attached requires params.min_count >= 1",
                ));
            };
            let Some(min_count) = min_count_value.as_i64() else {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!("validators.{}.params.min_count", validator_name),
                    "validator param min_count must be an integer",
                ));
            };
            if min_count < 1 {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!("validators.{}.params.min_count", validator_name),
                    "validator param min_count must be at least 1",
                ));
            }
            let kind = mapping
                .get("kind")
                .map(|value| {
                    value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                        policy_error_with_field(
                            "workflow_config_invalid_validator",
                            display_path,
                            format!("validators.{}.params.kind", validator_name),
                            "validator param kind must be a string",
                        )
                    })
                })
                .transpose()?;
            Ok(Some(ValidatorParams::EvidenceAttached { min_count, kind }))
        }
        _ => {
            if params.is_some() {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!("validators.{}.params", validator_name),
                    format!("built-in validator '{}' does not accept params", builtin),
                ));
            }
            Ok(None)
        }
    }
}

fn parse_workflows(
    mapping: &Mapping,
    display_path: &str,
) -> Result<BTreeMap<String, WorkflowDefinition>> {
    if mapping.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_workflow",
            display_path,
            "workflows",
            "workflows must define at least one workflow",
        ));
    }
    let mut workflows = BTreeMap::new();
    for (key, value) in mapping {
        let Some(workflow_name) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_workflow",
                display_path,
                "workflow names must be strings",
            ));
        };
        workflows.insert(
            workflow_name.to_string(),
            parse_workflow_definition(workflow_name, value, display_path)?,
        );
    }
    Ok(workflows)
}

fn parse_workflow_definition(
    workflow_name: &str,
    value: &Value,
    display_path: &str,
) -> Result<WorkflowDefinition> {
    let workflow_field = format!("workflows.{}", workflow_name);
    ensure_identifier(
        workflow_name,
        display_path,
        &workflow_field,
        "workflow_config_invalid_workflow",
        "workflow names",
    )?;
    let raw = deserialize_entry::<WorkflowDefinitionRaw>(
        value,
        display_path,
        &workflow_field,
        "workflow_config_invalid_workflow",
    )?;
    validate_workflow_shape(workflow_name, &raw, display_path)?;
    let transitions = parse_transitions(workflow_name, &raw.transitions, display_path)?;

    Ok(WorkflowDefinition {
        applies_to: raw.applies_to,
        initial_status: raw.initial_status,
        done_statuses: raw.done_statuses,
        transitions,
    })
}

fn validate_workflow_shape(
    workflow_name: &str,
    raw: &WorkflowDefinitionRaw,
    display_path: &str,
) -> Result<()> {
    if raw.applies_to.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_issue_type",
            display_path,
            format!("workflows.{}.applies_to", workflow_name),
            "workflow applies_to must contain at least one registered issue type",
        ));
    }
    if raw.done_statuses.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_workflow",
            display_path,
            format!("workflows.{}.done_statuses", workflow_name),
            "workflow done_statuses must contain at least one terminal status",
        ));
    }
    if raw.transitions.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_workflow",
            display_path,
            format!("workflows.{}.transitions", workflow_name),
            "workflow transitions must contain at least one transition",
        ));
    }
    Ok(())
}

fn parse_transitions(
    workflow_name: &str,
    mapping: &Mapping,
    display_path: &str,
) -> Result<BTreeMap<String, TransitionDefinition>> {
    let mut transitions = BTreeMap::new();
    for (key, value) in mapping {
        let Some(transition_name) = key.as_str() else {
            return Err(policy_error_with_field(
                "workflow_config_invalid_transition",
                display_path,
                format!("workflows.{}.transitions", workflow_name),
                "transition names must be strings",
            ));
        };
        transitions.insert(
            transition_name.to_string(),
            parse_transition_definition(workflow_name, transition_name, value, display_path)?,
        );
    }
    Ok(transitions)
}

fn parse_transition_definition(
    workflow_name: &str,
    transition_name: &str,
    value: &Value,
    display_path: &str,
) -> Result<TransitionDefinition> {
    let transition_field = format!(
        "workflows.{}.transitions.{}",
        workflow_name, transition_name
    );
    ensure_identifier(
        transition_name,
        display_path,
        &transition_field,
        "workflow_config_invalid_transition",
        "transition names",
    )?;
    let raw = deserialize_entry::<TransitionDefinitionRaw>(
        value,
        display_path,
        &transition_field,
        "workflow_config_invalid_transition",
    )?;
    let from = parse_transition_from(workflow_name, transition_name, raw.from, display_path)?;
    validate_required_fields(
        workflow_name,
        transition_name,
        &raw.required_fields,
        display_path,
    )?;
    let validators = parse_transition_validators(
        ValidatorContext::Transition,
        workflow_name,
        transition_name,
        &raw.validators,
        display_path,
    )?;
    Ok(TransitionDefinition {
        from,
        to: raw.to,
        required_fields: raw.required_fields,
        description: raw.description,
        validators,
    })
}

#[derive(Clone, Copy)]
enum ValidatorContext {
    Transition,
    MergeChecks,
}

fn parse_transition_validators(
    context: ValidatorContext,
    workflow_name: &str,
    transition_name: &str,
    values: &[Value],
    display_path: &str,
) -> Result<Vec<ValidatorDefinition>> {
    let mut validators = Vec::new();
    for value in values {
        match value {
            Value::String(name) => {
                if name == "shell.check" {
                    return Err(anyhow!("shell.check requires command parameters"));
                }
                validate_builtin_validator_name(
                    context,
                    workflow_name,
                    transition_name,
                    name,
                    display_path,
                )?;
                validators.push(ValidatorDefinition {
                    builtin: name.clone(),
                    params: None,
                });
            }
            Value::Mapping(mapping) if mapping.len() == 1 => {
                let (key, params) = mapping.iter().next().expect("mapping has one entry");
                let Some(name) = key.as_str() else {
                    return Err(policy_error_with_field(
                        "workflow_config_invalid_validator",
                        display_path,
                        format!(
                            "workflows.{}.transitions.{}.validators",
                            workflow_name, transition_name
                        ),
                        "validator map keys must be strings",
                    ));
                };
                validate_builtin_validator_name(
                    context,
                    workflow_name,
                    transition_name,
                    name,
                    display_path,
                )?;
                let params =
                    parse_validator_params(name, name, Some(params.clone()), display_path)?;
                validators.push(ValidatorDefinition {
                    builtin: name.to_string(),
                    params,
                });
            }
            Value::Mapping(_) => {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!(
                        "workflows.{}.transitions.{}.validators",
                        workflow_name, transition_name
                    ),
                    "validator maps must contain exactly one built-in validator name",
                ));
            }
            _ => {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!(
                        "workflows.{}.transitions.{}.validators",
                        workflow_name, transition_name
                    ),
                    "validators must be built-in validator strings or single-key validator maps",
                ));
            }
        }
    }
    Ok(validators)
}

fn validate_builtin_validator_name(
    context: ValidatorContext,
    workflow_name: &str,
    transition_name: &str,
    name: &str,
    display_path: &str,
) -> Result<()> {
    if name == "shell.check" && !matches!(context, ValidatorContext::MergeChecks) {
        return Err(anyhow!("shell.check is available only in merge_checks"));
    }
    if BUILTIN_VALIDATORS.contains(&name) {
        Ok(())
    } else {
        Err(policy_error_with_field(
            "workflow_config_invalid_validator",
            display_path,
            format!(
                "workflows.{}.transitions.{}.validators",
                workflow_name, transition_name
            ),
            format!(
                "unsupported built-in validator '{}'; expected {}",
                name,
                BUILTIN_VALIDATORS.join(", ")
            ),
        ))
    }
}

fn parse_transition_from(
    workflow_name: &str,
    transition_name: &str,
    from: StatusSelectorRaw,
    display_path: &str,
) -> Result<Vec<String>> {
    match from {
        StatusSelectorRaw::One(status) => Ok(vec![status]),
        StatusSelectorRaw::Many(statuses) if !statuses.is_empty() => Ok(statuses),
        StatusSelectorRaw::Many(_) => Err(policy_error_with_field(
            "workflow_config_invalid_transition",
            display_path,
            format!(
                "workflows.{}.transitions.{}.from",
                workflow_name, transition_name
            ),
            "transition from must be a status name or a non-empty list of status names",
        )),
    }
}

fn validate_required_fields(
    workflow_name: &str,
    transition_name: &str,
    required_fields: &[String],
    display_path: &str,
) -> Result<()> {
    for required_field in required_fields {
        if !ALLOWED_REQUIRED_FIELDS.contains(&required_field.as_str()) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_transition",
                display_path,
                format!(
                    "workflows.{}.transitions.{}.required_fields",
                    workflow_name, transition_name
                ),
                format!(
                    "unsupported required field '{}'; expected {}",
                    required_field,
                    ALLOWED_REQUIRED_FIELDS.join(", ")
                ),
            ));
        }
    }
    Ok(())
}

fn validate_workflows(policy: &WorkflowPolicy, display_path: &str) -> Result<()> {
    for (workflow_name, workflow) in &policy.workflows {
        validate_workflow_statuses(policy, workflow_name, workflow, display_path)?;
        validate_workflow_transitions(policy, workflow_name, workflow, display_path)?;
    }
    Ok(())
}

fn validate_workflow_statuses(
    policy: &WorkflowPolicy,
    workflow_name: &str,
    workflow: &WorkflowDefinition,
    display_path: &str,
) -> Result<()> {
    ensure_known_status(
        &policy.statuses,
        &workflow.initial_status,
        display_path,
        &format!("workflows.{}.initial_status", workflow_name),
    )?;
    if workflow.done_statuses.contains(&workflow.initial_status) {
        return Err(policy_error_with_field(
            "workflow_config_invalid_workflow",
            display_path,
            format!("workflows.{}.initial_status", workflow_name),
            format!(
                "workflow '{}' initial_status '{}' cannot also be terminal",
                workflow_name, workflow.initial_status
            ),
        ));
    }
    for done_status in &workflow.done_statuses {
        ensure_identifier(
            done_status,
            display_path,
            &format!("workflows.{}.done_statuses", workflow_name),
            "workflow_config_invalid_workflow",
            "workflow terminal status names",
        )?;
        ensure_terminal_status(policy, workflow_name, done_status, display_path)?;
    }
    Ok(())
}

fn ensure_terminal_status(
    policy: &WorkflowPolicy,
    workflow_name: &str,
    done_status: &str,
    display_path: &str,
) -> Result<()> {
    ensure_known_status(
        &policy.statuses,
        done_status,
        display_path,
        &format!("workflows.{}.done_statuses", workflow_name),
    )?;
    let category = &policy.statuses[done_status].category;
    if category != "done" {
        return Err(policy_error_with_field(
            "workflow_config_invalid_workflow",
            display_path,
            format!("workflows.{}.done_statuses", workflow_name),
            format!(
                "workflow '{}' terminal status '{}' must use category 'done', found '{}'",
                workflow_name, done_status, category
            ),
        ));
    }
    Ok(())
}

fn validate_workflow_transitions(
    policy: &WorkflowPolicy,
    workflow_name: &str,
    workflow: &WorkflowDefinition,
    display_path: &str,
) -> Result<()> {
    for (transition_name, transition) in &workflow.transitions {
        validate_transition_statuses(
            policy,
            workflow_name,
            workflow,
            transition_name,
            transition,
            display_path,
        )?;
    }
    Ok(())
}

fn validate_transition_statuses(
    policy: &WorkflowPolicy,
    workflow_name: &str,
    workflow: &WorkflowDefinition,
    transition_name: &str,
    transition: &TransitionDefinition,
    display_path: &str,
) -> Result<()> {
    ensure_known_status(
        &policy.statuses,
        &transition.to,
        display_path,
        &format!(
            "workflows.{}.transitions.{}.to",
            workflow_name, transition_name
        ),
    )?;
    if transition
        .required_fields
        .iter()
        .any(|field| field == "close_reason")
        && policy.status_category(&transition.to) != Some("done")
    {
        return Err(policy_error_with_field(
            "workflow_config_invalid_transition",
            display_path,
            format!("workflows.{workflow_name}.transitions.{transition_name}.required_fields"),
            "close_reason may only be required by a done-category transition",
        ));
    }
    for from_status in &transition.from {
        ensure_known_status(
            &policy.statuses,
            from_status,
            display_path,
            &format!(
                "workflows.{}.transitions.{}.from",
                workflow_name, transition_name
            ),
        )?;
        if workflow.done_statuses.contains(from_status) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_transition",
                display_path,
                format!(
                    "workflows.{}.transitions.{}.from",
                    workflow_name, transition_name
                ),
                format!(
                    "transition '{}' in workflow '{}' cannot leave terminal status '{}'",
                    transition_name, workflow_name, from_status
                ),
            ));
        }
    }
    Ok(())
}

pub fn validate_issue_against_policy(
    policy: &WorkflowPolicy,
    issue: &Issue,
    policy_path: &Path,
) -> Result<()> {
    if !policy.issue_types.contains_key(&issue.issue_type) {
        return Err(WorkflowPolicyError {
            code: "workflow_issue_type_unknown",
            path: policy_path.display().to_string(),
            message: format!(
                "issue {} has issue_type '{}' which is not registered in issue_types",
                issue.id, issue.issue_type
            ),
            field: Some("issue_type".to_string()),
            reference: Some(issue.id.clone()),
            line: None,
            column: None,
        }
        .into());
    }
    let workflow_name = policy
        .workflow_by_issue_type
        .get(&issue.issue_type)
        .ok_or_else(|| {
            policy_error_with_field(
                "workflow_config_invalid_issue_type",
                WORKFLOW_POLICY_PATH,
                format!("workflows.*.applies_to.{}", issue.issue_type),
                format!(
                    "missing workflow mapping for issue type '{}' required by issue {}",
                    issue.issue_type, issue.id
                ),
            )
        })?;
    let workflow = policy.workflows.get(workflow_name).ok_or_else(|| {
        policy_error_with_reference(
            "workflow_config_invalid_issue_type",
            WORKFLOW_POLICY_PATH,
            format!("workflows.*.applies_to.{}", issue.issue_type),
            workflow_name,
            format!(
                "issue type '{}' points to undefined workflow '{}' required by issue {}",
                issue.issue_type, workflow_name, issue.id
            ),
        )
    })?;
    let allowed_statuses = workflow_statuses(workflow);
    if !allowed_statuses.contains(&issue.status) {
        let mut allowed = allowed_statuses.into_iter().collect::<Vec<_>>();
        allowed.sort();
        return Err(WorkflowPolicyError {
            code: "workflow_issue_status_invalid",
            path: policy_path.display().to_string(),
            message: format!(
                "issue {} has status '{}' which is not valid for workflow '{}' selected by issue_type '{}'; allowed statuses: {}",
                issue.id,
                issue.status,
                workflow_name,
                issue.issue_type,
                allowed.join(", ")
            ),
            field: None,
            reference: Some(issue.id.clone()),
            line: None,
            column: None,
        }
        .into());
    }
    validate_issue_fields_against_policy(policy, issue, policy_path)?;
    Ok(())
}

fn validate_issue_fields_against_policy(
    _policy: &WorkflowPolicy,
    issue: &Issue,
    policy_path: &Path,
) -> Result<()> {
    if let Some(strategy) = &issue.strategy {
        if issue.issue_type != "mission" {
            return invalid_strategy_field(
                issue,
                policy_path,
                "strategy is only valid on issue_type 'mission'",
            );
        }
        IssueStrategy::new(&strategy.path)?;
    }
    if let Some(branch) = &issue.workflow_branch {
        for (key, value) in [
            ("work_branch", &branch.work_branch),
            ("branch_base", &branch.branch_base),
        ] {
            validate_branch_value(
                value,
                &policy_path.display().to_string(),
                &format!("workflow_branch.{key}"),
                "workflow branch",
            )?;
        }
    }
    Ok(())
}

fn invalid_strategy_field(issue: &Issue, policy_path: &Path, reason: &str) -> Result<()> {
    Err(WorkflowPolicyError {
        code: "workflow_issue_field_invalid",
        path: policy_path.display().to_string(),
        message: format!("issue {} field 'strategy' is invalid: {}", issue.id, reason),
        field: Some(ISSUE_STRATEGY_FIELD.to_string()),
        reference: Some(issue.id.clone()),
        line: None,
        column: None,
    }
    .into())
}

fn workflow_statuses(workflow: &WorkflowDefinition) -> BTreeSet<String> {
    let mut statuses = BTreeSet::new();
    statuses.insert(workflow.initial_status.clone());
    for status in &workflow.done_statuses {
        statuses.insert(status.clone());
    }
    for transition in workflow.transitions.values() {
        statuses.insert(transition.to.clone());
        for from in &transition.from {
            statuses.insert(from.clone());
        }
    }
    statuses
}

fn require_mapping<'a>(
    root: &'a Mapping,
    display_path: &str,
    key: &str,
    code: &'static str,
    message: &str,
) -> Result<&'a Mapping> {
    let Some(value) = root.get(key) else {
        return Err(policy_error_with_field(code, display_path, key, message));
    };
    value
        .as_mapping()
        .ok_or_else(|| policy_error_with_field(code, display_path, key, message))
}

fn deserialize_entry<T>(
    value: &Value,
    display_path: &str,
    field: &str,
    code: &'static str,
) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_yaml::from_value::<T>(value.clone()).map_err(|error| {
        let message = error.to_string();
        let error_code = if message.contains("unknown field") {
            "workflow_config_unknown_field"
        } else {
            code
        };
        policy_error_with_field(error_code, display_path, field, message)
    })
}

fn validate_branch_value(value: &str, display_path: &str, field: &str, kind: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains("..")
        || value.contains(' ')
        || value.contains('\\')
    {
        return Err(policy_error_with_field(
            "workflow_config_invalid_branch_policy",
            display_path,
            field,
            format!("{} is not a valid branch policy value: '{}'", kind, value),
        ));
    }
    Ok(())
}

fn ensure_identifier(
    name: &str,
    display_path: &str,
    field: &str,
    code: &'static str,
    kind: &str,
) -> Result<()> {
    if is_identifier(name) {
        Ok(())
    } else {
        Err(policy_error_with_field(
            code,
            display_path,
            field,
            format!("{} must match ^[a-z][a-z0-9_]*$; found '{}'", kind, name),
        ))
    }
}

fn ensure_known_status(
    statuses: &BTreeMap<String, StatusDefinition>,
    status: &str,
    display_path: &str,
    field: &str,
) -> Result<()> {
    if statuses.contains_key(status) {
        Ok(())
    } else {
        Err(policy_error_with_reference(
            "workflow_config_unknown_reference",
            display_path,
            field,
            status,
            format!("status '{}' is not defined in top-level statuses", status),
        ))
    }
}

fn is_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

fn policy_error(code: &'static str, path: &str, message: impl Into<String>) -> anyhow::Error {
    WorkflowPolicyError {
        code,
        path: path.to_string(),
        message: message.into(),
        field: None,
        reference: None,
        line: None,
        column: None,
    }
    .into()
}

fn policy_error_with_field(
    code: &'static str,
    path: &str,
    field: impl Into<String>,
    message: impl Into<String>,
) -> anyhow::Error {
    WorkflowPolicyError {
        code,
        path: path.to_string(),
        message: message.into(),
        field: Some(field.into()),
        reference: None,
        line: None,
        column: None,
    }
    .into()
}

fn policy_error_with_reference(
    code: &'static str,
    path: &str,
    field: impl Into<String>,
    reference: impl Into<String>,
    message: impl Into<String>,
) -> anyhow::Error {
    WorkflowPolicyError {
        code,
        path: path.to_string(),
        message: message.into(),
        field: Some(field.into()),
        reference: Some(reference.into()),
        line: None,
        column: None,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_policy() -> &'static str {
        STARTER_POLICY_YAML
    }

    fn default_branch_policy() -> &'static str {
        "branch_policy:\n  base_branch: main\n  merge_strategy: fast_forward_only\n"
    }

    fn configured_policy() -> String {
        valid_policy().replace(
            default_branch_policy(),
            "branch_policy:\n  base_branch: trunk\n  merge_strategy: fast_forward_only\n",
        )
    }

    fn work_close_validators() -> &'static str {
        "        description: Done claim; commit it on the work branch before review. A parent closes after its children.\n        validators:\n        - blockers.none_open\n        - work.terminal\n      supersede:\n        from: [backlog, todo, blocked, in_progress]\n"
    }

    fn with_work_close_validators(validators: &str) -> String {
        valid_policy().replace(
            work_close_validators(),
            &format!(
                "        description: Done claim; commit it on the work branch before review. A parent closes after its children.\n        validators:\n{validators}      supersede:\n        from: [backlog, todo, blocked, in_progress]\n"
            ),
        )
    }

    fn test_issue() -> Issue {
        let now = chrono::Utc::now();
        Issue {
            id: "atelier-pr01".to_string(),
            title: "Test".to_string(),
            description: None,
            issue_type: "task".to_string(),
            status: "todo".to_string(),
            priority: "medium".to_string(),
            strategy: None,
            workflow_branch: None,
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        }
    }

    fn validator_names(validators: &[ValidatorDefinition]) -> Vec<&str> {
        validators
            .iter()
            .map(|validator| validator.builtin.as_str())
            .collect()
    }

    #[test]
    fn parses_valid_policy() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(policy.schema_version, 3);
        for (issue_type, workflow) in [
            ("mission", "objective"),
            ("feature", "work"),
            ("task", "work"),
            ("validation", "work"),
        ] {
            assert_eq!(
                policy
                    .workflow_by_issue_type
                    .get(issue_type)
                    .map(String::as_str),
                Some(workflow)
            );
        }
        assert_eq!(
            policy
                .issue_types
                .get("task")
                .map(|issue_type| issue_type.label.as_str()),
            Some("Task")
        );
        assert_eq!(policy.status_category("done"), Some("done"));
        assert_eq!(policy.status_category("review"), Some("active"));
        let close = &policy.workflows["work"].transitions["close"];
        assert_eq!(close.required_fields, vec!["close_reason".to_string()]);
        assert_eq!(
            validator_names(&close.validators),
            vec!["blockers.none_open", "work.terminal"]
        );
        assert_eq!(
            validator_names(&policy.workflows["objective"].transitions["start"].validators),
            vec!["blockers.transitive_none_open"]
        );
        assert_eq!(
            policy.branch_policy.merge_strategy,
            MergeStrategy::FastForwardOnly
        );
        assert_eq!(policy.branch_policy.base_branch, "main");
    }

    #[test]
    fn starter_policy_has_objective_work_and_decision_workflows_only() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(
            policy
                .workflows
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["decision", "objective", "work"]
        );
        assert!(!policy.statuses.contains_key("publish_review"));
    }

    #[test]
    fn transition_actions_are_unknown_fields() {
        let policy = valid_policy().replace(
            "        validators:\n        - blockers.transitive_none_open\n",
            "        validators:\n        - blockers.transitive_none_open\n        actions:\n        - git.prepare_branch\n",
        );
        let error = parse_policy_text(&policy, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_unknown_field"), "{error}");
        assert!(error.contains("actions"), "{error}");
    }

    #[test]
    fn status_role_is_an_unknown_field() {
        let policy = valid_policy().replace(
            "  in_progress:\n    category: active\n",
            "  in_progress:\n    category: active\n    role: worker\n",
        );
        let error = parse_policy_text(&policy, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_unknown_field"), "{error}");
        assert!(error.contains("role"), "{error}");
    }

    #[test]
    fn starter_policy_distinguishes_draft_objectives_from_execution_ready_objectives() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();

        assert!(!policy.can_enter_active("mission", "draft").unwrap());
        assert!(policy.can_enter_active("mission", "todo").unwrap());
        assert!(!policy.can_enter_active("task", "backlog").unwrap());
        assert!(policy.can_enter_active("task", "todo").unwrap());
    }

    #[test]
    fn accepts_retained_validators_with_evidence_params() {
        let policy = with_work_close_validators(
            "        - issue.sections_parseable\n        - blockers.none_open\n        - blockers.transitive_none_open\n        - evidence.attached: { min_count: 2, kind: test }\n        - lint.none_blocking\n        - git.worktree_clean\n        - work.terminal\n",
        );
        let policy = parse_policy_text(&policy, WORKFLOW_POLICY_PATH).unwrap();
        let close = &policy.workflows["work"].transitions["close"];
        assert_eq!(close.validators.len(), 7);
        assert_eq!(
            close.validators[3].params.as_ref(),
            Some(&ValidatorParams::EvidenceAttached {
                min_count: 2,
                kind: Some("test".to_string()),
            })
        );
    }

    #[test]
    fn rejects_removed_validators() {
        for removed in [
            "children.proof_complete",
            "objective.work_terminal",
            "objective.work_present",
            "epic.proof_route_present",
            "validation.criteria_satisfied",
            "validation.parent_epic",
            "closeout.failures_classified",
            "baseline.default_checks",
            "command_surface_current",
            "ignored_tests_reviewed",
            "git.on_base",
            "git.on_mission_branch",
            "no_open_work",
            "tracker.current",
        ] {
            let policy = with_work_close_validators(&format!("        - {removed}\n"));
            let error = parse_policy_text(&policy, WORKFLOW_POLICY_PATH)
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("workflow_config_invalid_validator"),
                "{error}"
            );
            assert!(error.contains(removed), "{error}");
        }
    }

    #[test]
    fn validates_strategy_field_as_mission_only() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut issue = test_issue();
        issue.strategy = Some(IssueStrategy::new("docs/strategies/adaptive-mission.md").unwrap());
        issue.issue_type = "mission".to_string();
        issue.status = "draft".to_string();

        validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH)).unwrap();

        issue.issue_type = "feature".to_string();
        issue.status = "todo".to_string();
        let error = validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("only valid on issue_type 'mission'"),
            "{error}"
        );
    }

    #[test]
    fn validates_workflow_branch_field_shape() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut issue = test_issue();
        issue.workflow_branch = Some(atelier_core::WorkflowBranch {
            work_branch: "task/atelier-pr01".to_string(),
            branch_base: "main".to_string(),
        });

        validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH)).unwrap();
    }

    #[test]
    fn typed_workflow_branch_rejects_obsolete_integration_field() {
        assert!(serde_json::from_value::<atelier_core::WorkflowBranch>(serde_json::json!({"work_branch": "task/test", "branch_base": "main", "integration_target": "main"})).is_err());
    }

    #[test]
    fn rejects_unknown_issue_type_in_record() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut issue = test_issue();
        issue.issue_type = "incident".to_string();

        let error = validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH))
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_issue_type_unknown"));
        assert!(error.contains("incident"));
        assert!(error.contains("issue_types"));
    }

    #[test]
    fn rejects_removed_top_level_fields() {
        let text = format!("{}\nguidance_templates: {{}}\n", valid_policy());
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_config_unknown_field"));
        assert!(error.contains("guidance_templates"));
    }

    #[test]
    fn parses_configured_branch_policy() {
        let policy = parse_policy_text(&configured_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(policy.branch_policy.base_branch, "trunk");
        assert_eq!(
            policy.branch_policy.merge_strategy,
            MergeStrategy::FastForwardOnly
        );
    }

    #[test]
    fn rejects_configured_branch_policy_without_base_branch() {
        let text = valid_policy().replace(
            default_branch_policy(),
            "branch_policy:\n  merge_strategy: fast_forward_only\n",
        );
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_invalid_branch_policy"));
        assert!(error.contains("branch_policy.base_branch"));
    }

    #[test]
    fn missing_branch_policy_is_rejected() {
        let text = valid_policy().replace(default_branch_policy(), "");
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_config_invalid_branch_policy"));
        assert!(error.contains("branch_policy is required"));
    }

    #[test]
    fn branch_name_for_owner_uses_issue_type_and_id() {
        let policy = parse_policy_text(&configured_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut issue = test_issue();
        issue.id = "atelier-abc1".to_string();

        assert_eq!(
            policy.branch_name_for_owner(&issue).unwrap(),
            "task/atelier-abc1"
        );
        issue.issue_type = "feature".to_string();
        assert_eq!(
            policy.branch_name_for_owner(&issue).unwrap(),
            "feature/atelier-abc1"
        );
    }

    #[test]
    fn rejects_unknown_top_level_field() {
        let error = parse_policy_text(
            &format!("{}\nunknown: true\n", valid_policy()),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_unknown_field"));
        assert!(error.contains("unknown"));
    }

    #[test]
    fn rejects_invalid_status_category() {
        let error = parse_policy_text(
            &valid_policy().replace("category: todo", "category: queued"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_status"));
        assert!(error.contains("queued"));
    }

    #[test]
    fn rejects_unknown_inline_validator() {
        let error = parse_policy_text(
            &with_work_close_validators("        - missing_validator\n"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_validator"));
        assert!(error.contains("missing_validator"));
    }

    #[test]
    fn rejects_missing_issue_type_coverage() {
        let error = parse_policy_text(
            &valid_policy().replace(
                "  validation:\n    label: Validation\n",
                "  validation:\n    label: Validation\n  incident:\n    label: Incident\n",
            ),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_issue_type"));
        assert!(error.contains("incident"));
    }

    #[test]
    fn parses_custom_issue_type_registry() {
        let policy = valid_policy()
            .replace(
                "  validation:\n    label: Validation\n",
                "  validation:\n    label: Validation\n  incident:\n    label: Incident\n",
            )
            .replace("    - task\n", "    - task\n    - incident\n");

        let policy = parse_policy_text(&policy, WORKFLOW_POLICY_PATH).unwrap();

        assert_eq!(
            policy
                .issue_types
                .get("incident")
                .map(|issue_type| issue_type.label.as_str()),
            Some("Incident")
        );
        assert_eq!(
            policy
                .workflow_by_issue_type
                .get("incident")
                .map(String::as_str),
            Some("work")
        );
    }

    #[test]
    fn rejects_missing_issue_type_registry_entry() {
        let error = parse_policy_text(
            &valid_policy().replace("    - task\n", "    - task\n    - incident\n"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("workflow_config_invalid_issue_type"));
        assert!(error.contains("incident"));
        assert!(error.contains("unregistered"));
    }

    #[test]
    fn rejects_invalid_issue_type_name_and_label() {
        let invalid_name =
            valid_policy().replace("  task:\n    label: Task\n", "  Task:\n    label: Task\n");
        let error = parse_policy_text(&invalid_name, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_invalid_issue_type"));
        assert!(error.contains("issue type names"));

        let empty_label =
            valid_policy().replace("  task:\n    label: Task\n", "  task:\n    label: \"\"\n");
        let error = parse_policy_text(&empty_label, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_invalid_issue_type"));
        assert!(error.contains("label must not be empty"));
    }

    #[test]
    fn rejects_invalid_evidence_validator_params() {
        let error = parse_policy_text(
            &with_work_close_validators("        - evidence.attached: { min_count: 0 }\n"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_validator"));
        assert!(error.contains("min_count"));
    }

    #[test]
    fn rejects_duplicate_issue_type_coverage() {
        let error = parse_policy_text(
            &valid_policy().replace("    - mission\n", "    - mission\n    - task\n"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_issue_type"));
        assert!(error.contains("task"));
    }
    #[test]
    fn close_reason_requirement_rejects_nonterminal_destination() {
        let policy = valid_policy().replace(
            "        description: Start work.",
            "        required_fields: [close_reason]\n        description: Start work.",
        );
        let error = parse_policy_text(&policy, WORKFLOW_POLICY_PATH).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("close_reason may only be required by a done-category transition"),
            "{error}"
        );
    }
}
