---
created_at: "2026-06-13T20:35:28.489106637+00:00"
id: "atelier-foy0"
issue_type: "epic"
labels:
- "closeout"
- "stabilization"
- "validation"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3iom"
  - kind: "issue"
    id: "atelier-bk6n"
  - kind: "issue"
    id: "atelier-ngat"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Validate stabilization closeout end to end"
updated_at: "2026-06-13T20:55:09.103395868+00:00"
---

## Description

Close the stabilization mission only after the command surface, record formats, and architecture cleanup claims are proven by focused transcripts, tests, and an independent closeout audit.

## Outcome

- Each mission outcome maps to linked child work and attached evidence.
- Validation distinguishes passed, failed, deferred, and not-applicable findings without relying on broad green tests alone.
- Any deferred simplification has a concrete owner issue and does not block the mission silently.

## Evidence

- Mission audit transcript maps validation lines to linked work and evidence IDs.
- Final health proof records atelier lint, atelier export --check, atelier doctor, focused CLI tests, cargo fmt -- --check, and git diff --check.
- Independent validation issue records residual risks and follow-up IDs.
