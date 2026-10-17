---
created_at: "2026-10-17T07:17:24.592249693+00:00"
id: "atelier-lr8o"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-lr8o
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Resolve short evidence IDs in bundle files like command arguments"
updated_at: "2026-10-17T08:57:28.424041999+00:00"
---

## Goal

Bundle files resolve existing issue IDs through the shared short, case-insensitive matcher, but evidence IDs only by exact full ID, and an unknown one gives the old "Record id 'X' not found" wording (`crates/atelier-cli/src/commands/bundle.rs`, about lines 1320-1333). A bundle naming evidence `052d` fails although `evidence show 052d` works. Already true before atelier-qrwr. Found in its review (finding atelier-sklz).

## Done when

- Bundle references to existing evidence and reviews resolve like command arguments, and an unknown one uses the shared not-found message.
- A bundle test covers a short evidence ID and an unknown one.
