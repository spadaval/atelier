---
created_at: "2026-06-18T16:20:32.323131205+00:00"
id: "atelier-7ssp"
issue_type: "task"
labels:
- "docs"
- "session-pr-overhaul"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-5k7k"
  - kind: "issue"
    id: "atelier-ff55"
  - kind: "issue"
    id: "atelier-hzo7"
  - kind: "issue"
    id: "atelier-jdvz"
  - kind: "issue"
    id: "atelier-lvgo"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Document session-as-issue-events attribution model"
updated_at: "2026-06-18T16:49:25.845977046+00:00"
---

## Description

Update the product/domain contract for the minimal session and PR attribution model before dependent implementation proceeds. The docs and ADRs must explicitly replace the older durable optional session model with session-as-issue-events: sessions are derived issue-scoped worker/reviewer/validator attempts from canonical issue activity, PRs link to issues or epics, and session commands are inspection surfaces rather than workflow drivers.

## Outcome

CONTEXT.md, CLI surface documentation, validation guidance, and the current session/PR ADR describe the new model consistently. Public guidance removes `session begin/end`, mutating session kinds, admin sessions, and coordination sessions from the happy path. docs/product/zen.md is updated only if the minimality principle needs an explicit session/PR note.

## Evidence

Documentation diff and `atelier lint` output show the session-as-issue-events contract is consistent and accepted before dependent CLI or PR implementation starts.
