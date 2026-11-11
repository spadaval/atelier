---
created_at: "2026-10-17T01:10:57.984605781+00:00"
id: "atelier-d4b6"
issue_type: "bug"
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
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:37.272993561+00:00"
status: "superseded"
title: "Classify draft missions consistently and drop empty context panels"
updated_at: "2026-11-11T03:55:37.272993561+00:00"
---

## Goal

A `todo` mission is classified Context and listed under "Parents and Missions (context)" (fixture atelier-j76w), but a `draft` mission is Backlog because `can_enter_active` is false (crates/atelier-app/src/work_classification.rs:206-272), so on master `work all` lists atelier-1w8c and atelier-r6bt under "Backlog Work" among someday tasks. `work ready` also prints an empty `Parents and Missions (context)` / `(none)` panel.

## Done when

- [ ] Open missions of every status appear in one place with their status word, or `work all` leaves missions to `work missions`.
- [ ] Empty context panels are omitted in `work ready` and `work all`.
- [ ] Tests cover a draft and a todo mission.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes how `work all` and `work ready` classify missions, and the What is removed table removes the `work` views; the new `list` heads each open mission with its status.
