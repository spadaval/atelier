//! `setup`: the three Git hooks that run `check --hook`, and the harness
//! hooks that start a session with `status`
//! (docs/design.md, Hooks and the bypass, and
//! Threads and roles). It writes only inside the repository it runs in.

use crate::commands::Outcome;
use crate::{git, output};
use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::io::{ErrorKind, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// The Git hooks `setup` installs, each running `check --hook <name>`.
pub const HOOKS: [&str; 3] = ["pre-commit", "pre-push", "post-checkout"];

/// The script `setup` writes for the hook `name`. A hook is Atelier's when
/// it is this file, byte for byte; Git runs it when it is executable too.
fn script(name: &str) -> String {
    format!("#!/bin/sh\nexec atelier check --hook {name} \"$@\"\n")
}

fn is_ours(path: &Path, name: &str) -> bool {
    fs::symlink_metadata(path).is_ok_and(|metadata| metadata.is_file())
        && fs::read(path).is_ok_and(|bytes| bytes == script(name).as_bytes())
}

/// Whether anything is at `path`, a dangling symbolic link included.
fn present(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

/// Whether Git would run the hook at `path`: a file, or a link to one,
/// that is executable.
pub fn runs(path: &Path) -> bool {
    fs::metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

/// Whether the directory Git runs this clone's hooks from has any of
/// Atelier's, for `status`.
pub fn installed(root: &Path) -> Result<bool> {
    let dir = git::hooks_dir(root)?;
    Ok(HOOKS.iter().any(|name| {
        let hook = dir.join(name);
        is_ours(&hook, name) && runs(&hook)
    }))
}

/// What `setup` does to one hook.
#[derive(Clone, Copy, PartialEq)]
pub enum Step {
    /// The hook is Atelier's already.
    Unchanged,
    /// No hook was there, or Atelier's was not executable; Atelier's is
    /// written.
    Written,
    /// The clone's own hook is moved to `<name>.local` and Atelier's
    /// written; `runs` is whether Git ran it, and so `check` will.
    Kept { runs: bool },
}

/// `atelier setup`: Atelier's three hooks in the hooks directory every
/// worktree of the clone shares. A hook it did not write is moved to
/// `<name>.local`, which `check --hook` runs after its own rule; where
/// that name is taken, nothing is changed.
pub fn git_hooks(root: &Path) -> Result<Outcome> {
    let dir = git::hooks_dir(root)?;
    let common = git::common_dir(root)?;
    // core.hooksPath can point Git outside the clone, or at a directory of
    // each worktree's own.
    if !dir.starts_with(&common) {
        let shared = common.join("hooks");
        return Ok(Outcome::Refused(output::hooks_elsewhere(&dir, &shared)));
    }
    let mut steps = Vec::new();
    let mut taken = Vec::new();
    for name in HOOKS {
        let hook = dir.join(name);
        let step = if is_ours(&hook, name) && runs(&hook) {
            Step::Unchanged
        } else if is_ours(&hook, name) || !present(&hook)? {
            Step::Written
        } else if present(&dir.join(format!("{name}.local")))? {
            taken.push(name);
            continue;
        } else {
            Step::Kept { runs: runs(&hook) }
        };
        steps.push((name, step));
    }
    if !taken.is_empty() {
        return Ok(Outcome::Refused(output::hooks_taken(&dir, &taken)));
    }
    fs::create_dir_all(&dir)?;
    for &(name, step) in &steps {
        // Stopped between the move and the write, `setup` run again
        // finds no hook and the kept one in place, and writes Atelier's.
        if let Step::Kept { .. } = step {
            fs::rename(dir.join(name), dir.join(format!("{name}.local")))?;
        }
        if step != Step::Unchanged {
            replace(&dir.join(name), script(name).as_bytes(), 0o755)?;
        }
    }
    Ok(Outcome::Done(output::hooks_set(&dir, &steps)))
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Harness {
    Claude,
    Codex,
}

/// `atelier setup claude` and `setup codex`: the harness's project hooks
/// that start a session with `status`, and for Claude Code end each turn
/// with `check`, merged into the hook settings the repository has.
pub fn harness(root: &Path, harness: Harness) -> Result<Outcome> {
    let (file, hooks): (&str, &[(&str, &str)]) = match harness {
        // A Stop hook that exits 2 makes Claude Code go on and run it
        // again, so an error of `check` is ended as a plain failure.
        Harness::Claude => (
            ".claude/settings.json",
            &[
                ("SessionStart", "atelier status"),
                ("Stop", "atelier check || exit 1"),
            ],
        ),
        // Codex 0.167 runs a project's SessionStart hooks from
        // .codex/hooks.json, in the form Claude Code's settings use.
        Harness::Codex => (".codex/hooks.json", &[("SessionStart", "atelier status")]),
    };
    Ok(match merge_hooks(root, file, hooks)? {
        Err(why) => Outcome::Refused(output::harness_refused(harness, file, &why)),
        Ok(changed) => Outcome::Done(output::harness_set(harness, file, changed)),
    })
}

/// Puts each `(event, command)` of `hooks` into the hook settings at
/// `file` under `root`, keeping every other hook there, and says whether
/// the file changed; or says why the file cannot take them.
///
/// Copied from `crates/atelier-cli/src/commands/setup.rs` (`run`), trimmed
/// of its `--binary` and `--quiet` options, its message and its check of
/// `CLAUDE.md`, and changed to leave a hook already in place where it is,
/// so that a second run changes nothing.
fn merge_hooks(root: &Path, file: &str, hooks: &[(&str, &str)]) -> Result<Result<bool, String>> {
    let path = root.join(file);
    let directory = path.parent().expect("the settings file is in a directory");
    for candidate in [directory, &path] {
        if fs::symlink_metadata(candidate).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            return Ok(Err(format!(
                "{} is a symbolic link, which setup would replace with a file",
                candidate.strip_prefix(root).unwrap_or(candidate).display()
            )));
        }
    }
    let before = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(error.into()),
    };
    let mut settings: Value = if before.is_empty() {
        json!({})
    } else {
        match serde_json::from_slice(&before) {
            Ok(settings) => settings,
            Err(error) => return Ok(Err(format!("{file} is not valid JSON: {error}"))),
        }
    };
    let Some(events) = settings
        .as_object_mut()
        .map(|object| object.entry("hooks").or_insert_with(|| json!({})))
        .and_then(Value::as_object_mut)
    else {
        return Ok(Err(format!(
            "{file} is not an object whose hooks are an object"
        )));
    };
    for &(event, action) in hooks {
        // The marker is the old tool's, so that each tool's setup replaces
        // the other's hooks.
        let marker = format!(" # atelier-managed:{event}");
        let command = format!("{action}{marker}");
        let Some(groups) = events
            .entry(event)
            .or_insert_with(|| json!([]))
            .as_array_mut()
        else {
            return Ok(Err(format!("hooks.{event} in {file} is not a list")));
        };
        let mut present = false;
        for group in groups.iter_mut() {
            let Some(handlers) = group.get_mut("hooks").and_then(Value::as_array_mut) else {
                return Ok(Err(format!(
                    "an entry of hooks.{event} in {file} has no hooks list"
                )));
            };
            // Atelier's handlers that are not this one are an older setup's.
            let count = handlers.len();
            handlers.retain(|handler| {
                handler["command"]
                    .as_str()
                    .is_none_or(|text| !text.ends_with(&marker) || text == command)
            });
            present |= handlers
                .iter()
                .any(|handler| handler["command"] == command.as_str());
            if handlers.is_empty() && count > 0 {
                *group = Value::Null;
            }
        }
        groups.retain(|group| !group.is_null());
        if !present {
            groups.push(json!({"hooks": [{"type": "command", "command": command, "timeout": 30}]}));
        }
    }
    let mut after = serde_json::to_vec_pretty(&settings)?;
    after.push(b'\n');
    if after == before {
        return Ok(Ok(false));
    }
    fs::create_dir_all(directory)?;
    // Keep the settings file's mode; a new one gets 0644.
    let mode = fs::metadata(&path).map_or(0o644, |metadata| metadata.permissions().mode());
    if fs::read(&path).unwrap_or_default() != before {
        anyhow::bail!("{file} changed while setup ran; nothing was written. Run it again");
    }
    replace(&path, &after, mode)?;
    Ok(Ok(true))
}

/// Writes `bytes` to `path` with `mode` in one step: a file written beside
/// it, then renamed over it.
fn replace(path: &Path, bytes: &[u8], mode: u32) -> Result<()> {
    let mut staged =
        tempfile::NamedTempFile::new_in(path.parent().expect("a file is in a directory"))?;
    staged.write_all(bytes)?;
    staged
        .as_file()
        .set_permissions(fs::Permissions::from_mode(mode))?;
    staged.as_file().sync_all()?;
    staged.persist(path).map_err(|error| error.error)?;
    Ok(())
}
