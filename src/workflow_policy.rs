use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_yaml::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use crate::db::Database;
use crate::models::Issue;

pub const WORKFLOW_POLICY_PATH: &str = ".atelier/workflow.yaml";
const WORKFLOW_SCHEMA: &str = "atelier.workflow";
const WORKFLOW_SCHEMA_VERSION: i64 = 1;
const STATUS_CATEGORIES: &[&str] = &["todo", "active", "blocked", "review", "validation", "done"];
const BUILTIN_ISSUE_TYPES: &[&str] = &[
    "bug",
    "closeout",
    "epic",
    "feature",
    "spike",
    "task",
    "validation",
];
const BUILTIN_VALIDATORS: &[&str] = &[
    "durable_state_current",
    "evidence_attached",
    "review_complete",
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
    "issue_types",
    "statuses",
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

#[derive(Debug, Clone)]
pub struct WorkflowPolicy {
    pub issue_types: BTreeMap<String, String>,
    pub statuses: BTreeMap<String, StatusDefinition>,
    pub validators: BTreeMap<String, ValidatorDefinition>,
    pub guidance_templates: BTreeMap<String, GuidanceTemplate>,
    pub workflows: BTreeMap<String, WorkflowDefinition>,
}

#[derive(Debug, Clone)]
pub struct StatusDefinition {
    pub category: String,
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

#[derive(Debug, Clone)]
pub struct WorkflowCheckReport {
    pub issue_count: usize,
    pub policy: WorkflowPolicy,
}

pub const STARTER_POLICY_YAML: &str = r#"schema: atelier.workflow
schema_version: 1

issue_types:
  bug: standard_review_proof
  closeout: standard_review_proof
  epic: standard_review_proof
  feature: standard_review_proof
  spike: lightweight_spike
  task: standard_review_proof
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

pub fn check(db: &Database, repo_root: &Path) -> Result<WorkflowCheckReport> {
    let policy_path = repo_root.join(WORKFLOW_POLICY_PATH);
    let policy = load(repo_root)?;
    let issues = db.list_issues(Some("all"), None, None)?;
    for issue in &issues {
        validate_issue_against_policy(&policy, issue, &policy_path)?;
    }
    Ok(WorkflowCheckReport {
        issue_count: issues.len(),
        policy,
    })
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

    check_top_level_fields(root, display_path)?;
    validate_schema(root, display_path)?;

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
        issue_types,
        statuses,
        validators,
        guidance_templates,
        workflows,
    };
    validate_policy(&policy, display_path)?;
    Ok(policy)
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

fn check_top_level_fields(root: &Mapping, display_path: &str) -> Result<()> {
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

fn validate_schema(root: &Mapping, display_path: &str) -> Result<()> {
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
    Ok(())
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
        let Some(name) = key.as_str() else {
            return Err(policy_error(
                "workflow_config_invalid_workflow",
                display_path,
                "workflow names must be strings",
            ));
        };
        ensure_identifier(
            name,
            display_path,
            &format!("workflows.{}", name),
            "workflow_config_invalid_workflow",
            "workflow names",
        )?;
        let raw = deserialize_entry::<WorkflowDefinitionRaw>(
            value,
            display_path,
            &format!("workflows.{}", name),
            "workflow_config_invalid_workflow",
        )?;
        if raw.done_statuses.is_empty() {
            return Err(policy_error_with_field(
                "workflow_config_invalid_workflow",
                display_path,
                format!("workflows.{}.done_statuses", name),
                "workflow done_statuses must contain at least one terminal status",
            ));
        }
        if raw.transitions.is_empty() {
            return Err(policy_error_with_field(
                "workflow_config_invalid_workflow",
                display_path,
                format!("workflows.{}.transitions", name),
                "workflow transitions must contain at least one transition",
            ));
        }

        let mut transitions = BTreeMap::new();
        for (transition_key, transition_value) in &raw.transitions {
            let Some(transition_name) = transition_key.as_str() else {
                return Err(policy_error_with_field(
                    "workflow_config_invalid_transition",
                    display_path,
                    format!("workflows.{}.transitions", name),
                    "transition names must be strings",
                ));
            };
            ensure_identifier(
                transition_name,
                display_path,
                &format!("workflows.{}.transitions.{}", name, transition_name),
                "workflow_config_invalid_transition",
                "transition names",
            )?;
            let transition_raw = deserialize_entry::<TransitionDefinitionRaw>(
                transition_value,
                display_path,
                &format!("workflows.{}.transitions.{}", name, transition_name),
                "workflow_config_invalid_transition",
            )?;
            let from = match transition_raw.from {
                StatusSelectorRaw::One(status) => vec![status],
                StatusSelectorRaw::Many(statuses) if !statuses.is_empty() => statuses,
                StatusSelectorRaw::Many(_) => {
                    return Err(policy_error_with_field(
                        "workflow_config_invalid_transition",
                        display_path,
                        format!("workflows.{}.transitions.{}.from", name, transition_name),
                        "transition from must be a status name or a non-empty list of status names",
                    ));
                }
            };
            for required_field in &transition_raw.required_fields {
                if !ALLOWED_REQUIRED_FIELDS.contains(&required_field.as_str()) {
                    return Err(policy_error_with_field(
                        "workflow_config_invalid_transition",
                        display_path,
                        format!(
                            "workflows.{}.transitions.{}.required_fields",
                            name, transition_name
                        ),
                        format!(
                            "unsupported required field '{}'; expected {}",
                            required_field,
                            ALLOWED_REQUIRED_FIELDS.join(", ")
                        ),
                    ));
                }
            }
            transitions.insert(
                transition_name.to_string(),
                TransitionDefinition {
                    from,
                    to: transition_raw.to,
                    required_fields: transition_raw.required_fields,
                    validators: transition_raw.validators,
                    guidance: transition_raw.guidance,
                },
            );
        }

        workflows.insert(
            name.to_string(),
            WorkflowDefinition {
                initial_status: raw.initial_status,
                done_statuses: raw.done_statuses,
                transitions,
            },
        );
    }
    Ok(workflows)
}

fn validate_policy(policy: &WorkflowPolicy, display_path: &str) -> Result<()> {
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

    for (workflow_name, workflow) in &policy.workflows {
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
        }
        for (transition_name, transition) in &workflow.transitions {
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
        }
    }
    Ok(())
}

fn validate_issue_against_policy(
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
    Ok(())
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

    #[test]
    fn parses_valid_policy() {
        let policy = parse_policy_text(valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(
            policy.issue_types.get("task").map(String::as_str),
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
