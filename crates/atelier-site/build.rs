//! Builds the web app in `web/` into `web/dist`, which `src/app.rs` embeds.
//!
//! A release build (`cargo install`, `dist`) builds the app with pnpm when
//! `web/dist` is missing or older than the app's sources, and fails naming
//! the missing step when it cannot, so no release binary ships without the
//! app. A debug build only embeds `web/dist` if it exists, so Rust tests need
//! no Node; without it the server answers with a page saying how to build it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::time::SystemTime;

/// The app's sources: a change to any of them makes `web/dist` stale.
const SOURCES: &[&str] = &[
    "src",
    "index.html",
    "package.json",
    "pnpm-lock.yaml",
    "tsconfig.json",
    "vite.config.ts",
];

fn main() {
    let web = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("web");
    for source in SOURCES {
        println!("cargo::rerun-if-changed=web/{source}");
    }
    println!("cargo::rerun-if-changed=web/dist");

    if std::env::var("PROFILE").as_deref() != Ok("release") {
        return;
    }
    let built = modified(&web.join("dist/index.html"));
    let newest_source = SOURCES
        .iter()
        .filter_map(|source| newest(&web.join(source)))
        .max();
    if built.is_some() && built >= newest_source {
        return;
    }
    if !runs(Command::new("pnpm").arg("--version")) {
        fail(&format!(
            "building atelier's web app needs pnpm, which is not on PATH.\n\
             Install Node.js and pnpm (`corepack enable pnpm`, or https://pnpm.io/installation), \
             then build again; or build the app yourself with \
             `pnpm install --frozen-lockfile && pnpm build` in {}.",
            web.display()
        ));
    }
    for step in [&["install", "--frozen-lockfile"][..], &["build"][..]] {
        let command = format!("pnpm {}", step.join(" "));
        eprintln!("atelier-site: running `{command}` in {}", web.display());
        if !runs(Command::new("pnpm").args(step).current_dir(&web)) {
            fail(&format!(
                "`{command}` failed in {}; its output is above. Fix it and build again.",
                web.display()
            ));
        }
    }
}

/// Whether `command` ran and exited successfully. Its output goes to Cargo's
/// build log.
fn runs(command: &mut Command) -> bool {
    command
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn fail(message: &str) -> ! {
    eprintln!("error: {message}");
    exit(1);
}

fn modified(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).and_then(|meta| meta.modified()).ok()
}

/// The newest modification time of `path` or anything under it.
fn newest(path: &Path) -> Option<SystemTime> {
    let meta = fs::symlink_metadata(path).ok()?;
    let own = meta.modified().ok();
    if !meta.is_dir() {
        return own;
    }
    fs::read_dir(path)
        .ok()?
        .flatten()
        .filter_map(|entry| newest(&entry.path()))
        .chain(own)
        .max()
}
