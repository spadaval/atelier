//! The Desk's server (docs/architecture/desk.md): the reading routes as
//! JSON, read from commits through `atelier-app`'s views, and the built
//! app's files. It reads no working tree's records and runs no binary.

pub mod api;
pub mod config;
mod read;
pub mod server;

pub use config::Config;
pub use server::{router, router_for, Desk};
