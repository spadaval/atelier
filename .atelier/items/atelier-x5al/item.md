---
schema: atelier.item
schema_version: 3
id: atelier-x5al
title: Run every git and gh call through the core's git and forge modules
status: draft
mission: atelier-dy62
labels: []
priority: medium
blocked_by:
- atelier-ni3d
- atelier-yclc
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-14T05:56:52.574493145Z
---

## Goal

Give Git and the forge one home each. Today `git` is spawned from the
core's helper, from `at_commit.rs` beside it, and from the Desk's `fetch.rs`
and `stop.rs`, and `gh` from the forge code and from the same two files of
the Desk. Each copy sets its own flags and words its own errors.

## Done when

- [ ] Outside `tests/` directories and build scripts, `Command::new("git")` occurs in one source file of the workspace, `crates/atelier-core/src/git.rs`, and a `gh` program is spawned from one, the core's `forge` module. `git grep` shows it, and the Report quotes the two results.
- [ ] `records/at_commit.rs` starts its `git cat-file --batch` through the `git` module's command.
- [ ] The Desk's fetch, clone, config, commit and push go through the `git` module. Its calls still ask nothing of a terminal (`GIT_TERMINAL_PROMPT=0` and `ssh -o BatchMode=yes`), as a choice the caller makes, and the command's own calls are unchanged.
- [ ] The Desk's `gh pr list` and `gh pr close` are functions of the `forge` module, taking the `gh` program's path as they do now.
- [ ] No command's or page's behaviour changes: the tests of `atelier-cli` and `atelier-desk` pass unchanged, and the Desk's interface tests (`pnpm e2e`) pass.
- [ ] `docs/design.md`'s crate table and `docs/architecture/desk.md` say where Git and the forge are called from.

## Scope

In: `crates/atelier-core/src/git.rs`, the `forge` module, `records/at_commit.rs`, `crates/atelier-desk/src/fetch.rs` and `stop.rs`, `crates/atelier-cli/src/git.rs` where a helper moves, and the two documents.

Out: the other programs the code spawns (hooks in `check.rs`, the merge checks' `sh` in `merge.rs`, the Desk's notify program and T3 Code client); `crates/atelier-desk/build.rs`, which runs `git` at compile time to stamp the commit and cannot call the crate it builds for; what any Git or `gh` call does; a Git library in place of the `git` program; the tests' own Git calls.

## Context

It waits on atelier-yclc, which moves `forge.rs` from `atelier-cli` into the core. `at_commit.rs` has its own spawn because `atelier-records` sat below `atelier-app` and could not call its helper; atelier-ni3d removes that reason.
