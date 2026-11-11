use crate::{Ready, SchemaVersion, Status, Thread, Time};
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

/// The four statuses a mission can have, a subset of an item's, so a
/// record that names an item's status is refused when it is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionStatus {
    Draft,
    Ready,
    Done,
    Dropped,
}

impl MissionStatus {
    pub const ALL: [MissionStatus; 4] = [Self::Draft, Self::Ready, Self::Done, Self::Dropped];

    /// The same status as an item's, which the act table is written in.
    pub fn status(self) -> Status {
        match self {
            Self::Draft => Status::Draft,
            Self::Ready => Status::Ready,
            Self::Done => Status::Done,
            Self::Dropped => Status::Dropped,
        }
    }

    /// The mission status that is `status`, if a mission can have it.
    pub fn of(status: Status) -> Option<MissionStatus> {
        Self::ALL.into_iter().find(|s| s.status() == status)
    }
}
