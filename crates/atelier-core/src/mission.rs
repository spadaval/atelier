use crate::{Ready, SchemaVersion, Status, Thread, Threads, Time};
use serde::{Deserialize, Serialize};

/// A mission's front matter, in the order it is written, and its Markdown
/// body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mission {
    pub schema: MissionSchema,
    pub schema_version: SchemaVersion,
    pub id: String,
    pub title: String,
    pub status: MissionStatus,
    pub labels: Vec<String>,
    /// The thread that leads it, set by `lead`.
    pub lead: Option<Thread>,
    /// Every thread that has planned it, oldest first: `start --mission`
    /// binds the first, and a thread that submits its plan without being
    /// one is added. None of them may review its plan. A record written
    /// before planners were recorded has none.
    #[serde(default)]
    pub planners: Threads,
    /// Who said go, in a record written before a mission had no go-ahead.
    /// It is read and never written, so it does nothing: a mission needs
    /// no go-ahead.
    #[serde(default, skip_serializing)]
    pub ready: Option<Ready>,
    pub created_at: Time,
    #[serde(skip)]
    pub body: String,
}

/// The `schema` tag of a mission record.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionSchema {
    #[default]
    #[serde(rename = "atelier.mission")]
    Mission,
}

/// A mission's stored status. A mission is written `open`, `done` or
/// `dropped`. Whether an open one is proposed or agreed is where it is read
/// from: on a branch that has not merged it is proposed, on the base
/// agreed. A record from before has `ready`, which reads as `open`, or
/// `draft`, which is read and never written: it is proposed wherever it is
/// read, so it cannot be led nor have its items started.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionStatus {
    #[serde(alias = "ready")]
    Open,
    Done,
    Dropped,
    /// The old `draft`, read only.
    Draft,
}

impl MissionStatus {
    pub const ALL: [MissionStatus; 4] = [Self::Open, Self::Done, Self::Dropped, Self::Draft];

    /// The item status the act table writes this status as. The table's
    /// statuses are an item's, and a mission's `open` is written there as
    /// `ready`, the old draft as `draft`.
    pub fn status(self) -> Status {
        match self {
            Self::Draft => Status::Draft,
            Self::Open => Status::Ready,
            Self::Done => Status::Done,
            Self::Dropped => Status::Dropped,
        }
    }

    /// The mission status the act table's `status` stands for, if a
    /// mission can have it.
    pub fn of(status: Status) -> Option<MissionStatus> {
        Self::ALL.into_iter().find(|s| s.status() == status)
    }

    /// Its name as the record stores it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Done => "done",
            Self::Dropped => "dropped",
            Self::Draft => "draft",
        }
    }
}
