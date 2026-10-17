---
created_at: "2026-10-17T08:52:51.231100900+00:00"
id: "atelier-f4ko"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-f4ko
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
title: "Name the record a load-time policy or reference error is about"
updated_at: "2026-10-17T08:57:29.951861253+00:00"
---

## Goal

Found in review of atelier-x68o:
- An issue whose status or issue_type is invalid under the workflow makes every query fail with an error naming the absolute path of `workflow.yaml`; the issue's own file is never named, yet the recovery step says "fix every named record file" (`crates/atelier-app/src/use_cases.rs`, about line 20; finding atelier-nl8o).
- A review whose target is missing loads as "Issue <id> has review target reference ...", calling the review an Issue and naming no path, and `check` reports the same broken target twice (`crates/atelier-app/src/record_index.rs`, about line 784; finding atelier-1rb4).

## Done when

- Each error names the record file it is about, once, with the right record kind.
- `check` reports a broken review target once.
- Tests cover an invalid status, an invalid issue type and a missing review target.
