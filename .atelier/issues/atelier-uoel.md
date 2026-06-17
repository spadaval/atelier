---
created_at: "2026-06-17T17:59:49.363591102+00:00"
id: "atelier-uoel"
issue_type: "task"
labels:
- "docs"
- "product"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update product docs for sessions, PR commands, fields, sudo identity, and validators"
updated_at: "2026-06-17T17:59:49.363591102+00:00"
---

## Description

Update product and architecture documentation for session lifecycle, PR command
purpose, typed field configuration, Forgejo sudo identity, and validator
boundaries.

## Outcome

- Product docs describe `atelier session` and `atelier pr` surfaces without
  reviving removed legacy session/current-work semantics.
- Workflow configuration docs describe schema version 2 typed fields.
- Validation docs describe `linked_pr_merged` as a read-only external-state
  gate.

## Evidence

- Manual check of updated product/architecture doc file content names the
  session, PR, field, sudo, and validator guidance.
- Command transcript shows targeted docs/help parity checks and `git diff
  --check -- docs CONTEXT.md` pass.
