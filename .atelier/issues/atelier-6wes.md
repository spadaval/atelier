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
closed_at: "2026-10-23T14:13:50.448046359+00:00"
status: "superseded"
title: "Reject or support review targets in evidence attach"
updated_at: "2026-10-23T14:13:50.448046359+00:00"
---

## Goal

`evidence attach <id> review <review-id>` passes kind validation, then fails with "evidence record ... not found", because `validate_record_ref` treats any target that is not an issue as evidence. Found while implementing atelier-qrwr.

## Done when

- `evidence attach` either attaches to a review target or refuses it up front, with a message naming the kinds it accepts.
- A CLI test covers a review target.

## Close Reason

Done on master before rc8j started: since 1a9d6b9c, evidence attach limits target kinds to issue and evidence and rejects review with the shared invalid-value error, pinned by tests/cli_integration/record_refs.rs:502 (rc8j regroom 2026-10-23).
