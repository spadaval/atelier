---
schema: atelier.item
schema_version: 3
id: atelier-x5al
title: Run every git and gh call through the core's git and forge modules
status: approved
mission: atelier-dy62
labels: []
priority: medium
blocked_by:
- atelier-ni3d
- atelier-nl43
implementers:
- thread:delegated-task:command%3Amcp%3A7528a691-d081-4f17-89b8-d45bea97b6e3%3Adelegate-task%3A8b1ae668-ea37-441e-9029-3a499f04af34
ready:
  by: sumeet, in conversation on 2026-11-14
  at: 2026-11-14T06:21:12.860651075Z
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

- [x] Outside `tests/` directories and build scripts, `Command::new("git")` occurs in one source file of the workspace, `crates/atelier-core/src/git.rs`, and a `gh` program is spawned from one, the core's `forge` module. `git grep` shows it, and the Report quotes the two results.
- [x] `crates/atelier-cli/src/forge.rs` is `crates/atelier-core/src/forge.rs`, moved with its history, and `atelier-cli` calls it there. Its diff is its `use` lines and the paths in its comments; if it needs more than that, the Report says what and why. `atelier-core` takes `serde_json` and `tempfile` (which `forge.rs:312` uses for the deadline's files, and the core has only as a dev-dependency) from the workspace's dependencies for it, and nothing new to the workspace is added.
- [x] If the forge code cannot move without changing what it does or prints, the item stops at the Git half: every line here but that one and the two about `gh`, with the Report saying what stood in the way. The mission's criterion one-git is then met for `git` and not for `gh`, and its Result says so.
- [x] `records/at_commit.rs` starts its `git cat-file --batch` through the `git` module's command.
- [x] The Desk's fetch, clone, config, commit and push go through the `git` module. Its calls still ask nothing of a terminal (`GIT_TERMINAL_PROMPT=0` and `ssh -o BatchMode=yes`), as a choice the caller makes, and the command's own calls are unchanged.
- [x] The Desk's `gh pr list` and `gh pr close` are functions of the `forge` module, taking the `gh` program's path as they do now.
- [x] No command's or page's behaviour changes: the tests of `atelier-cli` and `atelier-desk` pass unchanged, and the Desk's interface tests (`pnpm e2e`) pass.
- [x] `docs/design.md`'s crate table and `docs/architecture/desk.md` say where Git and the forge are called from. The Decision this mission added to `docs/design.md` (line 1698 on 2026-11-14), which says `submit` and `merge` move with atelier-yclc, says instead that they stay in `atelier-cli` until a second caller needs them, and that the forge code moved because the Desk calls it.

## Scope

In: `crates/atelier-core/src/git.rs`, `crates/atelier-cli/src/forge.rs` and its move, `crates/atelier-core/Cargo.toml`, `Cargo.lock`, `records/at_commit.rs`, `crates/atelier-desk/src/fetch.rs` and `stop.rs`, `crates/atelier-cli/src/git.rs` where a helper moves, and the two documents.

Out: the other programs the code spawns (hooks in `check.rs`, the merge checks' `sh` in `merge.rs`, the Desk's notify program and T3 Code client); `crates/atelier-desk/build.rs`, which runs `git` at compile time to stamp the commit and cannot call the crate it builds for; what any Git or `gh` call does; a Git library in place of the `git` program; the tests' own Git calls; `merge.rs`, `submit` and anything else of `atelier-cli` but `forge.rs` and the lines that call it; every other mention of atelier-yclc in the documents and the Desk, which is atelier-f3ax's, and the wording of the Desk's refusals, which does not change here.

## Context

It waited on atelier-yclc of atelier-to83, which was to move `forge.rs` into the core. Sumeet dropped that item on 2026-11-14 and said this one moves the file itself. `forge.rs` (473 lines) uses `git::run`, which is the core's, `atelier_core::model::Forge`, `serde_json`, `tempfile` and the standard library, and nothing else of `atelier-cli`; `review.rs`, `merge.rs`, `output.rs` and `commands.rs` call it. `at_commit.rs` has its own spawn because `atelier-records` sat below `atelier-app` and could not call its helper; atelier-ni3d removes that reason.

## Report

Every `git` and `gh` program the shipped code starts now starts in one
file each: `crates/atelier-core/src/git.rs` and
`crates/atelier-core/src/forge.rs`. No command's or page's output
changed. `cargo nextest run --workspace` ran 362 tests, 362 passed and 1
skipped, the same count as the base; `pnpm e2e` passed 50 of 50 on a fresh
`pnpm build`. `cargo fmt -- --check` and `cargo clippy --all-targets -- -D
warnings` pass.

`git grep -n 'Command::new("git")' -- crates ':!**/tests/**'` gives two
lines, the second being the Desk's build script, which the item leaves out:

```
crates/atelier-core/src/git.rs:18:    let mut command = Command::new("git");
crates/atelier-desk/build.rs:10:    let output = Command::new("git").args(args).output().ok()?;
```

`git grep -nE 'Command::new\((gh|"gh")\)' -- crates ':!**/tests/**'`
gives the forge module alone: the command's `Github::gh`, and the Desk's
two functions, which run the `gh` its configuration names.

```
crates/atelier-core/src/forge.rs:319:        let mut child = match Command::new("gh")
crates/atelier-core/src/forge.rs:470:    let output = Command::new(gh)
crates/atelier-core/src/forge.rs:496:    let output = Command::new(gh)
```

The forge code moved, so the fallback line, stopping at the Git half, did
not apply. Commit by commit:

- d4b63e129 is the move alone. `git show -M` gives `crates/{atelier-cli =>
  atelier-core}/src/forge.rs`, 99% similar, and `git log --follow` reaches
  its history before the move. Its diff is one `use` line
  (`atelier_core::model::Forge` to `crate::model::Forge`) and one comment
  path (`output.rs` to `crates/atelier-cli/src/output.rs`, since it is no
  longer beside it). `use crate::git;` stays as it was: in the CLI it
  reached `run` through the CLI's re-export of the core's, and in the core
  it is that function. `atelier-cli` drops `mod forge` and its four callers
  say `atelier_core::forge::` for `crate::forge::`. `atelier-core` takes
  `serde_json` and `tempfile` from the workspace; `tempfile` leaves its
  dev-dependencies, since a dependency serves the tests too. `Cargo.lock`
  gains one line, `serde_json` under `atelier-core`; nothing is added to the
  workspace. `atelier-cli` keeps both, which `setup.rs` uses.
- f3a6cd791 routes the rest. `records/at_commit.rs` starts `cat-file
  --batch` with `crate::git::git(repo)`, which supplies the
  `--no-replace-objects` it passed itself, with its error words unchanged.
  `git.rs` gains `unattended(&mut Command)`, which sets
  `GIT_TERMINAL_PROMPT=0` and `GIT_SSH_COMMAND=ssh -o BatchMode=yes`; the
  Desk's fetch and its stop's `run` (fetch, checkout, clean, branch,
  remote, config, push) call it on `git::git(dir)`. The command's calls do
  not. The Desk's clone runs through `git::git(Path::new("."))`, where it
  ran before, since the configured paths may be relative; it set no
  prompt variables before and sets none now, as a clone from a local path
  asks nothing. The Desk's `gh pr list` and `gh pr close` are
  `forge::list_pull_requests(gh, dir, state, limit)` and
  `forge::close_pull_request(gh, dir, branch, comment)`, moved whole with
  their error words; `Listed` moved with them, and the Desk still builds
  the comment from its caller and the reason.
- 0cb5a1141 is the two documents. `docs/design.md`'s crate table names
  `git` and `forge` as the one place each program starts and says what
  the CLI and the Desk call through them; its Decision on moving code into
  the core now says the forge code moved because the Desk calls it, and
  `submit` and `merge` stay in `atelier-cli` until a second caller needs
  them. `docs/architecture/desk.md` names `unattended` and
  `list_pull_requests` under Fetching, adds `close_pull_request` to the
  table of action functions, and says how the server starts `git` and
  `gh`.

Two things a reviewer should weigh:

- The Desk's Git calls now carry `--no-replace-objects`, which
  `git::git` puts on every call and they did not have. I judged this no
  change in behaviour: fetch and push ignore replacements already, the
  Desk's clone fetches only `refs/heads/*` so it holds no `refs/replace/`,
  and the core's own reads in that clone, `Repo::open` and `commit_paths`,
  already pass it, so the checkout and the reads now see the same objects.
  A `git::git` without the flag would have kept the old calls exact at the
  cost of a second kind of command in the module.
- `forge.rs` grew by the two Desk functions after the move. The
  Done-when on the move limits its diff to `use` lines and comment paths,
  and the move commit keeps to that; the additions are the line about the
  Desk's `gh pr list` and `gh pr close` becoming functions of the module,
  in their own commit, with two sentences in the module comment naming
  them.

`crates/atelier-cli/src/git.rs` needed no change: no helper of it moved.
