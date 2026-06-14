---
created_at: "2026-06-14T02:51:56.372163784+00:00"
id: "atelier-0vjh"
issue_type: "task"
labels:
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T06:46:18.899245217+00:00"
status: "done"
title: "Update architecture source-layout map"
updated_at: "2026-06-14T06:46:18.899245217+00:00"
---

## Description

Refresh architecture docs so scouts know the current single-crate source layout and where command dispatch, projection, RecordStore, workflow policy, evidence, and worktree behavior live.

## Outcome

Agents no longer need to probe nonexistent crates/ paths or stale command module names to orient in the codebase.

## Evidence

Architecture index or linked architecture page names the current files/modules; stale path examples are absent; git diff --check passes.
