---
created_at: "2026-06-23T16:21:20.084264062+00:00"
id: "atelier-qqfe"
issue_type: "validation"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Validate command behavior after cache rewrite"
updated_at: "2026-07-06T21:01:42.016723483+00:00"
---

## Description

Run command-level validation across issue list/show/status, blockers, workflow status, evidence show/list, review flows, lint, and cache rebuild/repair paths.

## Outcome

- Evidence records the command outputs that prove the rewritten storage/cache model works across common workflows.

## Evidence

- Independent scenario evidence `atelier-mfxj` and transcript
  `/tmp/atelier-mska-qqfe-transcript.txt` cover issue list/show/status,
  ready/blocked and transition views, evidence capture and attachment, native
  review-room flows, check/lint, explicit rebuild, doctor repair, and lazy
  cache repair after a record-file write.
- Test evidence `atelier-oy2z` records all 448 Atelier CLI tests passing; the
  focused `atelier-sqlite` suite passed 8/8. The CLI inventory contains no
  ignored tests, and current help uses domain-cache/runtime-cache terminology.
