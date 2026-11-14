//! Atelier's domain values. Reading and writing them is `records`'.

mod config;
mod item;
mod mission;
mod review;

pub use config::{validate_prefix, validate_slug, Config, Forge, ForgeProvider, MergeCheck, Named};
pub use item::{Answer, Decision, Item, ItemSchema, Priority, Ready, Response, Status};
pub use mission::{Mission, MissionSchema, MissionStatus};
pub use review::{
    is_deferred, is_open, Finding, FindingName, Now, Outcome, Recommendation, Review, ReviewName,
    ReviewSchema, Settlement, Verdict,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A record's time, kept with the offset it was written in so a record
/// reads back as it was written.
pub type Time = chrono::DateTime<chrono::FixedOffset>;

/// The record format this tool reads and writes, `schema_version: 3`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SchemaVersion;

impl SchemaVersion {
    pub const NUMBER: u32 = 3;
}

impl Serialize for SchemaVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(Self::NUMBER)
    }
}

impl<'de> Deserialize<'de> for SchemaVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u32::deserialize(deserializer)? {
            Self::NUMBER => Ok(SchemaVersion),
            other => Err(serde::de::Error::custom(format!(
                "schema version {other} is not {}, the only one this tool reads",
                Self::NUMBER
            ))),
        }
    }
}

/// The T3 Code thread, or harness session, that holds a role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Thread(pub String);

impl fmt::Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Every thread that has held a role, oldest first. The last one holds it
/// now; the earlier ones are why a former implementer can never review.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Threads(pub Vec<Thread>);

impl Threads {
    /// Makes `thread` the holder. A thread that already holds the role is
    /// not added again, so running an entry command twice is no take-over.
    pub fn take(&mut self, thread: Thread) {
        if self.holder() != Some(&thread) {
            self.0.push(thread);
        }
    }

    pub fn holder(&self) -> Option<&Thread> {
        self.0.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn thread(id: &str) -> Thread {
        Thread(id.to_string())
    }

    #[test]
    fn the_last_thread_to_take_a_role_holds_it() {
        let mut threads = Threads::default();
        assert_eq!(threads.holder(), None);
        threads.take(thread("a"));
        threads.take(thread("b"));
        assert_eq!(threads.holder(), Some(&thread("b")));
        threads.take(thread("a"));
        assert_eq!(threads.0, [thread("a"), thread("b"), thread("a")]);
    }

    #[test]
    fn the_holder_taking_its_role_again_adds_nothing() {
        let mut threads = Threads(vec![thread("a")]);
        threads.take(thread("a"));
        assert_eq!(threads.0, [thread("a")]);
    }
}
