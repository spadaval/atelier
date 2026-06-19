---
created_at: "2026-06-19T03:58:41.591352441+00:00"
id: "atelier-at7i"
issue_type: "feature"
labels:
- "review"
- "room"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T04:57:25.727360134+00:00"
status: "done"
title: "Implement room open status show and timeline comments"
updated_at: "2026-06-19T04:57:25.727360134+00:00"
---

## Description

Implement the room creation and read/comment command surface for room-mode
projects.

## Outcome

- `atelier review open` creates or confirms the owner issue/epic room and
  writes the structured issue `review` field.
- `atelier review status` and `show` render current state derived from room
  events, including owner, branches, head, decisions, findings, and next action.
- `atelier review comment` writes room-level comments by default and supports
  file and optional line anchors without PR-style inline thread semantics.
- `atelier review comments` lists the ordered room timeline in a stable human
  format.

## Evidence

- CLI tests cover open, repeated open, status, show, comment, comments, file
  anchor, line anchor, and invalid anchor cases.
- Fixture diff shows deterministic `.atelier/reviews/<id>.yaml` event rendering.
- `atelier lint atelier-at7i` and focused room command tests pass.
