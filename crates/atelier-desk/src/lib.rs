//! The Desk's server (docs/architecture/desk.md): the reading routes as
//! JSON, read from commits and branches, local and fetched, through
//! `atelier-app`'s views; each repository's remote fetched on a timer; the
//! human told when a plan is proposed and when it merges; T3 Code's live
//! threads, joined to the records; the stop of a proposed plan, written by
//! `atelier-app`'s own function in a clone of the Desk's and published;
//! planning sessions started in T3 Code; and the built app's files. Its
//! pages read no working tree's records. It runs `git`, `gh`, the
//! configured `t3` command and the configured notify command, and never
//! the `atelier` binary.

pub mod api;
pub mod config;
pub mod fetch;
pub mod news;
mod read;
pub mod server;
mod sessions;
pub mod stop;
pub mod t3;
pub mod threads;

pub use config::Config;
pub use server::{keep_fetching, router, router_for, Desk};

/// The commit this binary was built from, or `unknown` (`build.rs`).
pub const BUILT_FROM: &str = env!("ATELIER_DESK_BUILT_FROM");
