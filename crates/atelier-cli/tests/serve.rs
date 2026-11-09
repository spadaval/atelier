//! `atelier serve` (atelier-zpmb): the JSON API over every repository under
//! its roots, read from each one's base commit. The tests live beside the
//! CLI so they can compare the API with what `atelier brief` and
//! `atelier debrief` print.

mod serve {
    mod api;
    mod discovery;
    mod divergent_head;
    pub mod fixture;
    mod read_only;

    use std::path::{Path, PathBuf};

    use atelier_site::{Client, SiteOptions};

    /// A client over `root`.
    pub fn client(root: &Path) -> Client {
        Client::new(SiteOptions::new(vec![root.to_path_buf()]))
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
