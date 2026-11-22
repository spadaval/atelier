---
schema: atelier.item
schema_version: 3
id: atelier-js4a
title: Add mission-plan actions to atelier_core::log and implement Desk mission history (M5)
status: building
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

- [ ] `crates/atelier-core/src/log.rs` emits mission-plan lifecycle actions:
  - `Happened::Plan` (printed as `plan`) when a mission record first appears as a new plan (`status: open` without a pre-existing `lead`/`Result` and whose `created_at` does not predate the commit by more than 24 hours, so bulk-converted pre-cutover missions such as `atelier-g6rp` and `atelier-qr5s` in `7f499285e` are not falsely dated to the conversion commit), attributed to its first planner (or `-` if none).
  - `Happened::PlanMerge` (printed as `plan-merge` — distinct from `merge` so `garden.md`'s `$4 == "merge"` item counter never miscounts merged plans as items) when a first-parent merge onto `base` brings a mission record onto `base` as open/agreed (or merges a plan amendment).
  - `crates/atelier-cli/src/history.rs`, `crates/atelier-cli/tests/log.rs`, and `docs/design.md` ("The log") format, test (including proving `plan-merge` is printed rather than `merge`), and document both actions (noting that pre-cutover records have no v3 record history before conversion).
- [ ] `atelier_core::log` (or Desk's wrapper calling it via `Repo::open_committed`) enforces a bounded execution deadline (15s) and caches mission history by repo ref tips.
- [ ] `GET /api/repos/{repo}/missions/{id}/history` is implemented in `crates/atelier-desk/src/server.rs`, returning the mission's and its items' actions from `atelier_core::log::history` (404 for unknown repo or unknown mission ID, 400 for invalid ID), with typed response structs in `crates/atelier-desk/src/api.rs` (including timestamp, commit, record id, action, thread, detail, and skips) and regenerated TypeScript types in `desk/src/api/` (`cargo nextest run -p atelier-desk --test types` passes).
- [ ] `GET /api/history` and `GET /api/launch` keep their existing commit-subject reader and JSON contract unchanged so `t3-extension/fixture-desk.mjs` and pre-cutover history continue working with zero edits under `t3-extension/`. File a follow-up item on a backlog branch (or note in Report for the lead to file) to migrate `/api/history` and `/api/launch` once `t3-extension/` is next opened.
- [ ] `docs/architecture/desk.md` and `docs/product/desk.md` document `GET /api/repos/{repo}/missions/{id}/history` (M5) as implemented and note why `/api/history` stays on commit subjects until `t3-extension/fixture-desk.mjs` is updated.
- [ ] `cargo nextest run -p atelier-core -p atelier-cli -p atelier-desk --locked`, `desk/` checks (`pnpm typecheck && pnpm test && pnpm build && pnpm e2e`), and `node t3-extension/run-tests.mjs` pass, with zero file changes in `t3-extension/`.

## Scope

In: `crates/atelier-core/src/log.rs`, `crates/atelier-cli/src/history.rs`, `crates/atelier-cli/tests/log.rs`, `crates/atelier-desk/src/{server.rs,server/views.rs,api.rs,read.rs}`, `crates/atelier-desk/tests/`, `desk/src/api/`, `docs/design.md`, `docs/architecture/desk.md`, and `docs/product/desk.md`.

Out: `t3-extension/` (including `t3-extension/fixture-desk.mjs`), replacing `crates/atelier-desk/src/history.rs`'s commit-subject parser for `/api/history` and `/api/launch`, and Desk frontend redesigns beyond keeping existing `desk/` tests passing.
