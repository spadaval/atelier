---
created_at: "2026-06-11T20:10:52.442449746+00:00"
id: "atelier-3es3"
issue_type: "task"
labels:
- "refactor"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-kxko"
  - kind: "issue"
    id: "atelier-pgkd"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-i3no"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Extract canonical and runtime path resolution into a storage layout module"
updated_at: "2026-06-11T23:22:24.975963037+00:00"
---

## Description

Centralize canonical .atelier record paths, runtime/cache paths, legacy .atelier-state discovery, state.db location, diagnostics paths, identity paths, and ignore policy helpers. Acceptance: command code no longer hard-codes .atelier-state or .atelier/state.db outside the layout boundary.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
