---
schema: atelier.item
schema_version: 3
id: atelier-jz8q
title: Move /api/history and /api/launch to atelier_core::log when t3-extension reopens
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T07:51:21.116569562Z
---

## Goal

Serve the Desk's whole-repository history from the records, as the mission
page's history already is. `atelier-js4a` gave `GET
/api/repos/{repo}/missions/{id}/history` exact history from
`atelier_core::log`, and left `/api/history` and `/api/launch` on the parser
of commit subjects in `crates/atelier-desk/src/history.rs`, because
`t3-extension/` was frozen and its fixture builds a closed mission from
empty commits. This item waits until Sumeet reopens `t3-extension/`.

## Done when

- [ ] `t3-extension/fixture-desk.mjs` builds its history with commits that change records, not `--allow-empty` commits.
- [ ] `/api/history` and `/api/launch` read `atelier_core::log`, and the commit-subject parser in `crates/atelier-desk/src/history.rs` is deleted.
- [ ] History from before the cutover, which has no version 3 records, shows what the item decides and the Desk's docs say.
- [ ] The "In CI" passage of `t3-extension/README.md` names what `atelier-1n49` changed: the `extension` job also runs on a change to `scripts/t3-extension-release*`, and it runs `scripts/t3-extension-release.test.sh`.
- [ ] `desk/` checks (`pnpm typecheck && pnpm test && pnpm build && pnpm e2e`) and `node t3-extension/run-tests.mjs` pass.

## Scope

In: `crates/atelier-desk`, `desk/src/api/`, `t3-extension/`, `docs/architecture/desk.md`, `docs/product/desk.md`.

Out: starting before Sumeet reopens `t3-extension/`, which he froze on 2026-11-22.
