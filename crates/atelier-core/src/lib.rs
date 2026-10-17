//! Core Atelier domain vocabulary.
//!
//! This crate is intentionally free of filesystem and Clap
//! dependencies. Concrete domain types move here as the migration advances.

use chrono::{DateTime, NaiveDate, Utc};
use std::fmt;
use std::path::{Component, Path};

pub type IssueId = String;

/// The one form of an invalid-value error: the field, the rejected value and
/// every value the field accepts, as in `Invalid status 'open'. Valid values:
/// backlog, done, todo`.
pub fn invalid_value_message<S: AsRef<str>>(field: &str, value: &str, valid: &[S]) -> String {
    let valid = valid.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    format!(
        "Invalid {field} '{value}'. Valid values: {}",
        valid.join(", ")
    )
}

pub const ISSUE_PRIORITY_LABELS: &[&str] = &["critical", "high", "medium", "low"];

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum IssuePriority {
    P0,
    P1,
    P2,
    P3,
}

impl IssuePriority {
    pub fn canonical_token(self) -> &'static str {
        match self {
            Self::P0 => "P0",
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::P0 => "critical",
            Self::P1 => "high",
            Self::P2 => "medium",
            Self::P3 => "low",
        }
    }

    pub fn sort_rank(self) -> u8 {
        match self {
            Self::P0 => 0,
            Self::P1 => 1,
            Self::P2 => 2,
            Self::P3 => 3,
        }
    }

    pub fn from_canonical_token(value: &str) -> Result<Self, ParseIssuePriorityError> {
        match value {
            "P0" => Ok(Self::P0),
            "P1" => Ok(Self::P1),
            "P2" => Ok(Self::P2),
            "P3" => Ok(Self::P3),
            other => Err(ParseIssuePriorityError::new(other)),
        }
    }

    pub fn from_label(value: &str) -> Result<Self, ParseIssuePriorityError> {
        match value {
            "critical" => Ok(Self::P0),
            "high" => Ok(Self::P1),
            "medium" => Ok(Self::P2),
            "low" => Ok(Self::P3),
            other => Err(ParseIssuePriorityError::new(other)),
        }
    }

    pub fn from_cli_input(value: &str) -> Result<Self, ParseIssuePriorityError> {
        Self::from_label(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseIssuePriorityError {
    value: String,
}

impl ParseIssuePriorityError {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl fmt::Display for ParseIssuePriorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid priority '{}'. Valid values: critical, high, medium, low",
            self.value
        )
    }
}

impl std::error::Error for ParseIssuePriorityError {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Issue {
    pub id: IssueId,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub issue_type: String,
    pub priority: String,
    #[serde(default)]
    pub strategy: Option<IssueStrategy>,
    #[serde(default)]
    pub workflow_branch: Option<WorkflowBranch>,
    pub parent_id: Option<IssueId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

pub const ISSUE_STRATEGY_FIELD: &str = "strategy";

/// A repository-relative strategy link. Strategy revisions live in Git.
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueStrategy {
    pub path: String,
}

impl IssueStrategy {
    pub fn new(path: impl Into<String>) -> Result<Self, IssueFieldError> {
        let path = path.into();
        let parsed = Path::new(&path);
        if path.trim().is_empty()
            || parsed.is_absolute()
            || !parsed
                .components()
                .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
        {
            return Err(IssueFieldError::InvalidStrategy(
                "path must be a non-empty repository-relative path without parent traversal"
                    .to_string(),
            ));
        }
        Ok(Self { path })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowBranch {
    pub work_branch: String,
    pub branch_base: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IssueFieldError {
    InvalidStrategy(String),
}
impl fmt::Display for IssueFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategy(detail) => write!(f, "invalid issue strategy: {detail}"),
        }
    }
}
impl std::error::Error for IssueFieldError {}
impl Issue {
    pub fn strategy(&self) -> Result<Option<IssueStrategy>, IssueFieldError> {
        self.strategy
            .as_ref()
            .map(|strategy| IssueStrategy::new(&strategy.path))
            .transpose()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Relation {
    pub issue_id_1: IssueId,
    pub issue_id_2: IssueId,
    pub relation_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct RelationshipTarget {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct AttachmentRelationship {
    pub kind: String,
    pub id: String,
    pub role: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct RelatesRelationship {
    pub kind: String,
    pub id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Relationships {
    pub blocks: Vec<RelationshipTarget>,
    pub children: Vec<RelationshipTarget>,
    pub attachments: Vec<AttachmentRelationship>,
    pub relates: Vec<RelatesRelationship>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecordHeader {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub status: String,
    pub labels: Vec<String>,
    pub relationships: Relationships,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EvidenceRecord {
    pub header: RecordHeader,
    pub data: EvidenceRecordData,
    pub summary: String,
}

/// Immutable review events. Decisions change by appending a successor.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReviewRecord {
    pub header: RecordHeader,
    pub target: String,
    pub identity: String,
    pub commit: String,
    pub event: ReviewEvent,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingDecision {
    FixNow,
    Defer,
    NoAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "event", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReviewEvent {
    Review,
    Approval,
    Finding {
        recommendation: FindingDecision,
    },
    Disposition {
        finding: String,
        decision: FindingDecision,
        supersedes: Option<String>,
        resolution_commit: Option<String>,
        follow_up: Option<String>,
    },
    TrivialExemption,
}

/// Acceptance content is typed by work kind; lifecycle notes are separate.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueSections {
    pub body: IssueBody,
    pub notes: Option<String>,
    pub close_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IssueBody {
    Issue {
        goal: String,
        done_when: String,
        scope: Option<String>,
        context: Option<String>,
        unknowns: Option<String>,
        design: Option<String>,
        background: Option<String>,
    },
    Mission {
        outcome: String,
        success_criteria: Option<String>,
        background: Option<String>,
    },
    /// A design question for the human (atelier-ox63): the Decision is
    /// required to close it.
    Decision {
        question: String,
        options: Option<String>,
        decision: Option<String>,
        context: Option<String>,
        background: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IssueSectionName {
    Goal,
    DoneWhen,
    Scope,
    Context,
    Unknowns,
    Design,
    Outcome,
    SuccessCriteria,
    /// Free-form research, references, and motivation shared by a mission or
    /// a parent's children (atelier-by8u).
    Background,
    Question,
    Options,
    Decision,
    Notes,
    CloseReason,
}

/// Note kinds accepted by `IssueSections::append_note`.
pub const ISSUE_NOTE_KINDS: &[&str] = &[
    "note",
    "plan",
    "observation",
    "blocker",
    "resolution",
    "result",
    "handoff",
    "human",
];

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssueSectionState {
    pub name: IssueSectionName,
    pub required: bool,
    pub present: bool,
    pub empty: bool,
}

impl IssueSections {
    pub const ALL_NAMES: [IssueSectionName; 14] = [
        IssueSectionName::Goal,
        IssueSectionName::DoneWhen,
        IssueSectionName::Scope,
        IssueSectionName::Context,
        IssueSectionName::Unknowns,
        IssueSectionName::Design,
        IssueSectionName::Outcome,
        IssueSectionName::SuccessCriteria,
        IssueSectionName::Question,
        IssueSectionName::Options,
        IssueSectionName::Decision,
        IssueSectionName::Background,
        IssueSectionName::Notes,
        IssueSectionName::CloseReason,
    ];

    pub fn for_type(issue_type: &str, text: Option<&str>) -> Self {
        let goal = text
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(if issue_type == "decision" {
                "Question was not specified."
            } else {
                "Goal was not specified."
            })
            .to_string();
        let proof =
            (issue_type == "mission").then(|| "Success criteria were not specified.".to_string());
        let body = IssueBody::for_type(issue_type, goal, proof, None);
        Self {
            body,
            notes: None,
            close_reason: None,
        }
    }

    /// Preserve acceptance text when an explicit type update changes its shape.
    pub fn change_type(&mut self, issue_type: &str) {
        if self.matches_type(issue_type) {
            return;
        }
        let to_decision = issue_type == "decision";
        let (goal, proof, scope) = match &self.body {
            // Done when is acceptance, not a decision: a retyped task starts
            // with an empty Decision so `decide` still needs one.
            IssueBody::Issue {
                goal,
                done_when,
                scope,
                ..
            } => (
                goal.clone(),
                (!to_decision).then(|| done_when.clone()),
                scope.clone(),
            ),
            IssueBody::Mission {
                outcome,
                success_criteria,
                ..
            } => (outcome.clone(), success_criteria.clone(), None),
            IssueBody::Decision {
                question, decision, ..
            } => (question.clone(), decision.clone(), None),
        };
        let background = self
            .section(IssueSectionName::Background)
            .map(str::to_string);
        // Context is shared by work items and decisions; a decision's Options
        // stay readable as part of it.
        let context = match (
            self.section(IssueSectionName::Context),
            self.section(IssueSectionName::Options),
        ) {
            (Some(context), Some(options)) => Some(format!("{context}\n\nOptions:\n\n{options}")),
            (None, Some(options)) => Some(format!("Options:\n\n{options}")),
            (context, None) => context.map(str::to_string),
        };
        self.body = IssueBody::for_type(issue_type, goal, proof, scope);
        match &mut self.body {
            IssueBody::Issue { context: slot, .. } | IssueBody::Decision { context: slot, .. } => {
                *slot = context;
            }
            IssueBody::Mission { .. } => {}
        }
        // Background is the one optional section both shapes share.
        match &mut self.body {
            IssueBody::Issue {
                background: slot, ..
            }
            | IssueBody::Mission {
                background: slot, ..
            }
            | IssueBody::Decision {
                background: slot, ..
            } => {
                *slot = background;
            }
        }
    }

    pub fn matches_type(&self, issue_type: &str) -> bool {
        match issue_type {
            "mission" => matches!(self.body, IssueBody::Mission { .. }),
            "decision" => matches!(self.body, IssueBody::Decision { .. }),
            _ => matches!(self.body, IssueBody::Issue { .. }),
        }
    }

    pub fn required_names(&self) -> Vec<IssueSectionName> {
        match self.body {
            IssueBody::Issue { .. } => vec![IssueSectionName::Goal, IssueSectionName::DoneWhen],
            IssueBody::Mission { .. } => vec![IssueSectionName::Outcome],
            IssueBody::Decision { .. } => vec![IssueSectionName::Question],
        }
    }

    /// The two sections that say what the item is for and what finishes it;
    /// brief views show these and leave the rest to the full record.
    pub fn lead_names(&self) -> [IssueSectionName; 2] {
        match self.body {
            IssueBody::Issue { .. } => [IssueSectionName::Goal, IssueSectionName::DoneWhen],
            IssueBody::Mission { .. } => {
                [IssueSectionName::Outcome, IssueSectionName::SuccessCriteria]
            }
            IssueBody::Decision { .. } => [IssueSectionName::Question, IssueSectionName::Decision],
        }
    }

    /// Append one dated `- YYYY-MM-DD [kind] text` bullet to the Notes
    /// section, creating the section when absent. Continuation lines are
    /// indented so note text cannot open a new record section.
    pub fn append_note(&mut self, date: NaiveDate, kind: &str, text: &str) {
        let mut lines = text.trim().lines();
        let mut bullet = format!(
            "- {} [{kind}] {}",
            date.format("%Y-%m-%d"),
            lines.next().unwrap_or_default().trim_end()
        );
        for line in lines {
            bullet.push('\n');
            if !line.trim().is_empty() {
                bullet.push_str("  ");
                bullet.push_str(line.trim_end());
            }
        }
        self.notes = Some(match self.notes.as_deref().map(str::trim) {
            Some(existing) if !existing.is_empty() => format!("{existing}\n{bullet}"),
            _ => bullet,
        });
    }

    pub fn section(&self, name: IssueSectionName) -> Option<&str> {
        use IssueSectionName::*;
        match (&self.body, name) {
            (IssueBody::Issue { goal, .. }, Goal) => Some(goal),
            (IssueBody::Issue { done_when, .. }, DoneWhen) => Some(done_when),
            (IssueBody::Issue { scope, .. }, Scope) => scope.as_deref(),
            (IssueBody::Issue { context, .. } | IssueBody::Decision { context, .. }, Context) => {
                context.as_deref()
            }
            (IssueBody::Decision { question, .. }, Question) => Some(question),
            (IssueBody::Decision { options, .. }, Options) => options.as_deref(),
            (IssueBody::Decision { decision, .. }, Decision) => decision.as_deref(),
            (IssueBody::Issue { unknowns, .. }, Unknowns) => unknowns.as_deref(),
            (IssueBody::Issue { design, .. }, Design) => design.as_deref(),
            (IssueBody::Mission { outcome, .. }, Outcome) => Some(outcome),
            (
                IssueBody::Issue { background, .. }
                | IssueBody::Mission { background, .. }
                | IssueBody::Decision { background, .. },
                Background,
            ) => background.as_deref(),
            (
                IssueBody::Mission {
                    success_criteria, ..
                },
                SuccessCriteria,
            ) => success_criteria.as_deref(),
            (_, Notes) => self.notes.as_deref(),
            (_, CloseReason) => self.close_reason.as_deref(),
            _ => None,
        }
    }
    pub fn section_states(&self) -> Vec<IssueSectionState> {
        Self::ALL_NAMES
            .into_iter()
            .map(|name| {
                let value = self.section(name);
                IssueSectionState {
                    name,
                    required: self.required_names().contains(&name),
                    present: value.is_some(),
                    empty: value.map(str::trim).is_none_or(str::is_empty),
                }
            })
            .collect()
    }
}
impl IssueBody {
    fn for_type(
        issue_type: &str,
        goal: String,
        proof: Option<String>,
        scope: Option<String>,
    ) -> Self {
        match issue_type {
            "mission" => IssueBody::Mission {
                outcome: goal,
                success_criteria: proof,
                background: None,
            },
            "decision" => IssueBody::Decision {
                question: goal,
                options: None,
                decision: proof,
                context: None,
                background: None,
            },
            _ => IssueBody::Issue {
                goal,
                done_when: proof.unwrap_or_else(|| "Done when was not specified.".to_string()),
                scope,
                context: None,
                unknowns: None,
                design: None,
                background: None,
            },
        }
    }
}

impl IssueSectionName {
    pub fn title(self) -> &'static str {
        match self {
            Self::Goal => "Goal",
            Self::DoneWhen => "Done when",
            Self::Scope => "Scope",
            Self::Context => "Context",
            Self::Outcome => "Outcome",
            Self::Unknowns => "Unknowns",
            Self::Design => "Design",
            Self::SuccessCriteria => "Success criteria",
            Self::Question => "Question",
            Self::Options => "Options",
            Self::Decision => "Decision",
            Self::Background => "Background",
            Self::Notes => "Notes",
            Self::CloseReason => "Close Reason",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceResult {
    Pass,
    Fail,
    Blocked,
    NotApplicable,
}
impl EvidenceResult {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not-applicable",
        }
    }
}
impl std::str::FromStr for EvidenceResult {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pass" => Ok(Self::Pass),
            "fail" => Ok(Self::Fail),
            "blocked" => Ok(Self::Blocked),
            "not-applicable" => Ok(Self::NotApplicable),
            _ => Err("result must be pass, fail, blocked, or not-applicable".to_string()),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceStreamSummary {
    pub summary: String,
    pub bytes: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceOutputSummary {
    pub limit_bytes_per_stream: usize,
    pub stdout: EvidenceStreamSummary,
    pub stderr: EvidenceStreamSummary,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceTarget {
    pub kind: String,
    pub id: String,
    #[serde(default = "default_evidence_target_role")]
    pub role: String,
}

fn default_evidence_target_role() -> String {
    "validates".to_string()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct EvidenceRecordData {
    pub commit: String,
    pub result: EvidenceResult,
    #[serde(alias = "kind")]
    pub evidence_type: String,
    pub captured_at: DateTime<Utc>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub producer: Option<String>,
    #[serde(default)]
    pub proof_scope: Option<String>,
    #[serde(default)]
    pub agent_identity: Option<String>,
    #[serde(default)]
    pub independence_level: Option<String>,
    #[serde(default)]
    pub residual_risks: Vec<String>,
    #[serde(default)]
    pub follow_up_ids: Vec<String>,
    #[serde(default)]
    pub exit_code: Option<i32>,
    #[serde(default)]
    pub exit_status: Option<String>,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub spawn_error: Option<String>,
    #[serde(default)]
    pub output: Option<EvidenceOutputSummary>,
    #[serde(default)]
    pub target: Option<EvidenceTarget>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RecordLink {
    pub source_kind: String,
    pub source_id: String,
    pub target_kind: String,
    pub target_id: String,
    pub relation_type: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_priority_maps_tokens_labels_and_ranks() {
        let cases = [
            (IssuePriority::P0, "P0", "critical", 0),
            (IssuePriority::P1, "P1", "high", 1),
            (IssuePriority::P2, "P2", "medium", 2),
            (IssuePriority::P3, "P3", "low", 3),
        ];

        for (priority, token, label, rank) in cases {
            assert_eq!(priority.canonical_token(), token);
            assert_eq!(priority.label(), label);
            assert_eq!(priority.sort_rank(), rank);
            assert_eq!(
                IssuePriority::from_canonical_token(token).unwrap(),
                priority
            );
            assert_eq!(IssuePriority::from_label(label).unwrap(), priority);
            assert_eq!(IssuePriority::from_cli_input(label).unwrap(), priority);
        }
    }

    #[test]
    fn default_body_shapes_and_type_changes_preserve_acceptance_and_notes() {
        let parent = IssueSections::for_type("feature", Some("  "));
        assert_eq!(
            parent.section(IssueSectionName::Goal),
            Some("Goal was not specified.")
        );
        assert_eq!(parent.section(IssueSectionName::Unknowns), None);
        let mission = IssueSections::for_type("mission", None);
        assert_eq!(
            mission.section(IssueSectionName::SuccessCriteria),
            Some("Success criteria were not specified.")
        );

        let mut sections = IssueSections::for_type("task", Some("  Keep intent.  "));
        if let IssueBody::Issue {
            done_when,
            scope,
            context,
            ..
        } = &mut sections.body
        {
            *done_when = "The result is independently checked.".into();
            *scope = Some("This repository.".into());
            *context = Some("Existing useful context.".into());
        }
        sections.notes = Some("A preserved note.".into());
        sections.close_reason = Some("A preserved reason.".into());
        let same_shape = sections.clone();
        sections.change_type("bug");
        assert_eq!(sections, same_shape);
        if let IssueBody::Issue { background, .. } = &mut sections.body {
            *background = Some("Shared research.".into());
        }
        sections.change_type("mission");
        assert_eq!(
            sections.section(IssueSectionName::Outcome),
            Some("Keep intent.")
        );
        assert_eq!(
            sections.section(IssueSectionName::Background),
            Some("Shared research.")
        );
        assert_eq!(
            sections.section(IssueSectionName::SuccessCriteria),
            Some("The result is independently checked.")
        );
        sections.change_type("task");
        assert_eq!(
            sections.section(IssueSectionName::Goal),
            Some("Keep intent.")
        );
        assert_eq!(
            sections.section(IssueSectionName::DoneWhen),
            Some("The result is independently checked.")
        );
        assert_eq!(
            sections.section(IssueSectionName::Background),
            Some("Shared research.")
        );
        assert_eq!(sections.notes, same_shape.notes);
        assert_eq!(sections.close_reason, same_shape.close_reason);
    }

    #[test]
    fn issue_priority_rejects_wrong_vocabulary_for_context() {
        let label_error = IssuePriority::from_label("P1").unwrap_err();
        assert_eq!(
            label_error.to_string(),
            "Invalid priority 'P1'. Valid values: critical, high, medium, low"
        );

        let token_error = IssuePriority::from_canonical_token("high").unwrap_err();
        assert_eq!(
            token_error.to_string(),
            "Invalid priority 'high'. Valid values: critical, high, medium, low"
        );
    }
}
