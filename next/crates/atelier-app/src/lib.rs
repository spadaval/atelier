//! Atelier's use cases, over records already read: who is calling, the act
//! table, which record of an item counts, and what `status` shows. The CLI
//! reads the records and runs Git.

pub mod acts;
mod identity;
mod review;
mod view;
mod work;

pub use identity::identity;
pub use review::{carried, carried_plan, Carried};
pub use view::{entries, entry, overview, Branch, Entry, Located, Overview};
pub use work::{in_progress, items_of, list, Filter, Group, LIFECYCLE, NEEDS_HUMAN};
