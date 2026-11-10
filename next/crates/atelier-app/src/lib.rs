//! Atelier's use cases, over records already read: who is calling, the act
//! table, which record of an item counts, and what `status` shows. The CLI
//! reads the records and runs Git.

pub mod acts;
mod identity;
mod view;

pub use identity::identity;
pub use view::{entry, overview, Branch, Entry, Located, Overview};
