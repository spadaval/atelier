//! The Desk's server (docs/architecture/desk.md): the reading routes as
//! JSON, read from commits through `atelier-app`'s views; T3 Code's live
//! threads, joined to the records; and the built app's files. It reads no
//! working tree's records, and runs no binary but the configured `t3`
//! command, for a token.

pub mod api;
pub mod config;
mod read;
pub mod server;
pub mod t3;
pub mod threads;

pub use config::Config;
pub use server::{router, router_for, Desk};
