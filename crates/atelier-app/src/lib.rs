//! Atelier's use cases: who is calling, the act table, which record of an
//! item counts, what `status` shows, the disagreements the records hold,
//! and the acts a program such as the Desk calls as functions, `ready` and,
//! until atelier-v958 removes the Desk's, `decide`, over a repository
//! opened by path ([`Repo::open`]). Each returns what it did, or a refusal, as
//! data; a [`Skip`] made for the invocation passes the rules its reason
//! covers.

pub mod acts;
mod decide;
mod disputed;
pub mod git;
mod identity;
pub mod log;
pub mod plan;
pub mod repo;
mod review;
pub mod say;
mod show;
mod skip;
mod view;
mod work;

pub use decide::decide;
pub use disputed::{disputes, rebuttals, split_rounds, Disputes, Judged, Rebuttal, SplitRound};
pub use identity::identity;
pub use log::{history, Happened, History, Logged, Move, PassedOver};
pub use plan::{ready, Done};
pub use repo::{Links, Repo};
pub use review::{carried, carried_plan, Carried};
pub use show::{annotate, annotate_all, item_view, mission_view, ItemView, MissionView};
pub use skip::{refused, Refused, Skip, VARIABLE as SKIP_VARIABLE};
pub use view::{
    entries, entry, mark, overview, unreached, Branch, Entry, Located, Overview, ReadFrom, Record,
};
pub use work::{
    in_progress, items_of, list, narrows_item, narrows_mission, Filter, Group, LIFECYCLE,
    NEEDS_HUMAN,
};
