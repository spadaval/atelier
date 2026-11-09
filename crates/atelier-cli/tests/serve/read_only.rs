//! The server writes nothing: with every fixture repository's working tree
//! and `.git` directory unwritable, every route answers as it does with them
//! writable, and refs, the index and `git status` are unchanged.

use std::path::Path;
use std::process::Command;

use super::api::{fleet, routes};
use super::{client, scratch};

/// Refs, index bytes and status of the repository checked out at `dir`.
fn state(dir: &Path) -> (String, Vec<u8>, String) {
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .current_dir(dir)
            .args(args)
            // Reading the status must not refresh the index itself.
            .env("GIT_OPTIONAL_LOCKS", "0")
            .output()
            .unwrap();
        assert!(output.status.success(), "git {args:?}");
        String::from_utf8(output.stdout).unwrap()
    };
    let index = std::fs::read(dir.join(".git/index")).unwrap();
    (
        git(&[
            "for-each-ref",
            "--format=%(refname) %(objectname) %(symref)",
        ]),
        index,
        git(&[
            "status",
            "--porcelain=v2",
            "--branch",
            "--untracked-files=all",
        ]),
    )
}

fn chmod(mode: &str, dir: &Path) {
    let status = Command::new("chmod")
        .args(["-R", mode])
        .arg(dir)
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn every_route_answers_the_same_from_unwritable_repositories() {
    let root = scratch();
    let (alpha, beta) = fleet(root.path());
    let paths = routes()
        .into_iter()
        .map(|(route, _)| route)
        .collect::<Vec<_>>();
    let before = [state(&alpha.dir), state(&beta.dir)];
    let writable = paths
        .iter()
        .map(|path| client(root.path()).get(path))
        .collect::<Vec<_>>();
    chmod("a-w", root.path());
    let probe = std::fs::write(alpha.dir.join(".git/probe"), "x");
    // A fresh server, so every view is read and built while unwritable.
    let unwritable = paths
        .iter()
        .map(|path| client(root.path()).get(path))
        .collect::<Vec<_>>();
    chmod("u+w", root.path());
    assert!(
        probe.is_err(),
        "the fixture's .git stayed writable, so this test proves nothing (running as root?)"
    );
    for ((path, writable), unwritable) in paths.iter().zip(&writable).zip(&unwritable) {
        let expected = if path.contains("alpha-m2/strategy")
            || path.contains("beta-m") && path.ends_with("/strategy")
            || path.contains("nowhere")
            || path.contains("alpha-zzzz")
        {
            404
        } else {
            200
        };
        assert_eq!(writable.0, expected, "GET {path}: {}", writable.1);
        assert_eq!(unwritable, writable, "GET {path}");
    }
    assert_eq!([state(&alpha.dir), state(&beta.dir)], before);
}
