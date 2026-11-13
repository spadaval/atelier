use crate::{FindingName, Outcome, SchemaVersion, Thread, Threads, Time};
use serde::{Deserialize, Serialize};

/// An item's front matter, in the order it is written, and its Markdown
/// body. The body's sections are read by `atelier-records`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub schema: ItemSchema,
    pub schema_version: SchemaVersion,
    pub id: String,
    pub title: String,
    pub status: Status,
    pub mission: Option<String>,
    pub labels: Vec<String>,
    pub priority: Priority,
    pub blocked_by: Vec<String>,
    pub implementers: Threads,
    pub ready: Option<Ready>,
    /// The reason given to `merge --trivial`.
    pub trivial: Option<String>,
    /// On a bug, the item ID or commit hash that introduced the defect.
    pub caused_by: Option<String>,
    pub created_at: Time,
    #[serde(skip)]
    pub body: String,
}

/// The `schema` tag of an item record.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemSchema {
    #[default]
    #[serde(rename = "atelier.item")]
    Item,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Draft,
    Ready,
    Building,
    InReview,
    ChangesRequested,
    Approved,
    Done,
    Dropped,
}

/// Ordered from most to least urgent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// Who said an item should start now, and when.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ready {
    pub by: String,
    pub at: Time,
}

/// One entry of an item's Decisions section, which `escalate` wrote until
/// it was removed: a question, the finding it is about if any, and once
/// answered the answer and its outcome for that finding. The binary still
/// reads one, so an item written then reads as it did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub question: String,
    pub finding: Option<FindingName>,
    pub answer: Option<Answer>,
    pub outcome: Option<Outcome>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub by: Thread,
    pub text: String,
}

/// One line of the Report's Responses: how the implementer answered an
/// open fix-now finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Fixed {
        finding: FindingName,
        commit: String,
    },
    Rebutted {
        finding: FindingName,
        fact: String,
    },
}

impl Response {
    pub fn finding(&self) -> &FindingName {
        match self {
            Response::Fixed { finding, .. } | Response::Rebutted { finding, .. } => finding,
        }
    }
}
