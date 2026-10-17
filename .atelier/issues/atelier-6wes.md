---
created_at: "2026-10-17T07:12:11.979255862+00:00"
id: "atelier-6wes"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Reject or support review targets in evidence attach"
updated_at: "2026-10-17T07:12:11.979255862+00:00"
---

## Goal

`evidence attach <id> review <review-id>` passes kind validation, then fails with "evidence record ... not found", because `validate_record_ref` treats any target that is not an issue as evidence. Found while implementing atelier-qrwr.

## Done when

- `evidence attach` either attaches to a review target or refuses it up front, with a message naming the kinds it accepts.
- A CLI test covers a review target.
