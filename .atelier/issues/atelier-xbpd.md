---
created_at: "2026-06-29T17:47:42.680885710+00:00"
id: "atelier-xbpd"
issue_type: "task"
labels:
- "docs"
- "evidence"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:20:34.990191587+00:00"
status: "done"
title: "Record validation results as pass-fail receipts"
updated_at: "2026-06-30T15:20:34.990191587+00:00"
---

## Description

Validation guidance and surfaces treat evidence records as receipts from checks that actually ran. Validator output records the claim checked, action taken, result classification, and artifact or transcript, without requiring planners to prewrite those checks.

## Outcome

Validation records are small receipts from real checks. They identify the claim checked, what the validator did, the result (`pass`, `fail`, `blocked`, or `deferred`), and the transcript/artifact or issue link that lets another operator inspect the result.
