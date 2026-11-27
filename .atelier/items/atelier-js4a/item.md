---
schema: atelier.item
schema_version: 3
id: atelier-js4a
title: Add mission-plan actions to atelier_core::log and implement Desk mission history (M5)
status: approved
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Aa7ab9ade-d27f-4fdc-9a10-37446bba3ca2%3Adelegate-task%3A25d2a75f-4f1b-49bd-9c19-fc6374735508
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.858188959Z
---

## Goal

Add mission-plan lifecycle actions to `atelier_core::log` and implement the Desk's missing mission history endpoint (`GET /api/repos/{repo}/missions/{id}/history`, M5) backed by `atelier_core::log::history`.

Today `GET /api/repos/{repo}/missions/{id}/history` returns 404, and `atelier_core::log::Happened` does not record when a mission was planned or when its plan merged onto `base`. Meanwhile, `GET /api/history` and `GET /api/launch` are consumed by `t3-extension/` (whose fixture `t3-extension/fixture-desk.mjs` builds its closed mission with `--allow-empty` commits) and cover pre-cutover history that has no v3 record diffs. Since `t3-extension/` is frozen in this mission, this item delivers exact core-backed history on M5 and prepares `atelier_core::log` without breaking `/api/history`, `/api/launch`, or `t3-extension/`.

## Done when

- [x] `crates/atelier-core/src/log.rs` emits mission-plan lifecycle actions:
  - `Happened::Plan` (printed as `plan`) when a mission record first appears as a new plan (`status: open` without a pre-existing `lead`/`Result` and whose `created_at` does not predate the commit by more than 24 hours, so bulk-converted pre-cutover missions such as `atelier-g6rp` and `atelier-qr5s` in `7f499285e` are not falsely dated to the conversion commit), attributed to its first planner (or `-` if none).
  - `Happened::PlanMerge` (printed as `plan-merge` — distinct from `merge` so `garden.md`'s `$4 == "merge"` item counter never miscounts merged plans as items) when a first-parent merge onto `base` brings a mission record onto `base` as open/agreed (or merges a plan amendment).
  - `crates/atelier-cli/src/history.rs`, `crates/atelier-cli/tests/log.rs`, and `docs/design.md` ("The log") format, test (including proving `plan-merge` is printed rather than `merge`), and document both actions (noting that pre-cutover records have no v3 record history before conversion).
- [x] `atelier_core::log` (or Desk's wrapper calling it via `Repo::open_committed`) enforces a bounded execution deadline (15s) and caches mission history by repo ref tips.
- [x] `GET /api/repos/{repo}/missions/{id}/history` is implemented in `crates/atelier-desk/src/server.rs`, returning the mission's and its items' actions from `atelier_core::log::history` (404 for unknown repo or unknown mission ID, 400 for invalid ID), with typed response structs in `crates/atelier-desk/src/api.rs` (including timestamp, commit, record id, action, thread, detail, and skips) and regenerated TypeScript types in `desk/src/api/` (`cargo nextest run -p atelier-desk --test types` passes).
- [x] `GET /api/history` and `GET /api/launch` keep their existing commit-subject reader and JSON contract unchanged so `t3-extension/fixture-desk.mjs` and pre-cutover history continue working with zero edits under `t3-extension/`. File a follow-up item on a backlog branch (or note in Report for the lead to file) to migrate `/api/history` and `/api/launch` once `t3-extension/` is next opened.
- [x] `docs/architecture/desk.md` and `docs/product/desk.md` document `GET /api/repos/{repo}/missions/{id}/history` (M5) as implemented and note why `/api/history` stays on commit subjects until `t3-extension/fixture-desk.mjs` is updated.
- [x] `cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk --locked`, `desk/` checks (`pnpm typecheck && pnpm test && pnpm build && pnpm e2e`), and `node t3-extension/run-tests.mjs` pass, with zero file changes in `t3-extension/`.

## Scope

In: `crates/atelier-core/src/log.rs`, `crates/atelier-cli/src/history.rs`, `crates/atelier-cli/tests/log.rs`, `crates/atelier-desk/src/{server.rs,server/views.rs,api.rs,read.rs}`, `crates/atelier-desk/tests/`, `desk/src/api/`, `docs/design.md`, `docs/architecture/desk.md`, and `docs/product/desk.md`.

Out: `t3-extension/` (including `t3-extension/fixture-desk.mjs`), replacing `crates/atelier-desk/src/history.rs`'s commit-subject parser for `/api/history` and `/api/launch`, and Desk frontend redesigns beyond keeping existing `desk/` tests passing.

## Report

A mission's timeline can now be read exactly from its records: `atelier log`
prints when a plan was written (`plan`) and when it merged (`plan-merge`), and
the Desk serves both, with the items' actions, at
`GET /api/repos/{repo}/missions/{id}/history`.

### What changed

One commit for each purpose:

- **074b2ee09, the log.** `plan` is a mission record's first appearance as a
  new open plan, with the 24-hour, lead and Result rules. `plan-merge` is a
  first-parent merge on the base that agrees a plan, or amends one
  (`amendment`) as `merge` judges a plan change; both use the same two helpers
  in `plan.rs`. Finding those merges takes one more `git log`, because the
  walk lists no files for a merge and drops a first parent with no records.
  The wording of what changed moved from the CLI to the core, for the Desk.
- **b18fed5ab, the deadline.** `history_until` kills its Git listings at the
  deadline and reads no further commit. `atelier log` gives none.
- **ff5530ce2, the route.** It caches by the repository's ref tips. Its 15
  seconds are enforced twice, for different failures: the core stops the
  work, and the Desk stops waiting, since a blocked `git cat-file` read
  cannot be interrupted from the walk.

One visible change to old output: a plan merged past a rule used to print
`merge` and now prints `plan-merge`, in `log` and under `show`'s Rules passed
(atelier-kw8u's merge in 4a94d05c5 is the one such line here).

Three files outside Scope's list changed because the work needed them:
`plan.rs` (the shared plan-change helpers), `git.rs` with `forge.rs` (one
wait-until-deadline, moved, not copied) and `bounded.rs` in the Desk.

### Validation

- `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D warnings`,
  `git diff --check` and `atelier check` pass.
- `cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk --locked`:
  409 passed, the `types` test among them.
- `desk/`: `pnpm typecheck`, `pnpm test` (57), `pnpm build` and `pnpm e2e`
  (134) pass.
- `evals/agent-factory/test.sh target/debug/atelier` passes.
- `t3-extension/` has no change. **`node t3-extension/run-tests.mjs` did not
  pass whole on this host:** 5 of 7 passed, and `test-extension` and
  `test-update` failed because `fs.watch` got ENOSPC. `zed-remote-server`
  holds 122,431 of the host's 122,567 inotify watches. With `fs.watch`
  replaced by polling in the test process alone (a preload outside the
  repository), both pass. That hides only whether the file watcher works,
  which this change does not touch. **CI's `extension` job ran them
  unmodified on 7f71563 and passed**
  (https://github.com/spadaval/atelier/actions/runs/44998265862/job/137133979345).
- Against this repository with the built Desk, before submit: atelier-ww7w's
  34 actions then (it gains one with each step of its items) equal
  `atelier log atelier-ww7w`, in 1.0 seconds and 0.05 cached. atelier-g6rp and
  atelier-qr5s get no `plan`. atelier-osel gets its plan, merge and amendment.
  An item's ID is 404 and a bad one 400.
- With an old blob replaced by a FIFO, so that `git cat-file` hangs, the route
  answered 500 after 15.00 seconds.

### After round 1

Round 1 approved, with two deferred findings, which are bugs 2 and 3 below.
Since then the branch changed only by `git merge origin/master`, which
brought in atelier-1n49 (CI's filters and clippy) with no conflict, and by
this Report. `merge` asks for a fresh review because the tree changed. On
the merged tree clippy passes and the three crates' 409 tests pass.

### For the lead to file

1. **Follow-up this item's Done when names:** "Move `/api/history` and
   `/api/launch` to `atelier_core::log` once `t3-extension/` reopens". It
   updates `t3-extension/fixture-desk.mjs` to make record commits in place of
   `--allow-empty` ones, migrates both routes, decides what shows for history
   from before the cutover (which has no version 3 records), and then deletes
   `crates/atelier-desk/src/history.rs`'s subject parser.
2. **Bug, found here and not fixed:** `atelier log` loses an item's `merge`
   line when the base held no record before that merge, as in a new
   repository whose first item is `start --new`. `--parents` rewrites that
   merge to one parent and `Walker::commit` reads it as an ordinary commit.
   Reproduce: a repository with only `config.toml`, `start --new`, `merge
   --trivial`, `log`. `plan-merge` is not affected.
3. **Bug, found here and not fixed:** a backlog branch merged past a rule
   prints `merge` for every record it changed (`Happened::Passed`), so
   `garden.md`'s `$4 == "merge"` counts those records as merged items. A
   plan's merge no longer does. A name of its own, such as `records-merge`,
   would fix the rest.
