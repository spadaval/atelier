//! `atelier serve` (atelier-zpmb, atelier-ahpf): the JSON API over every
//! repository under its roots, read from each one's base commit. The tests
//! live beside the CLI so they can compare the API with what
//! `atelier brief`, `atelier debrief` and `atelier status` print.

mod serve {
    mod api;
    mod board;
    mod discovery;
    mod divergent_head;
    pub mod fixture;
    mod inbox;
    mod read_only;

    use std::path::{Path, PathBuf};

    use atelier_site::{Client, Clock, SiteOptions};

    /// The day fixture clients take it to be: 2026-01-10, nine days after
    /// the fixture records' epoch.
    pub fn fixture_now() -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(1_767_225_600 + 9 * 86_400, 0).unwrap()
    }

    /// A client over `root`, on the fixture clock.
    pub fn client(root: &Path) -> Client {
        client_at(root, Clock::new(fixture_now))
    }

    /// A client over `root` on `clock`.
    pub fn client_at(root: &Path, clock: Clock) -> Client {
        Client::new(SiteOptions {
            clock,
            ..SiteOptions::new(vec![root.to_path_buf()])
        })
    }

    /// `GET path`, failing the test unless it answers `status`, as JSON.
    pub fn get_json(client: &Client, path: &str, status: u16) -> serde_json::Value {
        let (answered, body) = client.get(path);
        assert_eq!(answered, status, "GET {path}: {body}");
        serde_json::from_str(&body).unwrap_or_else(|error| panic!("GET {path}: {error}: {body}"))
    }

    /// A scratch directory for fixture repositories.
    pub fn scratch() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    /// The canonical form of `path`, as discovery reports checkouts.
    pub fn canonical(path: &Path) -> PathBuf {
        std::fs::canonicalize(path).unwrap()
    }
}
