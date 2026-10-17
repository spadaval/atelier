---
created_at: "2026-10-17T01:10:57.985530787+00:00"
id: "atelier-5q5v"
issue_type: "feature"
labels:
- "cli-output"
- "work-views"
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
status: "backlog"
title: "Show parent and child structure in work mission"
updated_at: "2026-10-17T01:10:57.985530787+00:00"
---

## Goal

Every mission row is rendered at `depth: 1` (crates/atelier-cli/src/commands/work.rs:536). Fixture parent atelier-qhnh's children land in Active (s47h) and Blocked (1gc5, 9t9q) with nothing tying them to qhnh, and `--all` prints one ID-sorted "Scoped Work" list where needs-human, backlog and parent rows all read "shown for context". A reader cannot tell which items share one branch.

## Done when

- `work mission --all` and `--blocked` indent children under their parent:
  ```
  atelier-qhnh … ⚠ blocked
      atelier-s47h … active
      atelier-9t9q … blocked by atelier-s47h
  ```
- In summary sections, child rows end with `· parent atelier-qhnh`.
- A test covers a parent with children in different states.
