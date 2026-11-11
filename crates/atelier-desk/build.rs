//! Records the commit the server is built from as `ATELIER_DESK_BUILT_FROM`,
//! which `GET /api/repos` reports and the app shows in its footer. It is
//! the full ID of `HEAD`, or `unknown` outside a Git checkout. Changes not
//! committed are not marked, so the install steps build from a clean
//! checkout. Cargo runs this again when `HEAD` or the branch it names moves.

use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn main() {
    for path in ["HEAD", "packed-refs"] {
        if let Some(file) = git(&["rev-parse", "--path-format=absolute", "--git-path", path]) {
            println!("cargo:rerun-if-changed={file}");
        }
    }
    if let Some(branch) = git(&["symbolic-ref", "-q", "HEAD"]) {
        if let Some(file) = git(&["rev-parse", "--path-format=absolute", "--git-path", &branch]) {
            println!("cargo:rerun-if-changed={file}");
        }
    }
    let built_from = git(&["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=ATELIER_DESK_BUILT_FROM={built_from}");
}
