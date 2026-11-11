---
schema: atelier.item
schema_version: 3
id: atelier-3i7x
title: Compute in-flight marks without snapshotting each branch's tracker
status: draft
mission: null
labels:
- orientation
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T03:12:59.504595810Z
---

## Goal

`work ready`, `work all` and `status` read every local work branch through `Snapshot::at` (`git archive | tar` of the whole `.atelier` tree, load, policy validation, review scope). Measured in review of atelier-7n5u: about 80-100 ms per owned branch on a release build (0.75 s for `work ready` with 7 branches vs 0.04 s for `work blocked`), so cost grows with branches times tracker size. The in-flight marks need only the owner record blob and `done_merged`.

## Done when

- [ ] In-flight marks come from the owner record blob without a full snapshot, and the snapshot-only facts (such as the active count in the Other local work branches section) are computed only when that section is shown. `work ready` with 7 owned branches on a release build stays within 2x of `work blocked`, shown by a timing recorded as evidence, and existing tests pass.

## Context

`crates/atelier-cli/src/commands/git.rs` `branch_summaries` (after atelier-7n5u). SessionStart runs `status`, so this also bounds hook latency.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-iigf
