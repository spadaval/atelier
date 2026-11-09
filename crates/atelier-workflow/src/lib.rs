use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_yaml::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use atelier_core::{Issue, IssueStrategy, ISSUE_STRATEGY_FIELD};

/// The workflow policy `atelier init` writes, naming `base_branch`, the
/// branch init detected, as the base.
pub fn starter_policy_yaml(base_branch: &str) -> String {
    // A branch name that YAML would misread is quoted.
    let base_branch = serde_yaml::to_string(base_branch).expect("a string serializes as YAML");
    format!(
        "schema: atelier.workflow\nschema_version: 3\nbranch_policy:\n  base_branch: {}\n{STARTER_POLICY_BODY}",
        base_branch.trim_end()
    )
}

/// The starter policy after its `branch_policy`.
const STARTER_POLICY_BODY: &str = r#"issue_types:
  bug:
    label: Bug
  decision:
    label: Decision
  feature:
    label: Feature
  loose_end:
    label: Loose end
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
  done:
    category: done
  superseded:
    category: done
  declined:
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
        from: [todo]
        to: in_progress
        description: Start work.
        validators:
        - blockers.transitive_none_open
      close:
        from: [in_progress]
        to: done
        required_fields: [close_reason]
        description: Close the mission once all its work is merged and its validation, if it has one, is done; the Report says what it achieved.
        validators:
        - blockers.none_open
      supersede:
        from: [draft, todo, in_progress]
        to: superseded
        required_fields: [close_reason]
        description: Retire work that evidence made obsolete.
  work:
    applies_to:
    - bug
    - feature
    - spike
    - task
    initial_status: todo
    done_statuses:
    - done
    - superseded
    - declined
    transitions:
      plan:
        from: [backlog]
        to: todo
        description: Pull someday work into the near-term plan.
      defer:
        from: [todo]
        to: backlog
        description: Park work that is recorded but not near-term.
      start:
        from: [todo]
        to: in_progress
        description: Start work.
        validators:
        - blockers.transitive_none_open
      supersede:
        from: [backlog, todo, in_progress]
        to: superseded
        required_fields: [close_reason]
        description: Retire work that is no longer needed. Work is done when atelier merge integrates it.
      decline:
        from: [backlog, todo, in_progress]
        to: declined
        required_fields: [close_reason]
        description: Decline real work that will not be done; the Report says why. It needs no branch, review or merge.
  decision:
    applies_to:
    - decision
    initial_status: todo
    done_statuses:
    - done
    - superseded
    transitions:
      decide:
        from: [todo]
        to: done
        required_fields: [close_reason]
        description: Record the decision. Decisions need no branch and are decided on the base branch as a planning change.
        validators:
        - decision.recorded
        - blockers.none_open
        - work.terminal
      supersede:
        from: [todo]
        to: superseded
        required_fields: [close_reason]
        description: Retire a question that no longer needs deciding.
  on_base:
    applies_to:
    - loose_end
    - validation
    initial_status: todo
    done_statuses:
    - done
    - superseded
    - declined
    transitions:
      plan:
        from: [backlog]
        to: todo
        description: Bring a parked item back into the near-term plan.
      defer:
        from: [todo]
        to: backlog
        description: Park an item that is recorded but not near-term, such as a validation for later.
      done:
        from: [todo]
        to: done
        required_fields: [close_reason]
        description: Record the one-line result in the Report once what it waits on is merged. It needs no branch, review or merge, and is done on the base branch as a planning change; a bad result files a linked bug.
        validators:
        - blockers.none_open
      supersede:
        from: [backlog, todo]
        to: superseded
        required_fields: [close_reason]
        description: Retire it once it is no longer needed.
      decline:
        from: [backlog, todo]
        to: declined
        required_fields: [close_reason]
        description: Decline an item that will not be done; the Report says why.
"#;

pub const WORKFLOW_POLICY_PATH: &str = ".atelier/workflow.yaml";
const WORKFLOW_SCHEMA: &str = "atelier.workflow";
const WORKFLOW_SCHEMA_VERSION: i64 = 3;
const STATUS_CATEGORIES: &[&str] = &["todo", "active", "blocked", "done"];
/// A validator an older policy may name, read and ignored: no gate reads
/// evidence receipts (atelier-md0v).
const IGNORED_VALIDATORS: &[&str] = &["evidence.attached"];
const BUILTIN_VALIDATORS: &[&str] = &[
    "issue.sections_parseable",
    "blockers.none_open",
    "blockers.transitive_none_open",
    "lint.none_blocking",
    "git.worktree_clean",
    "work.terminal",
    "decision.recorded",
    "shell.check",
    CI_PASSED,
];
/// The merge check that reads CI through the policy's forge (ADR 0031).
pub const CI_PASSED: &str = "ci.passed";
/// The only forge provider Atelier knows, reached through the `gh` CLI.
const FORGE_PROVIDERS: &[&str] = &["github"];
/// The one required field: a non-empty Report section. The policy key keeps
/// its old name (atelier-jnrh).
const CLOSE_REASON_FIELD: &str = "close_reason";
const ALLOWED_REQUIRED_FIELDS: &[&str] = &[CLOSE_REASON_FIELD];
/// Report parts an older policy required one by one; they are read as
/// `close_reason`, since Atelier now requires only a non-empty Report
/// (atelier-jnrh).
const RETIRED_REPORT_FIELDS: &[&str] = &[
    "report.deviations",
    "report.decisions",
    "report.surprises",
    "report.open_items",
];
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
    "forge",
];
#[derive(Debug, Clone)]
pub struct WorkflowPolicy {
    pub merge_checks: Vec<ValidatorDefinition>,
    /// The forge that carries branches and reports CI, when the policy names
    /// one; without it Atelier makes no forge call (ADR 0031).
    pub forge: Option<ForgeConfig>,
    pub schema_version: i64,
    pub branch_policy: BranchLifecycleConfig,
    pub issue_types: BTreeMap<String, IssueTypeDefinition>,
    pub workflow_by_issue_type: BTreeMap<String, String>,
    pub statuses: BTreeMap<String, StatusDefinition>,
    pub workflows: BTreeMap<String, WorkflowDefinition>,
}

/// The policy's `forge` block (ADR 0031). `github` is the only provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeConfig {
    /// A Git remote name or a remote URL; `origin` when the policy leaves
    /// it out.
    pub remote: String,
    /// The file name, under `.github/workflows`, of the workflow whose pull
    /// request runs `ci.passed` trusts.
    pub ci_workflow: String,
}

/// `atelier merge` always integrates with a merge commit, which also marks
/// the work done (atelier-jnrh), so the policy names only the base. An older
/// policy's `merge_strategy` is read and ignored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchLifecycleConfig {
    pub base_branch: String,
}

impl Default for BranchLifecycleConfig {
    fn default() -> Self {
        Self {
            base_branch: "main".to_string(),
        }
    }
}

/// The workflow whose items reach done when `atelier merge` integrates their
/// branch, rather than by a transition (atelier-jnrh).
pub const WORK_WORKFLOW: &str = "work";

/// The transition `atelier start` applies. A workflow without one gives its
/// items no branch: they are done on the base by a transition (atelier-8bh4).
pub const START_TRANSITION: &str = "start";

/// Work that changes no code and can only happen after a merge or a
/// release, such as pushing a tag or a play-test: it waits on what it
/// follows through `blocked_by` and is never part of a mission (ADR 0028,
/// decision 3).
pub const LOOSE_END_TYPE: &str = "loose_end";

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
    ShellCheck { command: String },
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

    /// The status `atelier merge` sets on an open item of this type when its
    /// branch merges: the first done status of the `work` workflow. `None`
    /// for types that reach done by a transition, such as missions and
    /// decisions (atelier-jnrh).
    pub fn merge_done_status(&self, issue_type: &str) -> Option<&str> {
        if self.workflow_name_for_issue_type(issue_type).ok()? != WORK_WORKFLOW {
            return None;
        }
        self.workflow_for_issue_type(issue_type)
            .ok()?
            .done_statuses
            .first()
            .map(String::as_str)
    }

    /// Whether items of this type are done on the base by a transition, with
    /// no branch, review or merge: their workflow has no `start` transition,
    /// as for decisions, loose ends and validations (atelier-8bh4). Judged by
    /// workflow, so a policy that keeps `validation` in `work` starts it as
    /// before. False for a type the policy does not map.
    pub fn done_on_base(&self, issue_type: &str) -> bool {
        self.workflow_for_issue_type(issue_type)
            .is_ok_and(|workflow| !workflow.transitions.contains_key(START_TRANSITION))
    }

    /// The transition that takes an item of this type from `status` to its
    /// workflow's first done status, such as a loose end's `done` or a
    /// decision's `decide`.
    pub fn done_transition(&self, issue_type: &str, status: &str) -> Option<&str> {
        let workflow = self.workflow_for_issue_type(issue_type).ok()?;
        let done = workflow.done_statuses.first()?;
        workflow
            .transitions
            .iter()
            .find(|(_, transition)| {
                transition.to == *done && transition.from.iter().any(|from| from == status)
            })
            .map(|(name, _)| name.as_str())
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

    /// The branch `atelier start` creates on the base for a branch owner (a
    /// parent or a standalone issue): its bare ID. Branch names carry no
    /// issue type (atelier-md0v).
    pub fn branch_name_for_owner(&self, owner: &Issue) -> Result<String> {
        let branch = owner.id.clone();
        validate_branch_value(
            &branch,
            WORKFLOW_POLICY_PATH,
            "branch_policy",
            "work branch name",
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

/// Whether `error` is a policy a newer Atelier wrote, whose one fix is the
/// reinstall its message names, not an edit (atelier-u4ql).
pub fn needs_newer_atelier(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause
            .downcast_ref::<WorkflowPolicyError>()
            .is_some_and(|policy| policy.code == "workflow_config_schema_newer")
    })
}

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
    let branch_policy = parse_branch_policy(root.get("branch_policy"), display_path)?;
    let forge = root
        .get("forge")
        .map(|value| parse_forge(value, display_path))
        .transpose()?;
    if forge.is_none() && merge_checks.iter().any(|check| check.builtin == CI_PASSED) {
        return Err(policy_error_with_field(
            "workflow_config_invalid_validator",
            display_path,
            "merge_checks",
            "ci.passed reads CI through a forge, and this policy has no forge block; add one naming forge.provider and forge.ci_workflow",
        ));
    }
    let policy = WorkflowPolicy {
        merge_checks,
        forge,
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

/// The `forge` block, refused naming the key that is wrong.
fn parse_forge(value: &Value, display_path: &str) -> Result<ForgeConfig> {
    let invalid = |field: &str, message: String| {
        policy_error_with_field(
            "workflow_config_invalid_forge",
            display_path,
            field,
            message,
        )
    };
    let Some(mapping) = value.as_mapping() else {
        return Err(invalid(
            "forge",
            "forge must be a mapping with provider, ci_workflow and an optional remote".into(),
        ));
    };
    let text = |key: &str| -> Result<Option<String>> {
        match mapping.get(key) {
            None => Ok(None),
            Some(Value::String(text)) => Ok(Some(text.trim().to_owned())),
            Some(_) => Err(invalid(
                &format!("forge.{key}"),
                format!("forge.{key} must be a string"),
            )),
        }
    };
    for key in mapping.keys() {
        match key.as_str() {
            Some("provider" | "remote" | "ci_workflow") => {}
            Some(other) => {
                return Err(invalid(
                    &format!("forge.{other}"),
                    format!(
                        "unknown forge field '{other}'; expected provider, remote or ci_workflow"
                    ),
                ))
            }
            None => return Err(invalid("forge", "forge keys must be strings".into())),
        }
    }
    match text("provider")? {
        None => {
            return Err(invalid(
                "forge.provider",
                "forge.provider is required; write github".into(),
            ))
        }
        Some(provider) if !FORGE_PROVIDERS.contains(&provider.as_str()) => {
            return Err(invalid(
                "forge.provider",
                format!("unsupported forge provider '{provider}'; expected github"),
            ))
        }
        Some(_) => {}
    }
    let remote = match text("remote")? {
        None => "origin".to_owned(),
        Some(remote) if remote.is_empty() => {
            return Err(invalid(
                "forge.remote",
                "forge.remote cannot be empty; name a Git remote or a remote URL, or leave it out for origin".into(),
            ))
        }
        Some(remote) => remote,
    };
    let ci_workflow = match text("ci_workflow")? {
        Some(workflow) if !workflow.is_empty() => workflow,
        _ => {
            return Err(invalid(
                "forge.ci_workflow",
                "forge.ci_workflow is required: the file name under .github/workflows of the workflow ci.passed trusts, such as ci.yml".into(),
            ))
        }
    };
    if ci_workflow.contains('/') {
        return Err(invalid(
            "forge.ci_workflow",
            format!("forge.ci_workflow '{ci_workflow}' must be a file name under .github/workflows, such as ci.yml, not a path"),
        ));
    }
    Ok(ForgeConfig {
        remote,
        ci_workflow,
    })
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
    if let Some(name) = raw.merge_strategy.as_deref() {
        validate_merge_strategy(name, display_path)?;
    }
    Ok(BranchLifecycleConfig { base_branch })
}

/// Every strategy an Atelier policy has named is read as `merge_commit`, the
/// only way `atelier merge` integrates (atelier-jnrh); any other name is a
/// mistake worth reporting.
fn validate_merge_strategy(value: &str, display_path: &str) -> Result<()> {
    match value {
        "merge_commit" | "fast_forward_only" | "squash" => Ok(()),
        other => Err(policy_error_with_field(
            "workflow_config_invalid_branch_policy",
            display_path,
            "branch_policy.merge_strategy",
            format!(
                "unsupported merge strategy '{}'; atelier merge always creates a merge commit, so write merge_commit or leave the field out",
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
    // A newer Atelier wrote it: the fix is a reinstall (atelier-u4ql).
    if schema_version > WORKFLOW_SCHEMA_VERSION {
        return Err(policy_error_with_field(
            "workflow_config_schema_newer",
            display_path,
            "schema_version",
            format!(
                "workflow schema_version {schema_version} is newer than the {WORKFLOW_SCHEMA_VERSION} this atelier reads, so the policy needs a newer atelier. Next: {}",
                atelier_core::reinstall_fix()
            ),
        ));
    }
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
    let mut required_fields = Vec::<String>::new();
    for field in &raw.required_fields {
        let field = if RETIRED_REPORT_FIELDS.contains(&field.as_str()) {
            CLOSE_REASON_FIELD
        } else {
            field.as_str()
        };
        if !required_fields.iter().any(|seen| seen == field) {
            required_fields.push(field.to_string());
        }
    }
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
        required_fields,
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
        // An ignored validator parses with any parameters and gates nothing.
        let ignored = match value {
            Value::String(name) => Some(name.as_str()),
            Value::Mapping(mapping) if mapping.len() == 1 => {
                mapping.keys().next().and_then(Value::as_str)
            }
            _ => None,
        }
        .is_some_and(|name| IGNORED_VALIDATORS.contains(&name));
        if ignored {
            continue;
        }
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
    if name == CI_PASSED && !matches!(context, ValidatorContext::MergeChecks) {
        return Err(policy_error_with_field(
            "workflow_config_invalid_validator",
            display_path,
            format!("workflows.{workflow_name}.transitions.{transition_name}.validators"),
            "ci.passed is available only in merge_checks",
        ));
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
        if !ALLOWED_REQUIRED_FIELDS.contains(&required_field.as_str())
            && !RETIRED_REPORT_FIELDS.contains(&required_field.as_str())
        {
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
    let requires_reason = transition
        .required_fields
        .iter()
        .any(|field| field == CLOSE_REASON_FIELD);
    if requires_reason && policy.status_category(&transition.to) != Some("done") {
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

/// Checks one issue against the policy. Errors about the issue itself name
/// `report_path`: the issue's record file when validating stored records.
/// Errors about a missing workflow mapping name the workflow file.
pub fn validate_issue_against_policy(
    policy: &WorkflowPolicy,
    issue: &Issue,
    report_path: &Path,
) -> Result<()> {
    if !policy.issue_types.contains_key(&issue.issue_type) {
        return Err(WorkflowPolicyError {
            code: "workflow_issue_type_unknown",
            path: report_path.display().to_string(),
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
            path: report_path.display().to_string(),
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
    validate_issue_fields_against_policy(policy, issue, report_path)?;
    Ok(())
}

fn validate_issue_fields_against_policy(
    _policy: &WorkflowPolicy,
    issue: &Issue,
    report_path: &Path,
) -> Result<()> {
    if let Some(strategy) = &issue.strategy {
        if issue.issue_type != "mission" {
            return invalid_strategy_field(
                issue,
                report_path,
                "strategy is only valid on issue_type 'mission'",
            );
        }
        IssueStrategy::new(&strategy.path)?;
    }
    if let Some(branch) = &issue.workflow_branch {
        for (key, value) in [
            ("work_branch", Some(&branch.work_branch)),
            ("branch_base", branch.branch_base.as_ref()),
        ] {
            let Some(value) = value else { continue };
            validate_branch_value(
                value,
                &report_path.display().to_string(),
                &format!("workflow_branch.{key}"),
                "workflow branch",
            )?;
        }
    }
    Ok(())
}

fn invalid_strategy_field(issue: &Issue, report_path: &Path, reason: &str) -> Result<()> {
    Err(WorkflowPolicyError {
        code: "workflow_issue_field_invalid",
        path: report_path.display().to_string(),
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

    fn valid_policy() -> String {
        starter_policy_yaml("main")
    }

    /// A higher schema_version says to reinstall; a lower one stays
    /// unsupported (atelier-u4ql).
    #[test]
    fn a_newer_policy_needs_a_newer_atelier() {
        let newer = parse_policy_text(
            &valid_policy().replace("schema_version: 3", "schema_version: 4"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert_eq!(
            newer,
            format!("error: workflow_config_schema_newer\npath: {WORKFLOW_POLICY_PATH}\nfield: schema_version\nmessage: workflow schema_version 4 is newer than the 3 this atelier reads, so the policy needs a newer atelier. Next: upgrade to the latest release with `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spadaval/atelier/releases/latest/download/atelier-cli-installer.sh | sh`, or from Atelier's source checkout run `cargo install --path crates/atelier-cli --locked`")
        );
        let older = parse_policy_text(
            &valid_policy().replace("schema_version: 3", "schema_version: 2"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap_err()
        .to_string();
        assert!(
            older.starts_with("error: workflow_config_schema_unsupported\n"),
            "{older}"
        );
    }

    /// A forge block defaults its remote to origin, and a policy without
    /// one has no forge (ADR 0031).
    #[test]
    fn a_forge_block_parses_and_ci_passed_needs_it_in_merge_checks() {
        let plain = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(plain.forge, None);
        let forge = format!(
            "{}\nforge:\n  provider: github\n  ci_workflow: ci.yml\nmerge_checks:\n  - ci.passed\n",
            valid_policy()
        );
        let policy = parse_policy_text(&forge, WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(
            policy.forge,
            Some(ForgeConfig {
                remote: "origin".into(),
                ci_workflow: "ci.yml".into()
            })
        );
        assert_eq!(policy.merge_checks[0].builtin, CI_PASSED);
        let remote = forge.replace(
            "  ci_workflow: ci.yml\n",
            "  ci_workflow: ci.yml\n  remote: git@github.com:o/r.git\n",
        );
        assert_eq!(
            parse_policy_text(&remote, WORKFLOW_POLICY_PATH)
                .unwrap()
                .forge
                .unwrap()
                .remote,
            "git@github.com:o/r.git"
        );
        let path = forge.replace(
            "ci_workflow: ci.yml",
            "ci_workflow: .github/workflows/ci.yml",
        );
        let error = parse_policy_text(&path, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("workflow_config_invalid_forge")
                && error.contains("field: forge.ci_workflow"),
            "{error}"
        );
        // Like shell.check, ci.passed is a merge check only.
        let transition = forge.replace(
            "        - blockers.transitive_none_open\n",
            "        - blockers.transitive_none_open\n        - ci.passed\n",
        );
        assert_ne!(transition, forge);
        let error = parse_policy_text(&transition, WORKFLOW_POLICY_PATH)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("workflow_config_invalid_validator")
                && error.contains("ci.passed is available only in merge_checks"),
            "{error}"
        );
    }

    fn default_branch_policy() -> &'static str {
        "branch_policy:\n  base_branch: main\n"
    }

    fn configured_policy() -> String {
        valid_policy().replace(
            default_branch_policy(),
            "branch_policy:\n  base_branch: trunk\n  merge_strategy: fast_forward_only\n",
        )
    }

    fn decide_validators() -> &'static str {
        "        validators:\n        - decision.recorded\n        - blockers.none_open\n        - work.terminal\n      supersede:\n        from: [todo]\n"
    }

    /// The starter policy with the decision workflow's `decide` running
    /// `validators` instead of its own.
    fn with_decide_validators(validators: &str) -> String {
        let policy = valid_policy().replace(
            decide_validators(),
            &format!("        validators:\n{validators}      supersede:\n        from: [todo]\n"),
        );
        assert_ne!(policy, valid_policy());
        policy
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
            adr: None,
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
        let policy = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(policy.schema_version, 3);
        for (issue_type, workflow) in [
            ("mission", "objective"),
            ("feature", "work"),
            ("task", "work"),
            ("loose_end", "on_base"),
            ("validation", "on_base"),
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
        assert_eq!(policy.status_category("review"), None);
        // Merge marks work done, so the work workflow has no close
        // (atelier-jnrh); supersede still requires a Report.
        let work = &policy.workflows["work"].transitions;
        assert!(!work.contains_key("close"));
        assert_eq!(
            work["supersede"].required_fields,
            vec!["close_reason".to_string()]
        );
        // Declined is terminal work that will not be done (atelier-stnv):
        // a done-category status reached from any open work status, with a
        // Report, and never what merge marks.
        assert_eq!(policy.status_category("declined"), Some("done"));
        assert_eq!(
            (work["decline"].from.clone(), work["decline"].to.as_str()),
            (
                vec!["backlog".into(), "todo".into(), "in_progress".into()],
                "declined"
            )
        );
        assert_eq!(
            work["decline"].required_fields,
            vec!["close_reason".to_string()]
        );
        assert_eq!(policy.merge_done_status("task"), Some("done"));
        assert_eq!(policy.merge_done_status("mission"), None);
        assert_eq!(policy.merge_done_status("decision"), None);
        assert_eq!(policy.merge_done_status("loose_end"), None);
        // Loose ends and validations share one workflow with no start: done
        // on the base through `done`, which requires a Report (atelier-8bh4),
        // and parked in `backlog` as work is (atelier-nn67).
        let on_base = &policy.workflows["on_base"].transitions;
        let mut names = on_base.keys().map(String::as_str).collect::<Vec<_>>();
        names.sort();
        assert_eq!(names, vec!["decline", "defer", "done", "plan", "supersede"]);
        for (name, from, to) in [
            ("plan", vec!["backlog"], "todo"),
            ("defer", vec!["todo"], "backlog"),
            ("supersede", vec!["backlog", "todo"], "superseded"),
            ("decline", vec!["backlog", "todo"], "declined"),
        ] {
            assert_eq!(
                (on_base[name].from.clone(), on_base[name].to.as_str()),
                (from.into_iter().map(String::from).collect(), to),
                "{name}"
            );
        }
        assert_eq!(on_base["done"].from, vec!["todo".to_string()]);
        assert_eq!(on_base["done"].to, "done");
        assert_eq!(
            on_base["done"].required_fields,
            vec!["close_reason".to_string()]
        );
        for (issue_type, on_base) in [
            ("loose_end", true),
            ("validation", true),
            ("decision", true),
            ("task", false),
            ("mission", false),
            ("unknown", false),
        ] {
            assert_eq!(policy.done_on_base(issue_type), on_base, "{issue_type}");
        }
        assert_eq!(policy.done_transition("loose_end", "todo"), Some("done"));
        assert_eq!(policy.done_transition("decision", "todo"), Some("decide"));
        assert_eq!(policy.done_transition("loose_end", "done"), None);
        assert_eq!(policy.done_transition("loose_end", "backlog"), None);
        let close = &policy.workflows["objective"].transitions["close"];
        assert_eq!(close.required_fields, vec!["close_reason".to_string()]);
        assert_eq!(
            validator_names(&close.validators),
            vec!["blockers.none_open"]
        );
        assert_eq!(
            validator_names(&policy.workflows["objective"].transitions["start"].validators),
            vec!["blockers.transitive_none_open"]
        );
        assert_eq!(policy.branch_policy.base_branch, "main");
    }

    /// The starter keeps the statuses and types repositories use
    /// (atelier-bqbz): `blocked` and `review` went, since blockers are
    /// `blocked_by` links and the done claim comes before review; every type
    /// stays.
    #[test]
    fn starter_policy_declares_the_statuses_and_types_in_use() {
        let policy = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(
            policy
                .statuses
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec![
                "backlog",
                "declined",
                "done",
                "draft",
                "in_progress",
                "superseded",
                "todo"
            ]
        );
        assert_eq!(
            policy
                .issue_types
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec![
                "bug",
                "decision",
                "feature",
                "loose_end",
                "mission",
                "spike",
                "task",
                "validation"
            ]
        );
        for workflow in policy.workflows.values() {
            for name in ["block", "request_review", "revise"] {
                assert!(!workflow.transitions.contains_key(name), "{name}");
            }
        }
    }

    /// Repository policies written from the earlier starter keep `blocked`,
    /// `review` and their transitions, and still load unchanged.
    #[test]
    fn the_earlier_starter_with_blocked_and_review_still_parses() {
        let policy = parse_policy_text(
            include_str!("../fixtures/starter-policy-with-blocked-and-review.yaml"),
            WORKFLOW_POLICY_PATH,
        )
        .unwrap();
        assert_eq!(policy.status_category("blocked"), Some("blocked"));
        assert_eq!(policy.status_category("review"), Some("active"));
        let work = &policy.workflows["work"].transitions;
        assert_eq!(work["block"].to, "blocked");
        assert_eq!(work["request_review"].to, "review");
        assert_eq!(work["revise"].to, "in_progress");
        assert!(work["close"].from.contains(&"review".to_string()));
        assert!(policy.workflows["decision"]
            .transitions
            .contains_key("block"));
        // A policy that keeps `validation` in `work` still starts it.
        assert!(!policy.done_on_base("validation"));
        assert!(policy.done_on_base("decision"));
    }

    #[test]
    fn starter_policy_has_objective_work_decision_and_on_base_workflows_only() {
        let policy = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        assert_eq!(
            policy
                .workflows
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["decision", "objective", "on_base", "work"]
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
        let policy = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();

        assert!(!policy.can_enter_active("mission", "draft").unwrap());
        assert!(policy.can_enter_active("mission", "todo").unwrap());
        assert!(!policy.can_enter_active("task", "backlog").unwrap());
        assert!(policy.can_enter_active("task", "todo").unwrap());
    }

    #[test]
    fn evidence_attached_still_parses_and_is_ignored() {
        // No gate reads receipts (atelier-md0v): an older policy naming the
        // validator, with or without parameters, parses and gates nothing.
        for named in [
            "        - evidence.attached: { min_count: 2, kind: test }\n",
            "        - evidence.attached: { min_count: 0 }\n",
            "        - evidence.attached\n",
        ] {
            let policy = with_decide_validators(&format!(
                "        - issue.sections_parseable\n        - blockers.none_open\n{named}        - work.terminal\n"
            ));
            let policy = parse_policy_text(&policy, WORKFLOW_POLICY_PATH).unwrap();
            let decide = &policy.workflows["decision"].transitions["decide"];
            assert_eq!(
                decide
                    .validators
                    .iter()
                    .map(|validator| validator.builtin.as_str())
                    .collect::<Vec<_>>(),
                [
                    "issue.sections_parseable",
                    "blockers.none_open",
                    "work.terminal"
                ],
                "{named}"
            );
        }
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
            let policy = with_decide_validators(&format!("        - {removed}\n"));
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
        let policy = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
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
        let policy = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut issue = test_issue();
        issue.workflow_branch = Some(atelier_core::WorkflowBranch {
            work_branch: "task/atelier-pr01".to_string(),
            branch_base: Some("main".to_string()),
        });
        validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH)).unwrap();

        // A record without `branch_base` is valid too (atelier-nn67).
        issue.workflow_branch.as_mut().unwrap().branch_base = None;
        validate_issue_against_policy(&policy, &issue, Path::new(WORKFLOW_POLICY_PATH)).unwrap();
    }

    #[test]
    fn typed_workflow_branch_rejects_obsolete_integration_field() {
        assert!(serde_json::from_value::<atelier_core::WorkflowBranch>(serde_json::json!({"work_branch": "task/test", "branch_base": "main", "integration_target": "main"})).is_err());
    }

    #[test]
    fn rejects_unknown_issue_type_in_record() {
        let policy = parse_policy_text(&valid_policy(), WORKFLOW_POLICY_PATH).unwrap();
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
    }

    /// `atelier merge` always creates a merge commit, so every strategy an
    /// older policy named, and none at all, still loads, with or without a
    /// shell check; an unknown name is still a mistake (atelier-jnrh).
    #[test]
    fn every_merge_strategy_is_read_as_a_merge_commit() {
        let with = |name: Option<&str>, checks: &str| {
            let mut text = valid_policy().to_string();
            if let Some(name) = name {
                text = text.replace(
                    default_branch_policy(),
                    &format!("{}  merge_strategy: {name}\n", default_branch_policy()),
                );
            }
            text.push_str(checks);
            parse_policy_text(&text, WORKFLOW_POLICY_PATH)
        };
        for checks in ["", "merge_checks:\n  - shell.check: { command: 'true' }\n"] {
            for name in [
                None,
                Some("merge_commit"),
                Some("fast_forward_only"),
                Some("squash"),
            ] {
                assert_eq!(
                    with(name, checks).unwrap().branch_policy,
                    BranchLifecycleConfig::default(),
                    "{name:?} {checks}"
                );
            }
        }
        let error = with(Some("rebase"), "").unwrap_err().to_string();
        assert!(error.contains("branch_policy.merge_strategy"), "{error}");
        assert!(
            error.contains("unsupported merge strategy 'rebase'"),
            "{error}"
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
    fn branch_name_for_owner_is_the_bare_id() {
        let policy = parse_policy_text(&configured_policy(), WORKFLOW_POLICY_PATH).unwrap();
        let mut issue = test_issue();
        issue.id = "atelier-abc1".to_string();

        assert_eq!(
            policy.branch_name_for_owner(&issue).unwrap(),
            "atelier-abc1"
        );
        issue.issue_type = "feature".to_string();
        assert_eq!(
            policy.branch_name_for_owner(&issue).unwrap(),
            "atelier-abc1"
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
            &with_decide_validators("        - missing_validator\n"),
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

    /// An older policy's report parts are read as `close_reason`: Atelier
    /// requires only a non-empty Report now (atelier-jnrh).
    #[test]
    fn retired_report_parts_are_read_as_close_reason() {
        let start = valid_policy().replace(
            "        description: Start work.",
            "        required_fields: [report.deviations]\n        description: Start work.",
        );
        let error = parse_policy_text(&start, WORKFLOW_POLICY_PATH).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("may only be required by a done-category transition"),
            "{error}"
        );
        for fields in [
            "[close_reason, report.deviations, report.decisions, report.surprises, report.open_items]",
            "[report.open_items]",
        ] {
            let report = valid_policy().replacen(
                "required_fields: [close_reason]",
                &format!("required_fields: {fields}"),
                1,
            );
            let policy = parse_policy_text(&report, WORKFLOW_POLICY_PATH).unwrap();
            assert_eq!(
                policy.workflows["objective"].transitions["close"].required_fields,
                vec!["close_reason".to_string()],
                "{fields}"
            );
        }
        let unknown = valid_policy().replacen(
            "required_fields: [close_reason]",
            "required_fields: [close_reason, report.lessons]",
            1,
        );
        let error = parse_policy_text(&unknown, WORKFLOW_POLICY_PATH).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("unsupported required field 'report.lessons'"),
            "{error}"
        );
    }
}
