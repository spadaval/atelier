---
schema: atelier.item
schema_version: 3
id: atelier-js4a
title: Back Desk history and mission history API with atelier_core::log::history
status: ready
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.858188959Z
---

## Goal

Replace the Desk's approximate commit-subject history parser with `atelier_core::log::history` and implement the missing mission history endpoint (`GET /api/repos/{repo}/missions/{id}/history`, M5).

Today `crates/atelier-desk/src/history.rs` runs its own `git log` and prefix-parses commit subjects, so any step committed under a non-standard subject is silently missed and `/api/repos/{repo}/missions/{id}/history` returns 404. Meanwhile `atelier_core::log::history` already derives exact lifecycle actions from record diffs across both parents of merges (`--full-history`).

## Done when

- [ ] `crates/atelier-desk/src/history.rs` derives `HistoryEvent` values from `atelier_core::log::history` (via `Repo::open_committed`), removing the commit-subject prefix parser (`parse()` and its custom `git log --format=%H%x09%cI%x09%s` command).
- [ ] `GET /api/history?repo=&days=&ids=` and `GET /api/launch?repo=` keep their existing JSON response contract intact (events sorted newest-first, compatible `EventKind` variants, `round`/`verdict` populated for reviews, `record`/`title` populated as before) so `desk/` and `t3-extension/` continue working with zero edits to `t3-extension/`.
- [ ] `GET /api/repos/{repo}/missions/{id}/history` is implemented in `crates/atelier-desk/src/server.rs`, returning the mission's and its items' actions from `atelier_core::log::history` (404 for unknown repo or unknown mission ID, 400 for invalid ID).
- [ ] History reads remain cached (keyed by repo ref tips / existing cache strategy) and bounded in execution time; include a sanity timing note in the Report on this repository so regressions are visible.
- [ ] Generated TypeScript types in `desk/src/api/` are up to date (`cargo nextest run -p atelier-desk --test types` passes).
- [ ] `docs/architecture/desk.md` and `docs/product/desk.md` are updated to remove the "approximate commit-subject parsing" caveats and mark `/api/repos/{repo}/missions/{id}/history` (M5) as implemented.
- [ ] `cargo nextest run -p atelier-desk --locked` and `desk/` checks (`pnpm typecheck && pnpm test && pnpm build`) pass, and `t3-extension/` has zero file changes.

## Scope

In: `crates/atelier-desk/src/{history.rs,server.rs,server/views.rs,api.rs,read.rs}`, `crates/atelier-desk/tests/`, `desk/src/api/` (generated types only if changed), `docs/architecture/desk.md`, and `docs/product/desk.md`.

Out: `t3-extension/`, changing `atelier-core`'s `log` CLI output format, and Desk frontend redesigns beyond keeping existing `desk/` tests passing.
