---
created_at: "2026-06-19T03:59:00.565873153+00:00"
id: "atelier-swxv"
issue_type: "validation"
labels:
- "review"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate provider mode parity and wrong-mode rejection"
updated_at: "2026-06-19T03:59:00.565873153+00:00"
---

## Description

Validate provider-mode behavior and room/provider command boundaries after the
rename to `atelier review`.

## Outcome

- Forgejo provider mode preserves open, link, status, show, comments, comment,
  approve, request-changes, and merge behavior through `atelier review`.
- Provider mode writes and reads `review.kind: pull_request` links.
- Provider-only commands reject in room mode and room-only commands reject in
  provider mode with direct corrective text.
- `atelier pr ...` rejection is covered separately from wrong-mode rejection.

## Evidence

- Forgejo parity test output or transcript compares provider behavior through
  `atelier review`.
- Negative command output covers room-only and provider-only wrong-mode cases.
- Validation evidence links exact command output, fixture records, and residual
  risk classification.
