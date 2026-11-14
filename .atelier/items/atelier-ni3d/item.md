---
schema: atelier.item
schema_version: 3
id: atelier-ni3d
title: Merge atelier-records and atelier-app into atelier-core, as modules, with no change in behaviour
status: done
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

- [x] `crates/atelier-records` and `crates/atelier-app` are gone, and the workspace's members are `atelier-core`, `atelier-cli` and `atelier-desk`. `Cargo.lock` is updated, and `atelier-core`'s dependencies are the union of the three crates', with none added.
- [x] In `crates/atelier-core/src`, the old `atelier-core` files are under `model/`, the old `atelier-records` files are under `records/`, and `atelier-app`'s `acts.rs` and `git.rs` keep their names. Where the rest of `atelier-app`'s files go is the implementer's choice, and the Report gives the layout in a few lines.
- [x] No file under `model/` names `std::fs`, `std::process` or a module of the crate outside `model`.
- [x] Every file is moved with its history (`git mv`), and `git diff -M` against the base shows each moved source file changing only its `use` and `mod` lines, paths in its comments, and visibility where the compiler requires it. The Report lists any file that changed more and says why.
- [x] `atelier-cli`, `atelier-desk` and every test compile against `atelier_core` alone. The tests of the three crates are in `crates/atelier-core/tests` with their fixtures. No test's inputs or assertions change: a test file's diff is the paths of crates and modules wherever they occur (`atelier_records::put_first` in a test's body, for one), the paths of fixtures that moved (`crates/atelier-app/tests/acts.rs:1253` joins `../atelier-records/tests/fixtures/open-missions`), and the list of crate names in `crates/atelier-desk/tests/dependencies.rs:8`, which loses the two crates that are gone.
- [x] `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass, with the same number of tests as on the base. The Report gives both counts.
- [x] `docs/design.md`'s crate table has three rows, every place it and `docs/architecture/desk.md` name `atelier-app` or `atelier-records` is corrected, and its Decisions hold the two rules of atelier-dy62's Outcome. Text that describes the old tool's crates, or what the rewrite copied from where, is history and stays.
- [x] Nothing outside `.atelier/` and those histories names `atelier-app`, `atelier_app`, `atelier-records` or `atelier_records`: `git grep` for them, and the Report lists what is left and why.

## Scope

In: `crates/`, `Cargo.toml`, `Cargo.lock`, `docs/design.md`, `docs/architecture/desk.md`, and any script, CI file or fixture that names a crate.

Out: renaming `acts.rs`, `Act` or the word "act" (atelier-nl43); moving the Desk's or `at_commit.rs`'s own `git` and `gh` spawns (atelier-x5al); moving anything out of `atelier-cli`; narrowing exports; splitting or merging files; any change in behaviour. Records under `.atelier/` that name the old crates are not edited here.

## Context

On 2026-11-14 there were 189 `use` sites of the three crates in 42 files. `atelier-core` and `atelier-app` each have a `review.rs`, which the `model/` directory keeps apart. `desk/e2e/fixtures.ts:158` names `atelier-app` in a made-up item's title and `scripts/scorecard/draw_samples.py:80` names `crates/atelier-records/src/document.rs` in a comment.

Two gardener branches are approved and unmerged, atelier-0019 and atelier-qb4f. atelier-qb4f edits six module headers under `crates/` and `docs/design.md`'s crate table. Whichever of these three merges later merges the base in first; do not rebase.

Build with a `CARGO_TARGET_DIR` of this worktree's own.

## Report

The three library crates are one, `atelier-core`, with no change in
behaviour. `cargo nextest run` ran 362 tests, 362 passed and 1 skipped, on
the base (b9f10590f) and on this branch; the 362 test names are the same,
some under a new module path (`model::review::tests::…` for
`review::tests::…`). `cargo fmt -- --check` and `cargo clippy
--all-targets -- -D warnings` pass. The workspace's members are
`atelier-core`, `atelier-cli` and `atelier-desk`, and `atelier-core`
depends on `anyhow`, `chrono`, `serde`, `serde_yaml` and `toml`, with
`tempfile` for its tests: the union of the three, with none added.

Layout of `crates/atelier-core/src`:

- `lib.rs` is `atelier-app`'s, with `pub mod model;` and `pub mod
  records;` added and a first paragraph saying what the crate is.
- `model/`: the old `atelier-core`; its `lib.rs` is `model/mod.rs`.
- `records/`: the old `atelier-records`; its `lib.rs` is `records/mod.rs`.
- Every other file of `atelier-app` (`acts.rs`, `git.rs`, `plan.rs`,
  `repo.rs`, `review.rs` and the rest) keeps its name at the root, so
  callers' paths change only in the crate's name: `atelier_app::X` is
  `atelier_core::X`, `atelier_records::X` is `atelier_core::records::X`,
  and the old `atelier_core::X` is `atelier_core::model::X`. Nothing is
  re-exported at the root that was not there before.

Inside the crate, `model` uses only `crate::model::…`, `records` uses
`crate::model` and `crate::records`, and the rest both. No file under
`model/` names `std::fs`, `std::process` or `crate::` outside
`crate::model`. Its doc comments in `item.rs`, `review.rs` and `mod.rs`
said "read by `atelier-records`" and now say `records`; that names the
module in prose only.

Reading the diff: `crates/atelier-core/src/lib.rs` and `review.rs` exist on
both sides with different contents, so plain `git diff -M` shows them as
rewritten and `model/mod.rs` and `model/review.rs` as new. `git diff -M -B
b9f10590f` pairs them: `atelier-app/src/lib.rs → atelier-core/src/lib.rs`,
`atelier-core/src/lib.rs → model/mod.rs`, and the same for `review.rs`.
Every move was `git mv`.

Moved files that changed beyond `use` and `mod` lines and comment paths,
and why:

- Fully qualified paths in code bodies, such as
  `atelier_core::MissionStatus::Dropped` in `repo.rs`, `say.rs`, `plan.rs`
  and `records/store.rs`, became `crate::model::…`. They are paths of
  crates and modules, written inline rather than in a `use`.
- `lib.rs`: the two `mod` lines and the first paragraph of its doc comment.
- No visibility changed: `pub(crate)` items of the old crates now reach the
  whole merged crate, and the compiler asked for nothing.

Outside the moved files: `rustfmt` broke one match arm in
`crates/atelier-cli/src/view_output.rs` over three lines because the
longer path made it too wide. `crates/atelier-core/tests/acts.rs` joins
`tests/fixtures/open-missions`, which `rustfmt` put on one line, and
`crates/atelier-desk/tests/dependencies.rs`'s `TRACKER` is `[&str; 2] =
["atelier-cli", "atelier-core"]`. No test's inputs or assertions changed.

Docs: `docs/design.md`'s crate table has three rows, with `atelier-desk`
added since it had none; a paragraph after it says what the three old
crates held, so the old names in records still read. Its Decisions end
with the two rules, dated 2026-11-14 for atelier-dy62. Every other mention
of `atelier-app` or `atelier-records` in it and in
`docs/architecture/desk.md` names `atelier-core`, and the old
`atelier_core::Mission` and its kind in `desk.md` became
`atelier_core::model::…`. `desk/e2e/fixtures.ts:158`'s made-up title says
`atelier-core`, and `scripts/scorecard/draw_samples.py:80` names
`crates/atelier-core/src/records/document.rs`. `pnpm e2e` was not run: that
title is shown in a page, and no test reads it.

What `git grep -E 'atelier[-_](app|records)' -- ':!.atelier'` still finds,
all history:

- The old tool's files that code was copied from:
  `crates/atelier-cli/src/forge.rs:12` (`crates/atelier-app/src/forge.rs`),
  `crates/atelier-cli/src/merge.rs:15`
  (`crates/atelier-records/src/mutation_lock.rs`),
  `crates/atelier-core/src/records/at_commit.rs:4`
  (`crates/atelier-app/src/git_freshness.rs`),
  `crates/atelier-core/src/records/document.rs:5`
  (`crates/atelier-records/src/lib.rs`) and
  `crates/atelier-core/src/records/ids.rs:1`
  (`crates/atelier-records/src/record_id.rs`). The old workspace's
  deletion, 819024c75, removed each of them.
- `docs/design.md:69` and `:1384`, the old tool's `forge.rs`, and
  `docs/architecture/desk.md:823`, the old tool's `mission_review.rs`.
- `docs/design.md:93-94`, the new paragraph that says what the old crates
  held.