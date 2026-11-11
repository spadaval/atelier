//! The Desk's server (docs/architecture/desk.md): the reading routes as
//! JSON, read from commits through `atelier-app`'s views; T3 Code's live
//! threads, joined to the records; the acts `ready` and `decide`, made by
//! `atelier-app`'s own functions; planning sessions started in T3 Code;
//! and the built app's files. Its pages read no working tree's records, and
//! it runs no binary but the configured `t3` command, for a token.

pub mod acts;
pub mod api;
pub mod config;
mod read;
pub mod server;
mod sessions;
pub mod t3;
pub mod threads;

pub use config::Config;
pub use server::{router, router_for, Desk};

/// The commit this binary was built from, or `unknown` (`build.rs`).
pub const BUILT_FROM: &str = env!("ATELIER_DESK_BUILT_FROM");
