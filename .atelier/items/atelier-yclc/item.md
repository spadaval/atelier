---
schema: atelier.item
schema_version: 3
id: atelier-yclc
title: Move submit and merge into the core, so the Desk can carry an open mission's stop through to the base
status: draft
mission: atelier-to83
labels: []
priority: medium
blocked_by:
- atelier-v958
- atelier-ni3d
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-14T01:33:18.787068585Z
---

## Goal

Let the Desk publish the stop of an open mission without running the
`atelier` binary. A stop of an open mission reaches the base as `drop` on a
`backlog/` branch, then `submit` and `merge`. `submit` and `merge` live in
`atelier-cli` (`merge.rs` and `forge.rs`, about 1,700 lines), and the Desk
acts only through the core (ADR 0032). The implementer of atelier-v958
raised this on 2026-11-13, and its lead moved the work here instead of
granting an exception to the ADR. Until this merges, the Desk stops a
proposed plan itself and, for an open mission, shows the commands to run.

## Done when

- [ ] `submit` and `merge` are functions of `atelier-app` that the command calls, with the forge code they need. The command's behaviour, output and exit statuses do not change, and its tests pass unchanged.
- [ ] The Desk's "stop" on an open mission writes the drop on a `backlog/` branch in a checkout of its own, commits it, and calls the core's submit and merge. It runs no `atelier` binary.
- [ ] The mission's page shows the stop as pending until the branch has merged, and then as stopped. When merge returns "not finished yet", the Desk tries again on its fetch timer and the page says what it waits on.
- [ ] The interface test covers an open mission's stop from the click to the base.
- [ ] `docs/design.md` and `docs/architecture/desk.md` say where submit and merge live and how the Desk publishes a stop.

## Scope

In: `crates/atelier-app`, `crates/atelier-cli`, `crates/atelier-desk`, `desk/`, and the two documents named.

Out: any change to what `merge` checks or refuses; moving `new`, `set` or `close`, which gardener run atelier-qb4f proposes and Sumeet has not agreed.

## Context

atelier-v958 moves a mission's `drop` into `atelier-app` and builds the stop of a proposed plan. Its Report says what it left for this item.
