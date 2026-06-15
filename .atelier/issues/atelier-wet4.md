---
created_at: "2026-06-15T03:54:39.162335094+00:00"
id: "atelier-wet4"
issue_type: "task"
labels:
- "cli"
- "docs"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t35w"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update status and role guidance for current-work sets"
updated_at: "2026-06-15T03:54:39.162335094+00:00"
---

## Description

Update human-facing status and role guidance after the active issue feature is removed. The default operator view should make the current work set legible without implying a hidden single active pointer.

## Outcome

- `atelier status` shows zero, one, or multiple in_progress issues with clear hierarchy and next actions.
- Role guides from `atelier man worker|reviewer|manager|admin` no longer mention local active-work associations, claim, root repair, or abandon as pointer cleanup.
- Help/common command lists route workers toward status, issue transitions, notes, evidence, and workflow status rather than cache repair.
- Multiple in_progress issues are treated as explainable status, not a hard invariant violation.

## Evidence

- CLI transcript or tests cover no in_progress issues, one in_progress issue, and multiple in_progress issues.
- Help transcript proves removed commands/guidance are absent.
- Role-guide transcript proves current-work guidance is status-derived and role-scoped.
- `atelier lint atelier-wet4`, `atelier export --check`, and focused CLI integration tests pass.
