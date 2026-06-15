---
created_at: "2026-06-15T05:13:27.586244262+00:00"
id: "atelier-4ra1"
issue_type: "task"
labels:
- "architecture"
- "cleanup"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T15:33:50.786963228+00:00"
status: "done"
title: "Define tracked migration adapter policy"
updated_at: "2026-06-15T15:33:50.786963228+00:00"
---

## Description

Define how temporary internal migration adapters are allowed during crate extraction so implementation can make progress without creating permanent compatibility layers.

## Outcome

- Architecture docs define what qualifies as a temporary internal adapter during the rewrite.
- Each adapter must have an owning issue, a removal condition, and no public CLI or Rust API compatibility promise.
- Closeout criteria require adapter inventory review before the mission can close.

## Evidence

- File change review of architecture docs names the adapter policy and removal expectation.
- Issue Markdown review for implementation epics references the policy.
- Search transcript or closeout inventory command can identify remaining adapter markers.
