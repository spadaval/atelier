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
  - kind: "review"
    id: "atelier-sklz"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T09:02:22.401806546+00:00"
status: "done"
title: "Resolve short evidence IDs in bundle files like command arguments"
updated_at: "2026-10-17T09:02:22.401806546+00:00"
---

## Goal

Bundle files resolve existing issue IDs through the shared short, case-insensitive matcher, but evidence IDs only by exact full ID, and an unknown one gives the old "Record id 'X' not found" wording (`crates/atelier-cli/src/commands/bundle.rs`, about lines 1320-1333). A bundle naming evidence `052d` fails although `evidence show 052d` works. Already true before atelier-qrwr. Found in its review (finding atelier-sklz).

## Done when

- Bundle references to existing evidence and reviews resolve like command arguments, and an unknown one uses the shared not-found message.
- A bundle test covers a short evidence ID and an unknown one.

## Close Reason

Bundle id references resolve existing issues and evidence through RecordIndex::resolve_any (shared short, case-insensitive matcher) and report unknown and wrong-kind references with the shared messages; covered by record_refs::bundle_ids_resolve_and_fail_like_command_arguments.
