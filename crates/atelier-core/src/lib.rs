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
/// backlog, done, todo`, or `none` when it accepts no value.
pub fn invalid_value_message<S: AsRef<str>>(field: &str, value: &str, valid: &[S]) -> String {
    let valid = valid.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    let valid = if valid.is_empty() {
        "none".to_string()
    } else {
        valid.join(", ")
    };
    format!("Invalid {field} '{value}'. Valid values: {valid}")
}

/// A command argument whose value is outside the set its field accepts.
///
/// It reads in the one form [`invalid_value_message`] writes, optionally led
/// by a code and followed by one sentence of guidance. The CLI exits 2 for
/// it, as for every usage error, wherever the value is checked.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvalidValue {
    message: String,
}

impl InvalidValue {
    pub fn new<S: AsRef<str>>(field: &str, value: &str, valid: &[S]) -> Self {
        Self {
            message: invalid_value_message(field, value, valid),
        }
    }

    /// Leads the message with the code of a check that shipped with one.
    pub fn with_code(self, code: &str) -> Self {
        Self {
            message: format!("{code}: {}", self.message),
        }
    }

    /// Follows the valid values with one sentence of guidance.
    pub fn with_hint(self, hint: &str) -> Self {
        Self {
            message: format!("{}. {hint}", self.message),
        }
    }
}

impl fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for InvalidValue {}

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

    /// Parses the stored form, `P0`-`P3`.
    pub fn from_canonical_token(value: &str) -> Result<Self, ParseIssuePriorityError> {
        match value {
            "P0" => Ok(Self::P0),
            "P1" => Ok(Self::P1),
            "P2" => Ok(Self::P2),
            "P3" => Ok(Self::P3),
            other => Err(ParseIssuePriorityError::new(other, PriorityForm::Stored)),
        }
    }

    /// Parses the displayed word, `critical`, `high`, `medium` or `low`.
    pub fn from_label(value: &str) -> Result<Self, ParseIssuePriorityError> {
        match value {
            "critical" => Ok(Self::P0),
            "high" => Ok(Self::P1),
            "medium" => Ok(Self::P2),
            "low" => Ok(Self::P3),
            other => Err(ParseIssuePriorityError::new(other, PriorityForm::Label)),
        }
    }

    /// Parses command input, which accepts the stored form and the word alike.
    pub fn from_cli_input(value: &str) -> Result<Self, InvalidValue> {
        Self::from_canonical_token(value)
            .or_else(|_| Self::from_label(value))
            .map_err(|_| {
                InvalidValue::new(
                    "priority",
                    value,
                    &["P0 or critical", "P1 or high", "P2 or medium", "P3 or low"],
                )
            })
    }
}

/// The stored priority tokens with their words, as record errors state them.
pub const ISSUE_PRIORITY_STORED_FORMS: &str = "P0 (critical), P1 (high), P2 (medium) or P3 (low)";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PriorityForm {
    Stored,
    Label,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseIssuePriorityError {
    value: String,
    expected: PriorityForm,
}

impl ParseIssuePriorityError {
    fn new(value: &str, expected: PriorityForm) -> Self {
        Self {
            value: value.to_string(),
            expected,
        }
    }
}

impl fmt::Display for ParseIssuePriorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let valid = match self.expected {
            PriorityForm::Stored => "P0, P1, P2, P3",
            PriorityForm::Label => "critical, high, medium, low",
        };
        write!(
            f,
            "Invalid priority '{}'. Valid values: {valid}",
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
    /// The ADR a decision record produced (atelier-fip8).
    #[serde(default)]
    pub adr: Option<IssueAdr>,
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
        repository_relative(&path).map_err(IssueFieldError::InvalidStrategy)?;
        Ok(Self { path })
    }
}

/// A repository-relative link from a decision record to the ADR it produced.
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueAdr {
    pub path: String,
}

impl IssueAdr {
    pub fn new(path: impl Into<String>) -> Result<Self, IssueFieldError> {
        let path = path.into();
        repository_relative(&path).map_err(IssueFieldError::InvalidAdr)?;
        Ok(Self { path })
    }
}

fn repository_relative(path: &str) -> Result<(), String> {
    let parsed = Path::new(path);
    if path.trim().is_empty()
        || parsed.is_absolute()
        || !parsed
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
    {
        return Err(
            "path must be a non-empty repository-relative path without parent traversal"
                .to_string(),
        );
    }
    Ok(())
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
    InvalidAdr(String),
}
impl fmt::Display for IssueFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategy(detail) => write!(f, "invalid issue strategy: {detail}"),
            Self::InvalidAdr(detail) => write!(f, "invalid decision ADR: {detail}"),
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

/// A trivial exemption of one commit on the base branch, with no owner issue
/// or work branch (atelier-mbrf). It is stored with the review events, as a
/// `trivial_exemption` that names `base` instead of a target, and covers
/// exactly `commit`, never a later one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BaseExemption {
    pub header: RecordHeader,
    /// The base branch the commit was made on.
    pub base: String,
    pub identity: String,
    pub commit: String,
    /// Why correctness is evident from the diff.
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingDecision {
    FixNow,
    Defer,
    NoAction,
}
impl FindingDecision {
    pub const ALL: [Self; 3] = [Self::FixNow, Self::Defer, Self::NoAction];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixNow => "fix_now",
            Self::Defer => "defer",
            Self::NoAction => "no_action",
        }
    }
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

impl ReviewEvent {
    /// The event's stored name, as in the record's `event` field.
    pub fn name(&self) -> &'static str {
        match self {
            ReviewEvent::Review => "review",
            ReviewEvent::Approval => "approval",
            ReviewEvent::Finding { .. } => "finding",
            ReviewEvent::Disposition { .. } => "disposition",
            ReviewEvent::TrivialExemption => "trivial_exemption",
        }
    }
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

/// One success criterion of a mission (atelier-et6q): a `- <slug>: <text>`
/// bullet under the success criteria heading.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Criterion {
    pub slug: String,
    pub text: String,
}

/// The criteria read from a success criteria section, in document order,
/// and what could not be read as a criterion.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct CriteriaParse {
    pub criteria: Vec<Criterion>,
    pub problems: Vec<String>,
}

impl CriteriaParse {
    pub fn get(&self, slug: &str) -> Option<&Criterion> {
        self.criteria
            .iter()
            .find(|criterion| criterion.slug == slug)
    }
}

/// A criterion slug is lowercase kebab-case (`pipe-safe`) or a legacy
/// positional `S<n>`.
pub fn is_criterion_slug(slug: &str) -> bool {
    let positional = slug
        .strip_prefix('S')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()));
    let kebab = !slug.is_empty()
        && slug.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        });
    positional || kebab
}

/// The body of a strategy's success criteria heading (any level, any case),
/// up to the next heading of the same or a higher level.
pub fn strategy_criteria_section(markdown: &str) -> Option<String> {
    markdown_section(markdown, "success criteria")
}

/// The body of the first heading (any level) whose text is `title`, any
/// case, up to the next heading of the same or a higher level.
pub fn markdown_section(markdown: &str, title: &str) -> Option<String> {
    let heading_level = |line: &str| {
        let level = line.chars().take_while(|c| *c == '#').count();
        (level > 0 && line[level..].starts_with(' ')).then_some(level)
    };
    let mut lines = markdown.lines();
    let level = lines.by_ref().find_map(|line| {
        let level = heading_level(line)?;
        line[level..]
            .trim()
            .eq_ignore_ascii_case(title)
            .then_some(level)
    })?;
    let body = lines
        .take_while(|line| heading_level(line).is_none_or(|next| next > level))
        .collect::<Vec<_>>()
        .join("\n");
    Some(body)
}

/// Reads `- <slug>: <text>` bullets. Indented lines continue the bullet
/// before them; any other top-level bullet, or a repeated slug, is a
/// problem.
pub fn parse_criteria(section: &str) -> CriteriaParse {
    let mut parsed = CriteriaParse::default();
    let mut open = false;
    for line in section.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with(char::is_whitespace) {
            if open {
                if let Some(criterion) = parsed.criteria.last_mut() {
                    criterion.text.push(' ');
                    criterion.text.push_str(line.trim());
                }
            }
            continue;
        }
        open = false;
        let Some(bullet) = line.strip_prefix("- ") else {
            continue;
        };
        let criterion = bullet
            .split_once(':')
            .map(|(slug, text)| (slug.trim(), text.trim()))
            .filter(|(slug, _)| is_criterion_slug(slug));
        match criterion {
            Some((slug, _)) if parsed.get(slug).is_some() => {
                parsed
                    .problems
                    .push(format!("criterion {slug} is defined twice"));
            }
            Some((slug, text)) => {
                parsed.criteria.push(Criterion {
                    slug: slug.to_string(),
                    text: text.to_string(),
                });
                open = true;
            }
            None => parsed.problems.push(format!(
                "bullet `{}` does not start with a criterion slug such as `pipe-safe:` or `S1:`",
                line.chars().take(60).collect::<String>()
            )),
        }
    }
    parsed
}

/// The implementer report's parts beyond the Outcome (atelier-sfsq). Each is
/// a `### <title>` subsection of Close Reason; the text before the first is
/// the Outcome. Policy requires a part with the `report.<key>` field.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReportPart {
    Deviations,
    Decisions,
    Surprises,
    OpenItems,
}

impl ReportPart {
    pub const ALL: [Self; 4] = [
        Self::Deviations,
        Self::Decisions,
        Self::Surprises,
        Self::OpenItems,
    ];

    /// The subsection heading, as in `### Open items`.
    pub fn title(self) -> &'static str {
        match self {
            Self::Deviations => "Deviations",
            Self::Decisions => "Decisions",
            Self::Surprises => "Surprises",
            Self::OpenItems => "Open items",
        }
    }

    /// The `required_fields` entry that requires this part.
    pub fn field(self) -> &'static str {
        match self {
            Self::Deviations => "report.deviations",
            Self::Decisions => "report.decisions",
            Self::Surprises => "report.surprises",
            Self::OpenItems => "report.open_items",
        }
    }

    pub fn from_field(field: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|part| part.field() == field)
    }

    fn from_heading(line: &str) -> Option<Self> {
        let title = line.strip_prefix("###")?;
        if title.starts_with('#') {
            return None;
        }
        let title = title.trim();
        Self::ALL
            .into_iter()
            .find(|part| part.title().eq_ignore_ascii_case(title))
    }
}

/// Close Reason read as the implementer report: the Outcome, then any report
/// parts. A `###` heading that names no part stays in the text around it.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct CloseReport {
    pub outcome: String,
    /// The parts in record order; a repeated heading adds to its part.
    pub parts: Vec<(ReportPart, String)>,
}

impl CloseReport {
    pub fn parse(text: &str) -> Self {
        let mut report = Self::default();
        let mut current: Option<ReportPart> = None;
        let mut outcome = Vec::new();
        let mut bodies: Vec<(ReportPart, Vec<&str>)> = Vec::new();
        for line in text.lines() {
            if let Some(part) = ReportPart::from_heading(line) {
                current = Some(part);
                if !bodies.iter().any(|(existing, _)| *existing == part) {
                    bodies.push((part, Vec::new()));
                }
                continue;
            }
            match current {
                Some(part) => {
                    if let Some((_, body)) =
                        bodies.iter_mut().find(|(existing, _)| *existing == part)
                    {
                        body.push(line);
                    }
                }
                None => outcome.push(line),
            }
        }
        report.outcome = outcome.join("\n").trim().to_string();
        report.parts = bodies
            .into_iter()
            .map(|(part, body)| (part, body.join("\n").trim().to_string()))
            .collect();
        report
    }

    /// The part's text; `None` when the part is absent or empty.
    pub fn part(&self, part: ReportPart) -> Option<&str> {
        self.parts
            .iter()
            .find(|(existing, _)| *existing == part)
            .map(|(_, body)| body.as_str())
            .filter(|body| !body.is_empty())
    }

    /// `text` with its Outcome replaced and its report parts kept.
    pub fn with_outcome(text: &str, outcome: &str) -> String {
        let mut offset = 0;
        for line in text.split_inclusive('\n') {
            if ReportPart::from_heading(line.trim_end_matches(['\n', '\r'])).is_some() {
                return format!("{}\n\n{}", outcome.trim(), text[offset..].trim_end());
            }
            offset += line.len();
        }
        outcome.trim().to_string()
    }
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
    DECISION_NOTE_KIND,
];

/// The note kind for a decision made inside a work item (atelier-fip8). Its
/// tag is `[decision]`, or `[decision one-way]` when it cannot be reversed.
pub const DECISION_NOTE_KIND: &str = "decision";
pub const ONE_WAY_MARK: &str = "one-way";
const DECISION_REASON_LABEL: &str = "Reason:";
const DECISION_REJECTED_LABEL: &str = "Rejected:";

/// One dated `- YYYY-MM-DD [tag] text` bullet from a Notes section. `text`
/// joins the first line and the unindented continuation lines.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IssueNote {
    pub date: String,
    pub tag: String,
    pub text: String,
}

impl IssueNote {
    /// The note kind: the first word of the tag.
    pub fn kind(&self) -> &str {
        self.tag.split_whitespace().next().unwrap_or_default()
    }
}

/// Splits Notes text into its dated bullets. A line that does not open a
/// bullet continues the one before it; text before the first bullet is not a
/// note.
pub fn parse_notes(notes: &str) -> Vec<IssueNote> {
    let mut parsed: Vec<IssueNote> = Vec::new();
    for line in notes.lines() {
        if let Some(note) = note_bullet(line) {
            parsed.push(note);
        } else if let Some(note) = parsed.last_mut() {
            note.text.push('\n');
            note.text.push_str(line.trim());
        }
    }
    for note in &mut parsed {
        note.text = note.text.trim_end().to_string();
    }
    parsed
}

fn note_bullet(line: &str) -> Option<IssueNote> {
    let rest = line.strip_prefix("- ")?;
    let (date, rest) = rest.split_once(' ')?;
    NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let (tag, text) = rest.strip_prefix('[')?.split_once(']')?;
    Some(IssueNote {
        date: date.to_string(),
        tag: tag.trim().to_string(),
        text: text.trim().to_string(),
    })
}

/// A decision note: the choice and the reason are required, rejected
/// alternatives are optional, and a decision is reversible unless one-way.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DecisionNote {
    pub date: String,
    pub choice: String,
    pub reason: String,
    pub rejected: Vec<String>,
    pub one_way: bool,
}

impl DecisionNote {
    /// The note tag and text `append_note` stores for a decision.
    pub fn note_parts(
        choice: &str,
        reason: &str,
        rejected: &[String],
        one_way: bool,
    ) -> (String, String) {
        let tag = if one_way {
            format!("{DECISION_NOTE_KIND} {ONE_WAY_MARK}")
        } else {
            DECISION_NOTE_KIND.to_string()
        };
        let flat = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
        let mut text = format!("{}\n{DECISION_REASON_LABEL} {}", flat(choice), flat(reason));
        for alternative in rejected {
            text.push_str(&format!(
                "\n{DECISION_REJECTED_LABEL} {}",
                flat(alternative)
            ));
        }
        (tag, text)
    }

    /// Builds the note a decision is stored as and parses it back, so the
    /// note command never writes a decision `check` would refuse.
    pub fn checked_note_parts(
        choice: &str,
        reason: &str,
        rejected: &[String],
        one_way: bool,
    ) -> Result<(String, String), String> {
        let starts_with_label = |text: &str| {
            let text = text.trim_start();
            text.starts_with(DECISION_REASON_LABEL) || text.starts_with(DECISION_REJECTED_LABEL)
        };
        if starts_with_label(choice) || rejected.iter().any(|value| starts_with_label(value)) {
            return Err(format!(
                "the choice and rejected alternatives must not start with `{DECISION_REASON_LABEL}` or `{DECISION_REJECTED_LABEL}`"
            ));
        }
        let (tag, text) = Self::note_parts(choice, reason, rejected, one_way);
        let note = IssueNote {
            date: String::new(),
            tag: tag.clone(),
            text: text.clone(),
        };
        match Self::from_note(&note) {
            Some(Ok(_)) => Ok((tag, text)),
            Some(Err(defect)) => Err(format!("the decision note {defect}")),
            None => Err("the note is not a decision".to_string()),
        }
    }

    /// Reads a decision from a note; `None` when the note is another kind,
    /// and an error naming the defect when the decision is malformed.
    pub fn from_note(note: &IssueNote) -> Option<Result<Self, String>> {
        (note.kind() == DECISION_NOTE_KIND).then(|| Self::parse(note))
    }

    fn parse(note: &IssueNote) -> Result<Self, String> {
        let one_way = match note.tag.split_whitespace().collect::<Vec<_>>()[..] {
            [_] => false,
            [_, ONE_WAY_MARK] => true,
            _ => {
                return Err(format!(
                    "tag [{}] is not [decision] or [decision one-way]",
                    note.tag
                ))
            }
        };
        let mut choice = Vec::new();
        let mut reason: Option<Vec<&str>> = None;
        let mut rejected: Vec<String> = Vec::new();
        // Lines after a label continue it until the next label.
        let mut in_reason = false;
        for line in note.text.lines() {
            if let Some(value) = line.strip_prefix(DECISION_REASON_LABEL) {
                if reason.is_some() {
                    return Err("has more than one Reason: line".to_string());
                }
                reason = Some(vec![value.trim()]);
                in_reason = true;
            } else if let Some(value) = line.strip_prefix(DECISION_REJECTED_LABEL) {
                rejected.push(value.trim().to_string());
                in_reason = false;
            } else if in_reason {
                reason.get_or_insert_with(Vec::new).push(line.trim());
            } else if let Some(last) = rejected.last_mut() {
                last.push(' ');
                last.push_str(line.trim());
            } else {
                choice.push(line.trim());
            }
        }
        let join = |parts: &[&str]| {
            parts
                .iter()
                .filter(|part| !part.is_empty())
                .copied()
                .collect::<Vec<_>>()
                .join(" ")
        };
        let choice = join(&choice);
        if choice.is_empty() {
            return Err("has no choice; write the choice on the note's first line".to_string());
        }
        let reason = reason.map(|parts| join(&parts)).unwrap_or_default();
        if reason.is_empty() {
            return Err(format!(
                "has no reason; add an indented `{DECISION_REASON_LABEL} ...` line"
            ));
        }
        if rejected
            .iter()
            .any(|alternative| alternative.trim().is_empty())
        {
            return Err(format!("has an empty `{DECISION_REJECTED_LABEL}` line"));
        }
        Ok(Self {
            date: note.date.clone(),
            choice,
            reason,
            rejected: rejected
                .into_iter()
                .map(|value| value.trim().to_string())
                .collect(),
            one_way,
        })
    }
}

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

    /// The dated bullets in the Notes section.
    pub fn note_entries(&self) -> Vec<IssueNote> {
        self.notes.as_deref().map(parse_notes).unwrap_or_default()
    }

    /// Every decision note, in order, each parsed or with its defect.
    pub fn decision_notes(&self) -> Vec<Result<DecisionNote, String>> {
        self.note_entries()
            .iter()
            .filter_map(DecisionNote::from_note)
            .collect()
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
    pub const ALL: [Self; 4] = [Self::Pass, Self::Fail, Self::Blocked, Self::NotApplicable];

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
    /// The mission criteria this receipt judges, each with its own result
    /// (atelier-et6q).
    #[serde(default)]
    pub criteria: Vec<EvidenceCriterion>,
    /// The mission whose criteria `criteria` names, fixed when the receipt
    /// is recorded so the proof stays with that mission if its target moves.
    #[serde(default)]
    pub criteria_mission: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceCriterion {
    pub slug: String,
    pub result: EvidenceResult,
}

impl std::str::FromStr for EvidenceCriterion {
    type Err = String;

    /// Reads `<slug>=<result>`, as `evidence record --criterion` takes it.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (slug, result) = value.split_once('=').ok_or_else(|| {
            format!(
                "criterion `{value}` needs a result: `<slug>=<result>`, such as `pipe-safe=pass`"
            )
        })?;
        let slug = slug.trim();
        if !is_criterion_slug(slug) {
            return Err(format!(
                "`{slug}` is not a criterion slug; use lowercase kebab-case, such as `pipe-safe`, or `S<n>`"
            ));
        }
        Ok(Self {
            slug: slug.to_string(),
            result: result.trim().parse()?,
        })
    }
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

/// A count as a plain phrase, count first, with a real plural: `1 blocker`,
/// `2 blockers`. Every message that states a count phrases it this way, so
/// none hedges the plural in parentheses. The CLI re-exports it from
/// `human_output`.
pub fn count_noun(count: usize, noun: &str) -> String {
    format!("{count} {noun}{}", plural_suffix(count))
}

/// `s` unless `count` is 1, for a noun whose plural adds `s`.
fn plural_suffix(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn criteria_parse_slugs_continuations_and_problems() {
        let strategy = "# Why\n\n- S9: not here\n\n# Success Criteria\n\n- S1: First\n  continues\n- pipe-safe: Second\n- Plain prose bullet\n- S1: again\n\n## Detail\n\n- S2: nested heading is still in the section\n\n# Shape\n\n- S3: outside\n";
        let section = strategy_criteria_section(strategy).unwrap();
        let parsed = parse_criteria(&section);
        assert_eq!(
            parsed.criteria,
            vec![
                Criterion {
                    slug: "S1".into(),
                    text: "First continues".into()
                },
                Criterion {
                    slug: "pipe-safe".into(),
                    text: "Second".into()
                },
                Criterion {
                    slug: "S2".into(),
                    text: "nested heading is still in the section".into()
                },
            ]
        );
        assert_eq!(parsed.problems.len(), 2, "{:?}", parsed.problems);
        assert!(parsed.problems[0].contains("Plain prose bullet"));
        assert!(parsed.problems[1].contains("S1 is defined twice"));
        assert!(strategy_criteria_section("# Why\n").is_none());
        for slug in ["S1", "S12", "pipe-safe", "a1-b2"] {
            assert!(is_criterion_slug(slug), "{slug}");
        }
        for slug in ["", "S", "Pipe", "pipe_safe", "-x", "x-", "a--b", "s 1"] {
            assert!(!is_criterion_slug(slug), "{slug}");
        }
    }

    fn sections_with_notes(notes: &str) -> IssueSections {
        let mut sections = IssueSections::for_type("task", Some("Goal"));
        sections.notes = Some(notes.to_string());
        sections
    }

    #[test]
    fn decision_notes_round_trip_through_append_note() {
        let mut sections = IssueSections::for_type("task", Some("Goal"));
        sections.append_note(
            NaiveDate::from_ymd_opt(2026, 10, 17).unwrap(),
            "handoff",
            "Earlier",
        );
        let (tag, text) = DecisionNote::note_parts(
            "Store the report in Close Reason",
            "Close Reason is\nexempt from approval freshness",
            &["a new Report section".into(), "a sidecar".into()],
            true,
        );
        sections.append_note(NaiveDate::from_ymd_opt(2026, 10, 18).unwrap(), &tag, &text);
        assert_eq!(
            sections.notes.as_deref().unwrap(),
            "- 2026-10-17 [handoff] Earlier\n- 2026-10-18 [decision one-way] Store the report in Close Reason\n  Reason: Close Reason is exempt from approval freshness\n  Rejected: a new Report section\n  Rejected: a sidecar"
        );
        assert_eq!(
            sections.decision_notes(),
            vec![Ok(DecisionNote {
                date: "2026-10-18".into(),
                choice: "Store the report in Close Reason".into(),
                reason: "Close Reason is exempt from approval freshness".into(),
                rejected: vec!["a new Report section".into(), "a sidecar".into()],
                one_way: true,
            })]
        );
    }

    #[test]
    fn notes_parse_continuations_and_skip_text_before_the_first_bullet() {
        let notes = parse_notes(
            "Loose prose\n- 2026-10-17 [note] First\n  more\n- not a date [x] still the first\n- 2026-10-18 [decision] Use X\n  Reason: Y",
        );
        assert_eq!(notes.len(), 2);
        assert_eq!(
            notes[0].text,
            "First\nmore\n- not a date [x] still the first"
        );
        assert_eq!(notes[1].kind(), "decision");
        let decisions =
            sections_with_notes("- 2026-10-18 [decision] Use X\n  Reason: Y\n  because Z")
                .decision_notes();
        assert_eq!(decisions[0].as_ref().unwrap().reason, "Y because Z");
        assert!(!decisions[0].as_ref().unwrap().one_way);
    }

    #[test]
    fn malformed_decision_notes_name_their_defect() {
        let defect = |notes: &str| {
            sections_with_notes(notes)
                .decision_notes()
                .remove(0)
                .unwrap_err()
        };
        assert!(defect("- 2026-10-18 [decision] Use X").contains("has no reason"));
        assert!(defect("- 2026-10-18 [decision] Use X\n  Reason:   ").contains("has no reason"));
        assert!(defect("- 2026-10-18 [decision]\n  Reason: Y").contains("has no choice"));
        assert!(
            defect("- 2026-10-18 [decision two-way] X\n  Reason: Y").contains("[decision two-way]")
        );
        assert!(
            defect("- 2026-10-18 [decision] X\n  Reason: Y\n  Reason: Z").contains("more than one")
        );
        assert!(defect("- 2026-10-18 [decision] X\n  Reason: Y\n  Rejected:").contains("empty"));
        assert!(sections_with_notes("- 2026-10-18 [decisions] X")
            .decision_notes()
            .is_empty());
    }

    #[test]
    fn close_reports_split_the_outcome_from_known_parts() {
        let text = "Shipped the view.\n### Detail\nkept\n\n### Deviations\nNone.\n### open items\n- follow-up\n#### deeper\n### Surprises\n";
        let report = CloseReport::parse(text);
        assert_eq!(report.outcome, "Shipped the view.\n### Detail\nkept");
        assert_eq!(report.part(ReportPart::Deviations), Some("None."));
        assert_eq!(
            report.part(ReportPart::OpenItems),
            Some("- follow-up\n#### deeper")
        );
        assert_eq!(
            report.part(ReportPart::Surprises),
            None,
            "an empty part is missing"
        );
        assert_eq!(report.part(ReportPart::Decisions), None);
        assert_eq!(
            CloseReport::with_outcome(text, " New outcome "),
            "New outcome\n\n### Deviations\nNone.\n### open items\n- follow-up\n#### deeper\n### Surprises"
        );
        assert_eq!(CloseReport::with_outcome("Old", "New"), "New");
        assert_eq!(
            ReportPart::from_field("report.open_items"),
            Some(ReportPart::OpenItems)
        );
        assert_eq!(ReportPart::from_field("close_reason"), None);
    }

    #[test]
    fn counts_read_count_first_with_a_real_plural() {
        assert_eq!(count_noun(0, "blocker"), "0 blockers");
        assert_eq!(count_noun(1, "blocker"), "1 blocker");
        assert_eq!(
            count_noun(2, "older evidence record"),
            "2 older evidence records"
        );
    }

    #[test]
    fn every_evidence_result_is_listed_and_round_trips() {
        for result in [
            EvidenceResult::Pass,
            EvidenceResult::Fail,
            EvidenceResult::Blocked,
            EvidenceResult::NotApplicable,
        ] {
            // Exhaustive, so a new variant fails to compile until listed above.
            match result {
                EvidenceResult::Pass
                | EvidenceResult::Fail
                | EvidenceResult::Blocked
                | EvidenceResult::NotApplicable => {}
            }
            assert!(
                EvidenceResult::ALL.contains(&result),
                "{result:?} is missing from EvidenceResult::ALL"
            );
        }
        for result in EvidenceResult::ALL {
            assert_eq!(result.as_str().parse::<EvidenceResult>(), Ok(result));
        }
    }

    #[test]
    fn every_finding_decision_is_listed_and_named_as_serialized() {
        for decision in [
            FindingDecision::FixNow,
            FindingDecision::Defer,
            FindingDecision::NoAction,
        ] {
            // Exhaustive, so a new variant fails to compile until listed above.
            match decision {
                FindingDecision::FixNow | FindingDecision::Defer | FindingDecision::NoAction => {}
            }
            assert!(
                FindingDecision::ALL.contains(&decision),
                "{decision:?} is missing from FindingDecision::ALL"
            );
        }
        for decision in FindingDecision::ALL {
            assert_eq!(
                serde_json::to_value(decision).unwrap(),
                serde_json::Value::from(decision.as_str())
            );
        }
    }

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
            "Invalid priority 'high'. Valid values: P0, P1, P2, P3"
        );

        let input_error = IssuePriority::from_cli_input("urgent").unwrap_err();
        assert_eq!(
            input_error.to_string(),
            "Invalid priority 'urgent'. Valid values: P0 or critical, P1 or high, P2 or medium, P3 or low"
        );
    }

    #[test]
    fn issue_priority_cli_input_accepts_stored_tokens_and_words() {
        for priority in [
            IssuePriority::P0,
            IssuePriority::P1,
            IssuePriority::P2,
            IssuePriority::P3,
        ] {
            assert_eq!(
                IssuePriority::from_cli_input(priority.canonical_token()),
                Ok(priority)
            );
            assert_eq!(
                IssuePriority::from_cli_input(priority.label()),
                Ok(priority)
            );
        }
        assert!(IssuePriority::from_cli_input("p1").is_err());
        assert!(IssuePriority::from_cli_input("High").is_err());
    }
}
