---
schema: atelier.item
schema_version: 3
id: atelier-ni3d
title: Merge atelier-records and atelier-app into atelier-core, as modules, with no change in behaviour
status: building
mission: atelier-dy62
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7528a691-d081-4f17-89b8-d45bea97b6e3%3Adelegate-task%3Ab38e4efa-052b-4634-b058-64f88bc4e1bc
ready:
  by: sumeet, in conversation on 2026-11-14
  at: 2026-11-14T06:21:12.860651075Z
trivial: null
caused_by: null
created_at: 2026-11-14T05:56:52.547204248Z
---

## Goal

Make the three library crates one, named `atelier-core`, by moving files
and changing `use` paths, so that "the core" is one crate and the later
items of atelier-dy62 and atelier-yclc write to its final names. This item
changes no behaviour.

## Done when

- [ ] `crates/atelier-records` and `crates/atelier-app` are gone, and the workspace's members are `atelier-core`, `atelier-cli` and `atelier-desk`. `Cargo.lock` is updated, and `atelier-core`'s dependencies are the union of the three crates', with none added.
- [ ] In `crates/atelier-core/src`, the old `atelier-core` files are under `model/`, the old `atelier-records` files are under `records/`, and `atelier-app`'s `acts.rs` and `git.rs` keep their names. Where the rest of `atelier-app`'s files go is the implementer's choice, and the Report gives the layout in a few lines.
- [ ] No file under `model/` names `std::fs`, `std::process` or a module of the crate outside `model`.
- [ ] Every file is moved with its history (`git mv`), and `git diff -M` against the base shows each moved source file changing only its `use` and `mod` lines, paths in its comments, and visibility where the compiler requires it. The Report lists any file that changed more and says why.
- [ ] `atelier-cli`, `atelier-desk` and every test compile against `atelier_core` alone. The tests of the three crates are in `crates/atelier-core/tests` with their fixtures. No test's inputs or assertions change: a test file's diff is the paths of crates and modules wherever they occur (`atelier_records::put_first` in a test's body, for one), the paths of fixtures that moved (`crates/atelier-app/tests/acts.rs:1253` joins `../atelier-records/tests/fixtures/open-missions`), and the list of crate names in `crates/atelier-desk/tests/dependencies.rs:8`, which loses the two crates that are gone.
- [ ] `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass, with the same number of tests as on the base. The Report gives both counts.
- [ ] `docs/design.md`'s crate table has three rows, every place it and `docs/architecture/desk.md` name `atelier-app` or `atelier-records` is corrected, and its Decisions hold the two rules of atelier-dy62's Outcome. Text that describes the old tool's crates, or what the rewrite copied from where, is history and stays.
- [ ] Nothing outside `.atelier/` and those histories names `atelier-app`, `atelier_app`, `atelier-records` or `atelier_records`: `git grep` for them, and the Report lists what is left and why.

## Scope

In: `crates/`, `Cargo.toml`, `Cargo.lock`, `docs/design.md`, `docs/architecture/desk.md`, and any script, CI file or fixture that names a crate.

Out: renaming `acts.rs`, `Act` or the word "act" (atelier-nl43); moving the Desk's or `at_commit.rs`'s own `git` and `gh` spawns (atelier-x5al); moving anything out of `atelier-cli`; narrowing exports; splitting or merging files; any change in behaviour. Records under `.atelier/` that name the old crates are not edited here.

## Context

On 2026-11-14 there were 189 `use` sites of the three crates in 42 files. `atelier-core` and `atelier-app` each have a `review.rs`, which the `model/` directory keeps apart. `desk/e2e/fixtures.ts:158` names `atelier-app` in a made-up item's title and `scripts/scorecard/draw_samples.py:80` names `crates/atelier-records/src/document.rs` in a comment.

Two gardener branches are approved and unmerged, atelier-0019 and atelier-qb4f. atelier-qb4f edits six module headers under `crates/` and `docs/design.md`'s crate table. Whichever of these three merges later merges the base in first; do not rebase.

Build with a `CARGO_TARGET_DIR` of this worktree's own.
