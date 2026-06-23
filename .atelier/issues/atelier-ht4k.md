---
created_at: "2026-06-23T20:17:28.792727344+00:00"
id: "atelier-ht4k"
issue_type: "task"
labels:
- "cli"
- "records"
- "workflow"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ih42"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Define context-only custom issue links"
updated_at: "2026-06-23T20:17:28.792727344+00:00"
---

## Description

Define and implement the narrow custom-link contract. Custom issue link types may be stored, displayed, searched, and preserved as typed relationship context, but they must not affect mission progress, readiness, blockers, branch selection, review ownership, or transition validation unless promoted to a built-in semantic.

## Outcome

Docs and validation distinguish built-in workflow-driving links from custom context links, and issue link/unlink accepts configured custom link types with display-only behavior.
