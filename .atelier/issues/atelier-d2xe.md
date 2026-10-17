---
created_at: "2026-10-17T01:10:57.983690051+00:00"
id: "atelier-d2xe"
issue_type: "bug"
labels:
- "cli-output"
- "status"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show every scope item and plain review words in the session owner block"
updated_at: "2026-10-17T01:10:57.983690051+00:00"
---

## Goal

`prime` builds the owner block that atelier-sdb0 moves into status (crates/atelier-cli/src/commands/session.rs:118-180). It has these rendering problems:
- It prints `Work: atelier-7b6e [in_progress] Batch the per-review history walks… — blockers: ` with an empty trailing label on every unblocked row.
- It lists only active and blocked scope items, so done children that the owner is judged on are hidden. In a test, done child atelier-jb2t failed land and did not appear.
- Reviews and evidence use Rust `Debug` formatting, `Review: atelier-36rk for atelier-43m1 — TrivialExemption: <full multi-sentence summary>` (`{:?}` at session.rs:136) and `Evidence: … recorded {:?}` (session.rs:170).

## Done when

In the status owner block:
- Every scope item appears, one line each with its exact status.
- `blocked by X` appears only when X exists.
- Review events and evidence results use CLI words (`trivial exemption`, `approval`, `passed`).
- Each review summary is cut to one line.

Tests assert no `{:?}`-style variant names and no empty `blockers:` suffix.

## Context

This extends atelier-sdb0, which moves this content into status.
