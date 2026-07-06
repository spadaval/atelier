---
created_at: "2026-07-06T17:20:25.689756160+00:00"
id: "atelier-yf06"
issue_type: "feature"
labels:
- "cli"
- "inventory"
- "rendering"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kiyq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Implement a flat issue inventory read and render path"
updated_at: "2026-07-06T17:20:25.689756160+00:00"
---

## Description

Introduce the issue-list-specific read model and renderer. Remove epic headers, nested child rows, subtree blocker context, mission grouping, and queue footer actions from `issue list`. Keep one deterministic row per matching issue with the metadata and empty-state behavior defined by the approved contract.

## Outcome

- Representative parent, child, mission, epic, standalone, blocked, and done records render as independent flat rows in `atelier issue list`, with no inferred group headings or indentation.
- Inventory rendering has a focused interface rather than reusing the legacy grouped operational queue pipeline.

## Evidence

Evidence was not specified in the bundle.
