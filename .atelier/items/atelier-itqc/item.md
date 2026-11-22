---
schema: atelier.item
schema_version: 3
id: atelier-itqc
title: A backlog merge past a rule prints merge for every record it changed
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-22T07:51:21.354391426Z
---

## Goal

A backlog branch merged past a rule (`ATELIER_SKIP`) prints one `merge` line
in `atelier log` for every record it changed (`Happened::Passed`). The
gardener counts merged items with `$4 == "merge"`
(`skills/agent-factory/procedures/garden.md`), so each of those records
counts as a merged item. `atelier-js4a` gave a plan's merge its own name,
`plan-merge`; a record-only merge still shares `merge`. Found by
`atelier-js4a`'s implementer.

## Done when

- [ ] A record-only merge past a rule prints an action other than `merge` (such as `records-merge`), once for the merge and not once for each record, or otherwise cannot be counted as an item's merge.
- [ ] A test in `crates/atelier-cli/tests/log.rs` proves it, and `docs/design.md` ("The log") names the action.
- [ ] `garden.md`'s counter gives the same number of merged items on this repository before and after, less those lines.

## Scope

In: `crates/atelier-core/src/log.rs`, `crates/atelier-cli/src/history.rs`, `docs/design.md`, and `skills/agent-factory/procedures/garden.md` only if its counter must change.
