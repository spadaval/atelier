//! Atelier's use cases: who is calling, the act table, which record of an
//! item counts, what `status` shows, and the acts a program such as the
//! Desk calls as functions, `ready` and `decide`, over a repository opened
//! by path ([`Repo::open`]). Each returns what it did, or a refusal, as
//! data; a [`Skip`] made for the invocation passes the rules its reason
//! covers.

pub mod acts;
mod decide;
pub mod git;
mod identity;
pub mod plan;
pub mod repo;
mod review;
pub mod say;
mod skip;
mod view;
mod work;

pub use decide::{decide, decider, one_line};
pub use identity::identity;
pub use plan::{ready, Done};
pub use repo::{Links, Repo};
pub use review::{carried, carried_plan, Carried};
pub use skip::{refused, Refused, Skip};
pub use view::{entries, entry, overview, Branch, Entry, Located, Overview};
pub use work::{in_progress, items_of, list, Filter, Group, LIFECYCLE, NEEDS_HUMAN};
