use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use serde_yaml::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use atelier_core::Issue;

pub use atelier_core::RecordId;

pub const STARTER_POLICY_YAML: &str = r#"schema: atelier.workflow
schema_version: 3

branch_policy:
  base_branch: main
  merge_strategy: squash
  branch_templates:
    epic: epic/{{ issue.id }}
    issue: codex/{{ issue.id }}

statuses:
  todo:
    category: todo
  in_progress:
    category: active
  blocked:
    category: blocked
  review:
    category: review
  validation:
    category: validation
  done:
    category: done
  archived:
    category: done

workflows:
  standard:
    applies_to: [bug, feature, task]
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, validation]
        to: blocked
      close:
        from: [in_progress, validation]
        to: done
        required_fields: [close_reason]
        description: "Closing requires attached evidence and no open blockers."
        validators:
          - evidence_attached: { min_count: 1 }
          - no_open_blockers
          - no_blocking_lints
          - durable_state_current

  epic_reviewed:
    applies_to: [epic]
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review, validation]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
        effects:
          - review_artifact_open
      request_validation:
        from: [in_progress, review]
        to: validation
        validators:
          - review_complete
      close:
        from: [validation]
        to: done
        description: "Closing requires attached evidence, complete child proof, review merge, and a clean worktree."
        validators:
          - evidence_attached: { min_count: 1 }
          - epic_child_proof_complete
          - no_open_blockers
          - no_blocking_lints
          - durable_state_current
          - git_worktree_clean

  validation_reviewed:
    applies_to: [validation]
    initial_status: todo
    done_statuses: [done, archived]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review, validation]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
        effects:
          - review_artifact_open
      request_validation:
        from: [in_progress, review]
        to: validation
        validators:
          - review_complete
      close:
        from: [validation]
        to: done
        description: "Closing requires attached evidence, complete child proof, and a clean worktree."
        validators:
          - evidence_attached: { min_count: 1 }
          - epic_child_proof_complete
          - no_open_blockers
          - no_blocking_lints
          - durable_state_current
          - git_worktree_clean

  spike:
    applies_to: [spike]
    initial_status: todo
    done_statuses: [done]
    transitions:
      start:
        from: [todo, blocked]
        to: in_progress
      block:
        from: [todo, in_progress, review]
        to: blocked
      request_review:
        from: [in_progress]
        to: review
      revise:
        from: [review]
        to: in_progress
      close:
        from: [review]
        to: done
        description: "Closing requires a complete review."
        validators:
          - review_complete
          - durable_state_current
"#;

pub const WORKFLOW_POLICY_PATH: &str = ".atelier/workflow.yaml";
const WORKFLOW_SCHEMA: &str = "atelier.workflow";
const WORKFLOW_SCHEMA_VERSION: i64 = 3;
const STATUS_CATEGORIES: &[&str] = &["todo", "active", "blocked", "review", "validation", "done"];
const BUILTIN_ISSUE_TYPES: &[&str] = &["bug", "epic", "feature", "spike", "task", "validation"];
const BUILTIN_VALIDATORS: &[&str] = &[
    "durable_state_current",
    "evidence_attached",
    "review_complete",
    "epic_child_proof_complete",
    "validation_criteria_satisfied",
    "linked_pr_merged",
    "no_open_blockers",
    "no_blocking_lints",
    "git_worktree_clean",
];
const BUILTIN_EFFECTS: &[&str] = &[
    "issue_status_write",
    "owner_branch_commit",
    "owner_branch_integrate",
    "review_artifact_open",
    "review_artifact_link",
];
const ALLOWED_REQUIRED_FIELDS: &[&str] = &["close_reason"];
const DEFERRED_TOP_LEVEL_FIELDS: &[&str] = &[
    "hooks",
    "triggers",
    "post_functions",
    "post-functions",
    "waivers",
    "workflow_projection_tables",
    "projection_tables",
];
const TOP_LEVEL_FIELDS: &[&str] = &[
    "schema",
    "schema_version",
    "branch_policy",
    "statuses",
    "workflows",
];
const ALLOWED_BRANCH_TEMPLATE_VARIABLES: &[&str] = &["issue.id", "issue.type"];

#[derive(Debug, Clone)]
pub struct WorkflowPolicy {
    pub schema_version: i64,
    pub branch_policy: BranchLifecycleConfig,
    pub workflow_by_issue_type: BTreeMap<String, String>,
    pub statuses: BTreeMap<String, StatusDefinition>,
    pub workflows: BTreeMap<String, WorkflowDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchLifecycleConfig {
    pub base_branch: String,
    pub merge_strategy: MergeStrategy,
    pub branch_templates: BranchTemplates,
}

impl Default for BranchLifecycleConfig {
    fn default() -> Self {
        Self {
            base_branch: "main".to_string(),
            merge_strategy: MergeStrategy::Squash,
            branch_templates: BranchTemplates::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchTemplates {
    pub epic: String,
    pub issue: String,
}

impl Default for BranchTemplates {
    fn default() -> Self {
        Self {
            epic: "epic/{{ issue.id }}".to_string(),
            issue: "codex/{{ issue.id }}".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    Squash,
    MergeCommit,
    FastForwardOnly,
}

impl MergeStrategy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Squash => "squash",
            Self::MergeCommit => "merge_commit",
            Self::FastForwardOnly => "fast_forward_only",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchOwnerKind {
    Epic,
    StandaloneIssue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchLifecycleResolution {
    pub issue_id: String,
    pub owner_id: String,
    pub owner_issue_type: String,
    pub owner_kind: BranchOwnerKind,
    pub expected_branch: String,
    pub base_branch: String,
    pub merge_strategy: MergeStrategy,
    pub merge_owned: bool,
    pub nested_under_epic: bool,
}

#[derive(Debug, Clone)]
pub struct StatusDefinition {
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDefinition {
    pub field_type: FieldType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    String,
    Bool,
    Integer,
    Enum { values: Vec<String> },
    Object { required: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct ValidatorDefinition {
    pub builtin: String,
    pub params: Option<ValidatorParams>,
}

#[derive(Debug, Clone)]
pub struct EffectDefinition {
    pub builtin: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorParams {
    EvidenceAttached {
        min_count: i64,
        kind: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct GuidanceTemplate {
    pub template: String,
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
    pub effects: Vec<EffectDefinition>,
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

    pub fn branch_name_for_owner(
        &self,
        owner: &Issue,
        owner_kind: &BranchOwnerKind,
    ) -> Result<String> {
        let template = match owner_kind {
            BranchOwnerKind::Epic => &self.branch_policy.branch_templates.epic,
            BranchOwnerKind::StandaloneIssue => &self.branch_policy.branch_templates.issue,
        };
        render_branch_template(template, owner)
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
    #[serde(default)]
    effects: Vec<Value>,
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
    #[serde(default)]
    branch_templates: Option<BranchTemplatesRaw>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BranchTemplatesRaw {
    #[serde(default)]
    epic: Option<String>,
    #[serde(default)]
    issue: Option<String>,
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

fn parse_policy_text(text: &str, display_path: &str) -> Result<WorkflowPolicy> {
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
    let workflow_by_issue_type = issue_type_mappings_from_workflows(&workflows, display_path)?;

    let policy = WorkflowPolicy {
        schema_version,
        branch_policy,
        workflow_by_issue_type,
        statuses,
        workflows,
    };
    validate_policy(&policy, display_path)?;
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
    let branch_templates = parse_branch_templates(raw.branch_templates, display_path)?;
    Ok(BranchLifecycleConfig {
        base_branch,
        merge_strategy,
        branch_templates,
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

fn parse_branch_templates(
    raw: Option<BranchTemplatesRaw>,
    display_path: &str,
) -> Result<BranchTemplates> {
    let defaults = BranchTemplates::default();
    let raw = raw.unwrap_or(BranchTemplatesRaw {
        epic: None,
        issue: None,
    });
    let templates = BranchTemplates {
        epic: raw.epic.unwrap_or(defaults.epic),
        issue: raw.issue.unwrap_or(defaults.issue),
    };
    validate_branch_template(
        &templates.epic,
        display_path,
        "branch_policy.branch_templates.epic",
    )?;
    validate_branch_template(
        &templates.issue,
        display_path,
        "branch_policy.branch_templates.issue",
    )?;
    Ok(templates)
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
    workflows: &BTreeMap<String, WorkflowDefinition>,
    display_path: &str,
) -> Result<BTreeMap<String, String>> {
    let mut mappings = BTreeMap::new();
    for (workflow_name, workflow) in workflows {
        for issue_type in &workflow.applies_to {
            if !BUILTIN_ISSUE_TYPES.contains(&issue_type.as_str()) {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_issue_type_mapping",
                    display_path,
                    format!("workflows.{}.applies_to", workflow_name),
                    format!(
                        "unsupported built-in issue type '{}'; expected {}",
                        issue_type,
                        BUILTIN_ISSUE_TYPES.join(", ")
                    ),
                ));
            }
            if let Some(existing_workflow) =
                mappings.insert(issue_type.clone(), workflow_name.clone())
            {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_issue_type_mapping",
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
    for issue_type in BUILTIN_ISSUE_TYPES {
        if !mappings.contains_key(*issue_type) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_issue_type_mapping",
                display_path,
                "workflows.*.applies_to",
                format!(
                    "missing workflow applies_to entry for built-in issue type '{}'",
                    issue_type
                ),
            ));
        }
    }
    Ok(mappings)
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
        "evidence_attached" => {
            let Some(params) = params else {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_validator",
                    display_path,
                    format!("validators.{}.params", validator_name),
                    "evidence_attached requires params.min_count >= 1",
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
                    "evidence_attached requires params.min_count >= 1",
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
            "workflow_config_invalid_issue_type_mapping",
            display_path,
            format!("workflows.{}.applies_to", workflow_name),
            "workflow applies_to must contain at least one built-in issue type",
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
        workflow_name,
        transition_name,
        &raw.validators,
        display_path,
    )?;
    let effects = parse_transition_effects(
        workflow_name,
        transition_name,
        &raw.effects,
        &raw.to,
        display_path,
    )?;

    Ok(TransitionDefinition {
        from,
        to: raw.to,
        required_fields: raw.required_fields,
        description: raw.description,
        validators,
        effects,
    })
}

fn parse_transition_effects(
    workflow_name: &str,
    transition_name: &str,
    raw: &[Value],
    to_status: &str,
    display_path: &str,
) -> Result<Vec<EffectDefinition>> {
    let mut effects = Vec::new();
    let mut seen = BTreeSet::new();
    for value in raw {
        let name = value.as_str().ok_or_else(|| {
            policy_error(
                "workflow_config_invalid_effect",
                display_path,
                format!(
                    "workflows.{}.transitions.{}.effects must contain built-in effect strings",
                    workflow_name, transition_name
                ),
            )
        })?;
        validate_builtin_effect_name(workflow_name, transition_name, name, display_path)?;
        if !seen.insert(name.to_string()) {
            return Err(policy_error(
                "workflow_config_invalid_effect",
                display_path,
                format!(
                    "workflows.{}.transitions.{}.effects contains duplicate effect '{}'",
                    workflow_name, transition_name, name
                ),
            ));
        }
        if matches!(name, "review_artifact_open" | "review_artifact_link") && to_status != "review"
        {
            return Err(policy_error(
                "workflow_config_invalid_effect",
                display_path,
                format!(
                    "workflows.{}.transitions.{}.effects declares '{}' but review artifact effects are only supported on transitions to review",
                    workflow_name, transition_name, name
                ),
            ));
        }
        effects.push(EffectDefinition {
            builtin: name.to_string(),
        });
    }
    Ok(effects)
}

fn validate_builtin_effect_name(
    workflow_name: &str,
    transition_name: &str,
    effect_name: &str,
    display_path: &str,
) -> Result<()> {
    if BUILTIN_EFFECTS.contains(&effect_name) {
        return Ok(());
    }
    Err(policy_error(
        "workflow_config_invalid_effect",
        display_path,
        format!(
            "workflows.{}.transitions.{}.effects has unsupported built-in effect '{}'; expected {}",
            workflow_name,
            transition_name,
            effect_name,
            BUILTIN_EFFECTS.join(", ")
        ),
    ))
}

fn parse_transition_validators(
    workflow_name: &str,
    transition_name: &str,
    values: &[Value],
    display_path: &str,
) -> Result<Vec<ValidatorDefinition>> {
    let mut validators = Vec::new();
    for value in values {
        match value {
            Value::String(name) => {
                validate_builtin_validator_name(
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
    workflow_name: &str,
    transition_name: &str,
    name: &str,
    display_path: &str,
) -> Result<()> {
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

fn validate_policy(policy: &WorkflowPolicy, display_path: &str) -> Result<()> {
    validate_workflows(policy, display_path)?;
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
    let workflow_name = policy
        .workflow_by_issue_type
        .get(&issue.issue_type)
        .ok_or_else(|| {
            policy_error_with_field(
                "workflow_config_invalid_issue_type_mapping",
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
            "workflow_config_invalid_issue_type_mapping",
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
    for (field_name, value) in &issue.fields {
        if field_name == "review" {
            validate_review_field(value, issue, policy_path)?;
        } else if field_name == "pull_request" {
            return Err(WorkflowPolicyError {
                code: "workflow_issue_field_legacy",
                path: policy_path.display().to_string(),
                message: format!(
                    "issue {} defines legacy field 'pull_request'; migrate to structured 'review'",
                    issue.id
                ),
                field: Some("pull_request".to_string()),
                reference: Some(issue.id.clone()),
                line: None,
                column: None,
            }
            .into());
        } else {
            return Err(WorkflowPolicyError {
                code: "workflow_issue_field_unknown",
                path: policy_path.display().to_string(),
                message: format!(
                    "issue {} defines field '{}' but schema_version 3 only supports built-in field 'review'",
                    issue.id, field_name
                ),
                field: Some(field_name.to_string()),
                reference: Some(issue.id.clone()),
                line: None,
                column: None,
            }
            .into());
        }
    }
    Ok(())
}

fn validate_review_field(value: &JsonValue, issue: &Issue, policy_path: &Path) -> Result<()> {
    let Some(object) = value.as_object() else {
        return invalid_review_field(issue, policy_path, "field 'review' must be an object");
    };
    let kind = object.get("kind").and_then(JsonValue::as_str).unwrap_or("");
    match kind {
        "room" => {
            let valid = object
                .get("id")
                .and_then(JsonValue::as_str)
                .is_some_and(|id| id.starts_with("atelier-"));
            if valid {
                Ok(())
            } else {
                invalid_review_field(
                    issue,
                    policy_path,
                    "room review must include id: <review-id>",
                )
            }
        }
        "pull_request" => {
            let provider_ok = object.get("provider").and_then(JsonValue::as_str) == Some("forgejo");
            let number_ok = object
                .get("number")
                .and_then(|number| {
                    number
                        .as_u64()
                        .or_else(|| number.as_i64().and_then(|n| u64::try_from(n).ok()))
                })
                .is_some_and(|number| number > 0);
            if provider_ok && number_ok {
                Ok(())
            } else {
                invalid_review_field(
                    issue,
                    policy_path,
                    "provider review must include provider: forgejo and positive number",
                )
            }
        }
        _ => invalid_review_field(
            issue,
            policy_path,
            "field 'review.kind' must be 'room' or 'pull_request'",
        ),
    }
}

fn invalid_review_field(issue: &Issue, policy_path: &Path, reason: &str) -> Result<()> {
    Err(WorkflowPolicyError {
        code: "workflow_issue_field_invalid",
        path: policy_path.display().to_string(),
        message: format!("issue {} field 'review' is invalid: {}", issue.id, reason),
        field: Some("review".to_string()),
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

fn validate_branch_template(template: &str, display_path: &str, field: &str) -> Result<()> {
    if template.trim().is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_branch_policy",
            display_path,
            field,
            "branch template must not be empty",
        ));
    }
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find("}}") else {
            return Err(policy_error_with_field(
                "workflow_config_invalid_branch_policy",
                display_path,
                field,
                "branch template contains '{{' without a matching '}}'",
            ));
        };
        let variable = rest[..end].trim();
        if !ALLOWED_BRANCH_TEMPLATE_VARIABLES.contains(&variable) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_branch_policy",
                display_path,
                field,
                format!(
                    "unsupported branch template variable '{}'; expected {}",
                    variable,
                    ALLOWED_BRANCH_TEMPLATE_VARIABLES.join(", ")
                ),
            ));
        }
        rest = &rest[end + 2..];
    }
    if rest.contains("}}") {
        return Err(policy_error_with_field(
            "workflow_config_invalid_branch_policy",
            display_path,
            field,
            "branch template contains '}}' without a matching '{{'",
        ));
    }
    let rendered = render_branch_template_with(template, "atelier-example", "task")?;
    validate_branch_value(&rendered, display_path, field, "branch template")?;
    Ok(())
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

fn render_branch_template(template: &str, issue: &Issue) -> Result<String> {
    let branch = render_branch_template_with(template, &issue.id, &issue.issue_type)?;
    validate_branch_value(
        &branch,
        WORKFLOW_POLICY_PATH,
        "branch_policy.branch_templates",
        "rendered branch name",
    )?;
    Ok(branch)
}

fn render_branch_template_with(template: &str, issue_id: &str, issue_type: &str) -> Result<String> {
    let mut rendered = String::new();
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        rendered.push_str(&rest[..start]);
        rest = &rest[start + 2..];
        let Some(end) = rest.find("}}") else {
            return Err(policy_error_with_field(
                "workflow_config_invalid_branch_policy",
                WORKFLOW_POLICY_PATH,
                "branch_policy.branch_templates",
                "branch template contains '{{' without a matching '}}'",
            ));
        };
        match rest[..end].trim() {
            "issue.id" => rendered.push_str(issue_id),
            "issue.type" => rendered.push_str(issue_type),
            variable => {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_branch_policy",
                    WORKFLOW_POLICY_PATH,
                    "branch_policy.branch_templates",
                    format!(
                        "unsupported branch template variable '{}'; expected {}",
                        variable,
                        ALLOWED_BRANCH_TEMPLATE_VARIABLES.join(", ")
                    ),
                ));
            }
        }
        rest = &rest[end + 2..];
    }
    rendered.push_str(rest);
    Ok(rendered)
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
        "branch_policy:\n  base_branch: main\n  merge_strategy: squash\n  branch_templates:\n    epic: epic/{{ issue.id }}\n    issue: codex/{{ issue.id }}\n"
    }

    fn configured_policy() -> String {
        valid_policy().replace(
            default_branch_policy(),
            "branch_policy:\n  base_branch: trunk\n  merge_strategy: fast_forward_only\n  branch_templates:\n    epic: review/{{ issue.id }}\n    issue: work/{{ issue.type }}/{{ issue.id }}\n",
        )
    }

    fn issue_with_fields(fields: std::collections::BTreeMap<String, JsonValue>) -> Issue {
        Issue {
            id: "atelier-pr01".to_string(),
            title: "Pull request".to_string(),
            description: None,
            status: "todo".to_string(),
            priority: "medium".to_string(),
            issue_type: "task".to_string(),
            fields,
            parent_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            closed_at: None,
        }
    }

    #[test]
    fn parses_valid_policy() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(policy.schema_version, 3);
        assert_eq!(
            policy
                .workflow_by_issue_type
                .get("task")
                .map(String::as_str),
            Some("standard")
        );
        assert_eq!(
            policy
                .workflow_by_issue_type
                .get("epic")
                .map(String::as_str),
            Some("epic_reviewed")
        );
        assert_eq!(
            policy
                .workflow_by_issue_type
                .get("validation")
                .map(String::as_str),
            Some("validation_reviewed")
        );
        assert_eq!(
            policy
                .statuses
                .get("done")
                .map(|status| status.category.as_str()),
            Some("done")
        );
        let close = &policy.workflows["standard"].transitions["close"];
        assert_eq!(close.required_fields, vec!["close_reason".to_string()]);
        assert_eq!(
            close.validators[0].params.as_ref(),
            Some(&ValidatorParams::EvidenceAttached {
                min_count: 1,
                kind: None,
            })
        );
        assert_eq!(
            effect_names(&policy.workflows["epic_reviewed"].transitions["request_review"].effects),
            vec!["review_artifact_open"]
        );
        assert_eq!(policy.branch_policy.merge_strategy, MergeStrategy::Squash);
        assert_eq!(policy.branch_policy.base_branch, "main");
    }

    fn effect_names(effects: &[EffectDefinition]) -> Vec<&str> {
        effects
            .iter()
            .map(|effect| effect.builtin.as_str())
            .collect()
    }

    #[test]
    fn rejects_unknown_transition_effect() {
        let policy =
            valid_policy().replace("          - review_artifact_open", "          - nope_run");
        let error = parse_policy_text(&policy, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_invalid_effect"));
        assert!(error.contains("nope_run"));
    }

    #[test]
    fn rejects_duplicate_transition_effect() {
        let policy = valid_policy().replace(
            "          - review_artifact_open",
            "          - review_artifact_open\n          - review_artifact_open",
        );
        let error = parse_policy_text(&policy, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_invalid_effect"));
        assert!(error.contains("duplicate effect"));
    }

    #[test]
    fn rejects_review_effect_on_non_review_transition() {
        let policy = valid_policy().replace(
            "      close:\n        from: [in_progress, validation]",
            "      close:\n        from: [in_progress, validation]\n        effects: [review_artifact_open]",
        );
        let error = parse_policy_text(&policy, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_invalid_effect"));
        assert!(error.contains("transitions to review"));
    }

    #[test]
    fn starter_policy_does_not_require_legacy_pr_merge_gate() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();

        for issue_type in BUILTIN_ISSUE_TYPES {
            let workflow_name = policy.workflow_by_issue_type.get(*issue_type).unwrap();
            let close_validators = policy.workflows[workflow_name]
                .transitions
                .get("close")
                .map(|transition| {
                    transition
                        .validators
                        .iter()
                        .map(|validator| validator.builtin.as_str())
                        .collect::<Vec<_>>()
                })
                .unwrap();
            let has_linked_pr_merged = close_validators.contains(&"linked_pr_merged");
            assert_eq!(
                has_linked_pr_merged,
                false,
                "unexpected legacy linked_pr_merged close validator for {issue_type}: {close_validators:?}"
            );
        }
    }

    #[test]
    fn validates_review_field_shape() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut fields = std::collections::BTreeMap::new();
        fields.insert(
            "review".to_string(),
            serde_json::json!({"kind": "pull_request", "provider": "forgejo", "number": 42}),
        );
        let issue = issue_with_fields(fields);

        validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH)).unwrap();
    }

    #[test]
    fn rejects_mismatched_review_field_shape() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut fields = std::collections::BTreeMap::new();
        fields.insert(
            "review".to_string(),
            serde_json::json!({"kind": "pull_request", "provider": "forgejo"}),
        );
        let issue = issue_with_fields(fields);

        let error = validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH))
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_issue_field_invalid"));
        assert!(error.contains("review"));
        assert!(error.contains("positive number"));
    }

    #[test]
    fn rejects_legacy_pull_request_field_shape() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut fields = std::collections::BTreeMap::new();
        fields.insert("pull_request".to_string(), serde_json::json!(42));
        let issue = issue_with_fields(fields);

        let error = validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH))
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_issue_field_legacy"));
        assert!(error.contains("pull_request"));
    }

    #[test]
    fn rejects_removed_top_level_fields() {
        let text = format!("{}\nissue_types: {{}}\n", valid_policy());
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_config_unknown_field"));
        assert!(error.contains("issue_types"));
    }

    #[test]
    fn parses_configured_branch_policy() {
        let policy = parse_policy_text(&configured_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(policy.branch_policy.base_branch, "trunk");
        assert_eq!(
            policy.branch_policy.merge_strategy,
            MergeStrategy::FastForwardOnly
        );
        assert_eq!(
            policy.branch_policy.branch_templates.epic,
            "review/{{ issue.id }}"
        );
        assert_eq!(
            policy.branch_policy.branch_templates.issue,
            "work/{{ issue.type }}/{{ issue.id }}"
        );
    }

    #[test]
    fn rejects_configured_branch_policy_without_base_branch() {
        let text = valid_policy().replace(
            default_branch_policy(),
            "branch_policy:\n  merge_strategy: squash\n",
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
    fn branch_name_for_owner_renders_configured_templates() {
        let policy = parse_policy_text(&configured_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let issue = Issue {
            id: "atelier-abc1".to_string(),
            title: "Branch owner".to_string(),
            description: None,
            status: "todo".to_string(),
            priority: "medium".to_string(),
            issue_type: "task".to_string(),
            fields: Default::default(),
            parent_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            closed_at: None,
        };

        let branch = policy
            .branch_name_for_owner(&issue, &BranchOwnerKind::StandaloneIssue)
            .unwrap();

        assert_eq!(branch, "work/task/atelier-abc1");
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
            &valid_policy().replace(
                "          - no_open_blockers\n",
                "          - missing_validator\n",
            ),
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
            &valid_policy().replace("    applies_to: [spike]\n", "    applies_to: []\n"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_issue_type_mapping"));
        assert!(error.contains("spike"));
    }

    #[test]
    fn rejects_invalid_evidence_validator_params() {
        let error = parse_policy_text(
            &valid_policy().replace("min_count: 1", "min_count: 0"),
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
            &valid_policy().replace(
                "    applies_to: [spike]\n",
                "    applies_to: [spike, task]\n",
            ),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_issue_type_mapping"));
        assert!(error.contains("task"));
    }
}
