---
created_at: "2026-11-07T14:11:42.911338096+00:00"
id: "atelier-hfme"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-nn67"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:52.529250545+00:00"
status: "declined"
title: "Stop writing branch_base and drop start's hidden --base once every Atelier CI pin reads records without it"
updated_at: "2026-11-11T03:55:52.529250545+00:00"
---

## Goal

atelier-md0v kept writing `branch_base` into ownership records (set to the policy base, ignored when read) and kept `start --base` as a hidden, ignored flag, so older pinned binaries and older guidance keep working. atelier-nn67 makes `branch_base` optional on read. Once every repository's CI pin is at or past that commit, remove both shims.

## Done when

- [ ] Every Atelier CI pin on this host (today only astromech's `ATELIER_REV`) is at or past the commit where `branch_base` became optional on read.
- [ ] New ownership records no longer write `branch_base`, and `start` no longer accepts `--base`; tests updated.
- [ ] ADR 0028 and docs/product say both are gone.

## Context

Pick this up when astromech next moves its pin past atelier-nn67's merge. docs/adr/0028-simpler-lifecycle.md decision 5; crates/atelier-core/src/lib.rs (`WorkflowBranch`).

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new records hold no branch field: an item's branch is named for its ID (product specification, Records), and the new `start` has no `--base`.
