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
schema_version: 1

branch_lifecycle:
  base_branch: main
  merge_strategy: squash
  branch_templates:
    epic: epic/{{ issue.id }}
    issue: codex/{{ issue.id }}

issue_types:
  bug: standard_proof
  epic: standard_review_proof
  feature: standard_proof
  spike: lightweight_spike
  task: standard_proof
  validation: standard_review_proof

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

validators:
  durable_current:
    builtin: durable_state_current
  review_ready:
    builtin: review_complete
  proof_attached:
    builtin: evidence_attached
    params:
      min_count: 1
  blockers_clear:
    builtin: no_open_blockers
  epic_child_proof:
    builtin: epic_child_proof_complete
  lint_clear:
    builtin: no_blocking_lints
  closeout_clean:
    builtin: git_worktree_clean

guidance_templates:
  close_with_proof:
    format: markdown
    template: |
      Closing {{ issue.id }} requires attached evidence and no open blockers.
  record_spike_outcome:
    format: markdown
    template: |
      Record a concise close reason that captures what {{ issue.id }} learned
      and what follow-up work remains.

workflows:
  standard_proof:
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
        validators:
          - proof_attached
          - blockers_clear
          - lint_clear
          - durable_current
        guidance: [close_with_proof]

  standard_review_proof:
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
      request_validation:
        from: [in_progress, review]
        to: validation
        validators: [review_ready]
      close:
        from: [validation]
        to: done
        required_fields: [close_reason]
        validators:
          - proof_attached
          - epic_child_proof
          - blockers_clear
          - lint_clear
          - durable_current
          - closeout_clean
        guidance: [close_with_proof]

  lightweight_spike:
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
        required_fields: [close_reason]
        validators:
          - review_ready
          - durable_current
        guidance: [record_spike_outcome]
"#;

pub const WORKFLOW_POLICY_PATH: &str = ".atelier/workflow.yaml";
const WORKFLOW_SCHEMA: &str = "atelier.workflow";
const WORKFLOW_SCHEMA_VERSION: i64 = 1;
const WORKFLOW_SCHEMA_VERSION_V2: i64 = 2;
const STATUS_CATEGORIES: &[&str] = &["todo", "active", "blocked", "review", "validation", "done"];
const BUILTIN_ISSUE_TYPES: &[&str] = &["bug", "epic", "feature", "spike", "task", "validation"];
const BUILTIN_VALIDATORS: &[&str] = &[
    "durable_state_current",
    "evidence_attached",
    "review_complete",
    "epic_child_proof_complete",
    "validation_criteria_satisfied",
    "no_open_blockers",
    "no_blocking_lints",
    "git_worktree_clean",
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
    "branch_lifecycle",
    "issue_types",
    "statuses",
    "fields",
    "validators",
    "guidance_templates",
    "workflows",
];
const ALLOWED_TEMPLATE_VARIABLES: &[&str] = &[
    "issue.id",
    "issue.type",
    "transition.name",
    "transition.from",
    "transition.to",
];
const ALLOWED_BRANCH_TEMPLATE_VARIABLES: &[&str] = &["issue.id", "issue.type"];

#[derive(Debug, Clone)]
pub struct WorkflowPolicy {
    pub schema_version: i64,
    pub branch_lifecycle: BranchLifecycleConfig,
    pub issue_types: BTreeMap<String, String>,
    pub statuses: BTreeMap<String, StatusDefinition>,
    pub fields: BTreeMap<String, FieldDefinition>,
    pub validators: BTreeMap<String, ValidatorDefinition>,
    pub guidance_templates: BTreeMap<String, GuidanceTemplate>,
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
    pub initial_status: String,
    pub done_statuses: Vec<String>,
    pub transitions: BTreeMap<String, TransitionDefinition>,
}

#[derive(Debug, Clone)]
pub struct TransitionDefinition {
    pub from: Vec<String>,
    pub to: String,
    pub required_fields: Vec<String>,
    pub validators: Vec<String>,
    pub guidance: Vec<String>,
}

impl WorkflowPolicy {
    pub fn workflow_name_for_issue_type(&self, issue_type: &str) -> Result<&str> {
        self.issue_types
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
            BranchOwnerKind::Epic => &self.branch_lifecycle.branch_templates.epic,
            BranchOwnerKind::StandaloneIssue => &self.branch_lifecycle.branch_templates.issue,
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
struct FieldDefinitionRaw {
    #[serde(rename = "type")]
    field_type: String,
    #[serde(default)]
    values: Vec<String>,
    #[serde(default)]
    required: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidatorDefinitionRaw {
    builtin: String,
    #[serde(default)]
    params: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GuidanceTemplateRaw {
    format: String,
    template: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkflowDefinitionRaw {
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
    validators: Vec<String>,
    #[serde(default)]
    guidance: Vec<String>,
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

    let issue_types = parse_issue_types(
        require_mapping(
            root,
            display_path,
            "issue_types",
            "workflow_config_invalid_issue_type_mapping",
            "issue_types must be a mapping of built-in issue types to workflow names",
        )?,
        display_path,
    )?;
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
    let fields = if let Some(value) = root.get("fields") {
        parse_fields(
            value.as_mapping().ok_or_else(|| {
                policy_error_with_field(
                    "workflow_config_invalid_field",
                    display_path,
                    "fields",
                    "fields must be a mapping of field names to field definitions",
                )
            })?,
            display_path,
        )?
    } else {
        BTreeMap::new()
    };
    let validators = parse_validators(
        require_mapping(
            root,
            display_path,
            "validators",
            "workflow_config_invalid_validator",
            "validators must be a mapping of validator names to validator definitions",
        )?,
        display_path,
    )?;
    let guidance_templates = parse_guidance_templates(
        require_mapping(
            root,
            display_path,
            "guidance_templates",
            "workflow_config_invalid_guidance_template",
            "guidance_templates must be a mapping of guidance names to template definitions",
        )?,
        display_path,
    )?;
    let branch_lifecycle = parse_branch_lifecycle(root.get("branch_lifecycle"), display_path)?;
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

    let policy = WorkflowPolicy {
        schema_version,
        branch_lifecycle,
        issue_types,
        statuses,
        fields,
        validators,
        guidance_templates,
        workflows,
    };
    validate_policy(&policy, display_path)?;
    Ok(policy)
}

fn parse_branch_lifecycle(
    value: Option<&Value>,
    display_path: &str,
) -> Result<BranchLifecycleConfig> {
    let Some(value) = value else {
        return Ok(BranchLifecycleConfig::default());
    };
    let raw = deserialize_entry::<BranchLifecycleRaw>(
        value,
        display_path,
        "branch_lifecycle",
        "workflow_config_invalid_branch_lifecycle",
    )?;
    let Some(base_branch) = raw.base_branch else {
        return Err(policy_error_with_field(
            "workflow_config_invalid_branch_lifecycle",
            display_path,
            "branch_lifecycle.base_branch",
            "branch_lifecycle.base_branch is required when branch_lifecycle is configured",
        ));
    };
    validate_branch_value(
        &base_branch,
        display_path,
        "branch_lifecycle.base_branch",
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
            "workflow_config_invalid_branch_lifecycle",
            display_path,
            "branch_lifecycle.merge_strategy",
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
        "branch_lifecycle.branch_templates.epic",
    )?;
    validate_branch_template(
        &templates.issue,
        display_path,
        "branch_lifecycle.branch_templates.issue",
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

fn check_top_level_fields(root: &Mapping, display_path: &str, schema_version: i64) -> Result<()> {
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
        if key == "fields" && schema_version < WORKFLOW_SCHEMA_VERSION_V2 {
            return Err(policy_error_with_field(
                "workflow_config_schema_unsupported",
                display_path,
                key,
                "top-level field 'fields' requires workflow schema_version 2",
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
        if schema_version == WORKFLOW_SCHEMA_VERSION_V2 {
            return Ok(schema_version);
        }
        return Err(policy_error_with_field(
            "workflow_config_schema_unsupported",
            display_path,
            "schema_version",
            format!(
                "unsupported workflow schema_version {}; expected {} or {}",
                schema_version, WORKFLOW_SCHEMA_VERSION, WORKFLOW_SCHEMA_VERSION_V2
            ),
        ));
    }
    Ok(schema_version)
}

fn parse_issue_types(mapping: &Mapping, display_path: &str) -> Result<BTreeMap<String, String>> {
    let mut parsed = BTreeMap::new();
    for (key, value) in mapping {
        let Some(issue_type) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_issue_type_mapping",
                display_path,
                "issue_types keys must be strings",
            ));
        };
        if !BUILTIN_ISSUE_TYPES.contains(&issue_type) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_issue_type_mapping",
                display_path,
                format!("issue_types.{}", issue_type),
                format!(
                    "unsupported built-in issue type '{}'; expected {}",
                    issue_type,
                    BUILTIN_ISSUE_TYPES.join(", ")
                ),
            ));
        }
        let Some(workflow_name) = value.as_str() else {
            return Err(policy_error_with_field(
                "workflow_config_invalid_issue_type_mapping",
                display_path,
                format!("issue_types.{}", issue_type),
                "issue type mappings must point to a workflow name string",
            ));
        };
        parsed.insert(issue_type.to_string(), workflow_name.to_string());
    }
    for issue_type in BUILTIN_ISSUE_TYPES {
        if !parsed.contains_key(*issue_type) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_issue_type_mapping",
                display_path,
                format!("issue_types.{}", issue_type),
                format!(
                    "missing workflow mapping for built-in issue type '{}'",
                    issue_type
                ),
            ));
        }
    }
    Ok(parsed)
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

fn parse_fields(
    mapping: &Mapping,
    display_path: &str,
) -> Result<BTreeMap<String, FieldDefinition>> {
    let mut fields = BTreeMap::new();
    for (key, value) in mapping {
        let Some(name) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_field",
                display_path,
                "fields keys must be strings",
            ));
        };
        ensure_identifier(
            name,
            display_path,
            &format!("fields.{}", name),
            "workflow_config_invalid_field",
            "field names",
        )?;
        let raw = deserialize_entry::<FieldDefinitionRaw>(
            value,
            display_path,
            &format!("fields.{}", name),
            "workflow_config_invalid_field",
        )?;
        fields.insert(
            name.to_string(),
            parse_field_definition(name, raw, display_path)?,
        );
    }
    Ok(fields)
}

fn parse_field_definition(
    field_name: &str,
    raw: FieldDefinitionRaw,
    display_path: &str,
) -> Result<FieldDefinition> {
    let field = format!("fields.{}", field_name);
    let field_type = match raw.field_type.as_str() {
        "string" => {
            reject_field_values(field_name, &raw, display_path)?;
            FieldType::String
        }
        "bool" => {
            reject_field_values(field_name, &raw, display_path)?;
            FieldType::Bool
        }
        "integer" => {
            reject_field_values(field_name, &raw, display_path)?;
            FieldType::Integer
        }
        "enum" => {
            if raw.required.is_empty() {
                validate_string_list(&raw.values, display_path, &format!("{field}.values"))?;
                if raw.values.is_empty() {
                    return Err(policy_error_with_field(
                        "workflow_config_invalid_field",
                        display_path,
                        format!("{field}.values"),
                        "enum fields require at least one value",
                    ));
                }
                FieldType::Enum { values: raw.values }
            } else {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_field",
                    display_path,
                    format!("{field}.required"),
                    "enum fields do not accept required keys",
                ));
            }
        }
        "object" => {
            if !raw.values.is_empty() {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_field",
                    display_path,
                    format!("{field}.values"),
                    "object fields do not accept enum values",
                ));
            }
            validate_string_list(&raw.required, display_path, &format!("{field}.required"))?;
            if raw.required.is_empty() {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_field",
                    display_path,
                    format!("{field}.required"),
                    "object fields require at least one required key",
                ));
            }
            FieldType::Object {
                required: raw.required,
            }
        }
        other => {
            return Err(policy_error_with_field(
                "workflow_config_invalid_field",
                display_path,
                format!("{field}.type"),
                format!(
                    "unsupported field type '{}'; expected string, bool, integer, enum, or object",
                    other
                ),
            ));
        }
    };
    Ok(FieldDefinition { field_type })
}

fn reject_field_values(
    field_name: &str,
    raw: &FieldDefinitionRaw,
    display_path: &str,
) -> Result<()> {
    if !raw.values.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_field",
            display_path,
            format!("fields.{}.values", field_name),
            format!("{} fields do not accept enum values", raw.field_type),
        ));
    }
    if !raw.required.is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_field",
            display_path,
            format!("fields.{}.required", field_name),
            format!("{} fields do not accept required keys", raw.field_type),
        ));
    }
    Ok(())
}

fn validate_string_list(values: &[String], display_path: &str, field: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        ensure_identifier(
            value,
            display_path,
            field,
            "workflow_config_invalid_field",
            "field values",
        )?;
        if !seen.insert(value.as_str()) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_field",
                display_path,
                field,
                format!("duplicate field value '{}'", value),
            ));
        }
    }
    Ok(())
}

fn parse_validators(
    mapping: &Mapping,
    display_path: &str,
) -> Result<BTreeMap<String, ValidatorDefinition>> {
    let mut validators = BTreeMap::new();
    for (key, value) in mapping {
        let Some(name) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_validator",
                display_path,
                "validators keys must be strings",
            ));
        };
        ensure_identifier(
            name,
            display_path,
            &format!("validators.{}", name),
            "workflow_config_invalid_validator",
            "validator names",
        )?;
        let raw = deserialize_entry::<ValidatorDefinitionRaw>(
            value,
            display_path,
            &format!("validators.{}", name),
            "workflow_config_invalid_validator",
        )?;
        if !BUILTIN_VALIDATORS.contains(&raw.builtin.as_str()) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_validator",
                display_path,
                format!("validators.{}.builtin", name),
                format!(
                    "unsupported built-in validator '{}'; expected {}",
                    raw.builtin,
                    BUILTIN_VALIDATORS.join(", ")
                ),
            ));
        }
        let params = parse_validator_params(name, &raw.builtin, raw.params, display_path)?;
        validators.insert(
            name.to_string(),
            ValidatorDefinition {
                builtin: raw.builtin,
                params,
            },
        );
    }
    Ok(validators)
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

fn parse_guidance_templates(
    mapping: &Mapping,
    display_path: &str,
) -> Result<BTreeMap<String, GuidanceTemplate>> {
    let mut templates = BTreeMap::new();
    for (key, value) in mapping {
        let Some(name) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_guidance_template",
                display_path,
                "guidance_templates keys must be strings",
            ));
        };
        ensure_identifier(
            name,
            display_path,
            &format!("guidance_templates.{}", name),
            "workflow_config_invalid_guidance_template",
            "guidance template names",
        )?;
        let raw = deserialize_entry::<GuidanceTemplateRaw>(
            value,
            display_path,
            &format!("guidance_templates.{}", name),
            "workflow_config_invalid_guidance_template",
        )?;
        if raw.format != "markdown" {
            return Err(policy_error_with_field(
                "workflow_config_invalid_guidance_template",
                display_path,
                format!("guidance_templates.{}.format", name),
                format!(
                    "unsupported guidance template format '{}'; expected markdown",
                    raw.format
                ),
            ));
        }
        validate_template_syntax(display_path, name, &raw.template)?;
        templates.insert(
            name.to_string(),
            GuidanceTemplate {
                template: raw.template,
            },
        );
    }
    Ok(templates)
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

    Ok(TransitionDefinition {
        from,
        to: raw.to,
        required_fields: raw.required_fields,
        validators: raw.validators,
        guidance: raw.guidance,
    })
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
    validate_issue_type_mappings(policy, display_path)?;
    validate_workflows(policy, display_path)?;
    Ok(())
}

fn validate_issue_type_mappings(policy: &WorkflowPolicy, display_path: &str) -> Result<()> {
    for (issue_type, workflow_name) in &policy.issue_types {
        if !policy.workflows.contains_key(workflow_name) {
            return Err(policy_error_with_reference(
                "workflow_config_invalid_issue_type_mapping",
                display_path,
                format!("issue_types.{}", issue_type),
                workflow_name,
                format!(
                    "issue type '{}' points to undefined workflow '{}'",
                    issue_type, workflow_name
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
        validate_transition_references(
            policy,
            workflow_name,
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

fn validate_transition_references(
    policy: &WorkflowPolicy,
    workflow_name: &str,
    transition_name: &str,
    transition: &TransitionDefinition,
    display_path: &str,
) -> Result<()> {
    for validator_name in &transition.validators {
        if !policy.validators.contains_key(validator_name) {
            return Err(policy_error_with_reference(
                "workflow_config_unknown_reference",
                display_path,
                format!(
                    "workflows.{}.transitions.{}.validators",
                    workflow_name, transition_name
                ),
                validator_name,
                format!(
                    "transition '{}' in workflow '{}' references undefined validator '{}'",
                    transition_name, workflow_name, validator_name
                ),
            ));
        }
    }
    for guidance_name in &transition.guidance {
        if !policy.guidance_templates.contains_key(guidance_name) {
            return Err(policy_error_with_reference(
                "workflow_config_unknown_reference",
                display_path,
                format!(
                    "workflows.{}.transitions.{}.guidance",
                    workflow_name, transition_name
                ),
                guidance_name,
                format!(
                    "transition '{}' in workflow '{}' references undefined guidance template '{}'",
                    transition_name, workflow_name, guidance_name
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
    let workflow_name = policy.issue_types.get(&issue.issue_type).ok_or_else(|| {
        policy_error_with_field(
            "workflow_config_invalid_issue_type_mapping",
            WORKFLOW_POLICY_PATH,
            format!("issue_types.{}", issue.issue_type),
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
            format!("issue_types.{}", issue.issue_type),
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
    policy: &WorkflowPolicy,
    issue: &Issue,
    policy_path: &Path,
) -> Result<()> {
    for (field_name, value) in &issue.fields {
        let Some(definition) = policy.fields.get(field_name) else {
            return Err(WorkflowPolicyError {
                code: "workflow_issue_field_unknown",
                path: policy_path.display().to_string(),
                message: format!(
                    "issue {} defines field '{}' but .atelier/workflow.yaml does not define it",
                    issue.id, field_name
                ),
                field: Some(format!("fields.{field_name}")),
                reference: Some(issue.id.clone()),
                line: None,
                column: None,
            }
            .into());
        };
        validate_issue_field_value(field_name, value, definition, issue, policy_path)?;
    }
    Ok(())
}

fn validate_issue_field_value(
    field_name: &str,
    value: &JsonValue,
    definition: &FieldDefinition,
    issue: &Issue,
    policy_path: &Path,
) -> Result<()> {
    let valid = match &definition.field_type {
        FieldType::String => value.is_string(),
        FieldType::Bool => value.is_boolean(),
        FieldType::Integer => value.as_i64().is_some() || value.as_u64().is_some(),
        FieldType::Enum { values } => value
            .as_str()
            .is_some_and(|value| values.iter().any(|allowed| allowed == value)),
        FieldType::Object { required } => {
            let Some(object) = value.as_object() else {
                return Err(issue_field_error(
                    policy_path,
                    issue,
                    field_name,
                    "must be an object",
                ));
            };
            for required_key in required {
                if !object.contains_key(required_key) {
                    return Err(issue_field_error(
                        policy_path,
                        issue,
                        field_name,
                        format!("is missing required key '{required_key}'"),
                    ));
                }
            }
            true
        }
    };
    if valid {
        if field_name == "forge_pr" {
            validate_forge_pr_field(value, issue, policy_path)?;
        }
        Ok(())
    } else {
        Err(issue_field_error(
            policy_path,
            issue,
            field_name,
            format!(
                "does not match workflow field type {}",
                field_type_name(&definition.field_type)
            ),
        ))
    }
}

fn validate_forge_pr_field(value: &JsonValue, issue: &Issue, policy_path: &Path) -> Result<()> {
    let Some(object) = value.as_object() else {
        return Ok(());
    };
    let provider = require_forge_pr_string(object, "provider", issue, policy_path)?;
    if provider != "forgejo" {
        return Err(issue_field_key_error(
            policy_path,
            issue,
            "provider",
            "must be 'forgejo'",
        ));
    }
    let host = require_forge_pr_string(object, "host", issue, policy_path)?;
    require_forge_pr_string(object, "owner", issue, policy_path)?;
    require_forge_pr_string(object, "repo", issue, policy_path)?;
    let number = require_forge_pr_number(object, issue, policy_path)?;
    let url = require_forge_pr_string(object, "url", issue, policy_path)?;
    if !url.contains(host) {
        return Err(issue_field_key_error(
            policy_path,
            issue,
            "url",
            "must contain the forge_pr host",
        ));
    }
    if !url.contains(&number.to_string()) {
        return Err(issue_field_key_error(
            policy_path,
            issue,
            "url",
            "must contain the forge_pr number",
        ));
    }
    require_forge_pr_string(object, "source_branch", issue, policy_path)?;
    require_forge_pr_string(object, "target_branch", issue, policy_path)?;
    Ok(())
}

fn require_forge_pr_string<'a>(
    object: &'a serde_json::Map<String, JsonValue>,
    key: &str,
    issue: &Issue,
    policy_path: &Path,
) -> Result<&'a str> {
    let Some(value) = object.get(key).and_then(JsonValue::as_str) else {
        return Err(issue_field_key_error(
            policy_path,
            issue,
            key,
            "must be a string",
        ));
    };
    if value.trim().is_empty() {
        return Err(issue_field_key_error(
            policy_path,
            issue,
            key,
            "must not be empty",
        ));
    }
    Ok(value)
}

fn require_forge_pr_number(
    object: &serde_json::Map<String, JsonValue>,
    issue: &Issue,
    policy_path: &Path,
) -> Result<u64> {
    let number = object
        .get("number")
        .and_then(|value| {
            value
                .as_u64()
                .or_else(|| value.as_i64().and_then(|n| n.try_into().ok()))
        })
        .filter(|number| *number > 0)
        .ok_or_else(|| {
            issue_field_key_error(policy_path, issue, "number", "must be a positive integer")
        })?;
    Ok(number)
}

fn issue_field_key_error(
    policy_path: &Path,
    issue: &Issue,
    key: &str,
    message: impl Into<String>,
) -> anyhow::Error {
    WorkflowPolicyError {
        code: "workflow_issue_field_invalid",
        path: policy_path.display().to_string(),
        message: format!(
            "issue {} field 'forge_pr.{}' {}",
            issue.id,
            key,
            message.into()
        ),
        field: Some(format!("fields.forge_pr.{key}")),
        reference: Some(issue.id.clone()),
        line: None,
        column: None,
    }
    .into()
}

fn issue_field_error(
    policy_path: &Path,
    issue: &Issue,
    field_name: &str,
    message: impl Into<String>,
) -> anyhow::Error {
    WorkflowPolicyError {
        code: "workflow_issue_field_invalid",
        path: policy_path.display().to_string(),
        message: format!(
            "issue {} field '{}' {}",
            issue.id,
            field_name,
            message.into()
        ),
        field: Some(format!("fields.{field_name}")),
        reference: Some(issue.id.clone()),
        line: None,
        column: None,
    }
    .into()
}

fn field_type_name(field_type: &FieldType) -> &'static str {
    match field_type {
        FieldType::String => "string",
        FieldType::Bool => "bool",
        FieldType::Integer => "integer",
        FieldType::Enum { .. } => "enum",
        FieldType::Object { .. } => "object",
    }
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

fn validate_template_syntax(display_path: &str, name: &str, template: &str) -> Result<()> {
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find("}}") else {
            return Err(policy_error_with_field(
                "workflow_config_invalid_guidance_template",
                display_path,
                format!("guidance_templates.{}.template", name),
                "template contains '{{' without a matching '}}'",
            ));
        };
        let variable = rest[..end].trim();
        if !ALLOWED_TEMPLATE_VARIABLES.contains(&variable) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_guidance_template",
                display_path,
                format!("guidance_templates.{}.template", name),
                format!(
                    "unsupported template variable '{}'; expected {}",
                    variable,
                    ALLOWED_TEMPLATE_VARIABLES.join(", ")
                ),
            ));
        }
        rest = &rest[end + 2..];
    }
    if rest.contains("}}") {
        return Err(policy_error_with_field(
            "workflow_config_invalid_guidance_template",
            display_path,
            format!("guidance_templates.{}.template", name),
            "template contains '}}' without a matching '{{'",
        ));
    }
    Ok(())
}

fn validate_branch_template(template: &str, display_path: &str, field: &str) -> Result<()> {
    if template.trim().is_empty() {
        return Err(policy_error_with_field(
            "workflow_config_invalid_branch_lifecycle",
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
                "workflow_config_invalid_branch_lifecycle",
                display_path,
                field,
                "branch template contains '{{' without a matching '}}'",
            ));
        };
        let variable = rest[..end].trim();
        if !ALLOWED_BRANCH_TEMPLATE_VARIABLES.contains(&variable) {
            return Err(policy_error_with_field(
                "workflow_config_invalid_branch_lifecycle",
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
            "workflow_config_invalid_branch_lifecycle",
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
            "workflow_config_invalid_branch_lifecycle",
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
        "branch_lifecycle.branch_templates",
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
                "workflow_config_invalid_branch_lifecycle",
                WORKFLOW_POLICY_PATH,
                "branch_lifecycle.branch_templates",
                "branch template contains '{{' without a matching '}}'",
            ));
        };
        match rest[..end].trim() {
            "issue.id" => rendered.push_str(issue_id),
            "issue.type" => rendered.push_str(issue_type),
            variable => {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_branch_lifecycle",
                    WORKFLOW_POLICY_PATH,
                    "branch_lifecycle.branch_templates",
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

    fn default_branch_lifecycle() -> &'static str {
        "branch_lifecycle:\n  base_branch: main\n  merge_strategy: squash\n  branch_templates:\n    epic: epic/{{ issue.id }}\n    issue: codex/{{ issue.id }}\n"
    }

    fn configured_policy() -> String {
        valid_policy().replace(
            default_branch_lifecycle(),
            "branch_lifecycle:\n  base_branch: trunk\n  merge_strategy: fast_forward_only\n  branch_templates:\n    epic: review/{{ issue.id }}\n    issue: work/{{ issue.type }}/{{ issue.id }}\n",
        )
    }

    fn forge_pr_policy() -> WorkflowPolicy {
        let text = valid_policy().replacen("schema_version: 1", "schema_version: 2", 1)
            + r#"
fields:
  forge_pr:
    type: object
    required: [provider, host, owner, repo, number, url, source_branch, target_branch]
"#;
        parse_policy_text(&text, WORKFLOW_POLICY_PATH).unwrap()
    }

    fn issue_with_fields(fields: std::collections::BTreeMap<String, JsonValue>) -> Issue {
        Issue {
            id: "atelier-fpr1".to_string(),
            title: "Forge PR".to_string(),
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
        assert_eq!(policy.schema_version, 1);
        assert_eq!(
            policy.issue_types.get("task").map(String::as_str),
            Some("standard_proof")
        );
        assert_eq!(
            policy.issue_types.get("epic").map(String::as_str),
            Some("standard_review_proof")
        );
        assert_eq!(
            policy
                .statuses
                .get("done")
                .map(|status| status.category.as_str()),
            Some("done")
        );
        assert_eq!(
            policy
                .validators
                .get("proof_attached")
                .and_then(|validator| validator.params.as_ref()),
            Some(&ValidatorParams::EvidenceAttached {
                min_count: 1,
                kind: None,
            })
        );
        assert_eq!(
            policy.branch_lifecycle.merge_strategy,
            MergeStrategy::Squash
        );
        assert_eq!(policy.branch_lifecycle.base_branch, "main");
    }

    #[test]
    fn parses_schema_version_2_field_definitions() {
        let text = valid_policy().replacen("schema_version: 1", "schema_version: 2", 1)
            + r#"
fields:
  release_note:
    type: string
  approved:
    type: bool
  retry_count:
    type: integer
  pr_state:
    type: enum
    values: [open, merged]
  forge_pr:
    type: object
    required: [provider, host, owner, repo, number, url, source_branch, target_branch]
"#;
        let policy = parse_policy_text(&text, WORKFLOW_POLICY_PATH).unwrap();

        assert_eq!(policy.schema_version, 2);
        assert_eq!(
            policy
                .fields
                .get("release_note")
                .map(|field| &field.field_type),
            Some(&FieldType::String)
        );
        assert_eq!(
            policy.fields.get("approved").map(|field| &field.field_type),
            Some(&FieldType::Bool)
        );
        assert_eq!(
            policy
                .fields
                .get("retry_count")
                .map(|field| &field.field_type),
            Some(&FieldType::Integer)
        );
        assert_eq!(
            policy.fields.get("pr_state").map(|field| &field.field_type),
            Some(&FieldType::Enum {
                values: vec!["open".to_string(), "merged".to_string()],
            })
        );
        assert_eq!(
            policy.fields.get("forge_pr").map(|field| &field.field_type),
            Some(&FieldType::Object {
                required: vec![
                    "provider".to_string(),
                    "host".to_string(),
                    "owner".to_string(),
                    "repo".to_string(),
                    "number".to_string(),
                    "url".to_string(),
                    "source_branch".to_string(),
                    "target_branch".to_string(),
                ],
            })
        );
    }

    #[test]
    fn validates_forge_pr_field_shape() {
        let policy = forge_pr_policy();
        let mut fields = std::collections::BTreeMap::new();
        fields.insert(
            "forge_pr".to_string(),
            serde_json::json!({
                "provider": "forgejo",
                "host": "forge.example.test",
                "owner": "tools",
                "repo": "atelier",
                "number": 42,
                "url": "https://forge.example.test/tools/atelier/pulls/42",
                "source_branch": "codex/atelier-fpr1",
                "target_branch": "master"
            }),
        );
        let issue = issue_with_fields(fields);

        validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH)).unwrap();
    }

    #[test]
    fn rejects_mismatched_forge_pr_field_shape() {
        let policy = forge_pr_policy();
        let mut fields = std::collections::BTreeMap::new();
        fields.insert(
            "forge_pr".to_string(),
            serde_json::json!({
                "provider": "github",
                "host": "forge.example.test",
                "owner": "tools",
                "repo": "atelier",
                "number": 42,
                "url": "https://forge.example.test/tools/atelier/pulls/42",
                "source_branch": "codex/atelier-fpr1",
                "target_branch": "master"
            }),
        );
        let issue = issue_with_fields(fields);

        let error = validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH))
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_issue_field_invalid"));
        assert!(error.contains("fields.forge_pr.provider"));
        assert!(error.contains("must be 'forgejo'"));
    }

    #[test]
    fn rejects_fields_in_schema_version_1_policy() {
        let text = format!(
            "{}\nfields:\n  forge_pr:\n    type: object\n    required: [provider]\n",
            valid_policy()
        );
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_config_schema_unsupported"));
        assert!(error.contains("fields"));
        assert!(error.contains("schema_version 2"));
    }

    #[test]
    fn rejects_invalid_schema_version_2_field_definition() {
        let text = valid_policy().replacen("schema_version: 1", "schema_version: 2", 1)
            + "\nfields:\n  forge-pr:\n    type: object\n    required: []\n";
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_config_invalid_field"));
        assert!(error.contains("fields.forge-pr"));

        let text = valid_policy().replacen("schema_version: 1", "schema_version: 2", 1)
            + "\nfields:\n  forge_pr:\n    type: object\n    required: []\n";
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();

        assert!(error.contains("workflow_config_invalid_field"));
        assert!(error.contains("object fields require at least one required key"));
    }

    #[test]
    fn parses_configured_branch_lifecycle_policy() {
        let policy = parse_policy_text(&configured_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(policy.branch_lifecycle.base_branch, "trunk");
        assert_eq!(
            policy.branch_lifecycle.merge_strategy,
            MergeStrategy::FastForwardOnly
        );
        assert_eq!(
            policy.branch_lifecycle.branch_templates.epic,
            "review/{{ issue.id }}"
        );
        assert_eq!(
            policy.branch_lifecycle.branch_templates.issue,
            "work/{{ issue.type }}/{{ issue.id }}"
        );
    }

    #[test]
    fn rejects_configured_branch_lifecycle_without_base_branch() {
        let text = valid_policy().replace(
            default_branch_lifecycle(),
            "branch_lifecycle:\n  merge_strategy: squash\n",
        );
        let error = parse_policy_text(&text, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(error.contains("workflow_config_invalid_branch_lifecycle"));
        assert!(error.contains("branch_lifecycle.base_branch"));
    }

    #[test]
    fn missing_branch_lifecycle_config_uses_default_policy() {
        let text = valid_policy().replace(default_branch_lifecycle(), "");
        let policy = parse_policy_text(&text, WORKFLOW_POLICY_PATH).unwrap();

        assert_eq!(policy.branch_lifecycle.base_branch, "main");
        assert_eq!(
            policy.branch_lifecycle.merge_strategy,
            MergeStrategy::Squash
        );
        assert_eq!(
            policy.branch_lifecycle.branch_templates.issue,
            "codex/{{ issue.id }}"
        );
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
    fn rejects_unknown_validator_reference() {
        let error = parse_policy_text(
            &valid_policy().replace(
                "          - proof_attached\n",
                "          - missing_validator\n",
            ),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_unknown_reference"));
        assert!(error.contains("missing_validator"));
    }

    #[test]
    fn rejects_missing_issue_type_mapping() {
        let error = parse_policy_text(
            &valid_policy().replace("  spike: lightweight_spike\n", ""),
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
    fn rejects_unknown_template_variable() {
        let error = parse_policy_text(
            &valid_policy().replace("{{ issue.id }}", "{{ issue.title }}"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("workflow_config_invalid_guidance_template"));
        assert!(error.contains("issue.title"));
    }
}
