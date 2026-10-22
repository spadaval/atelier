---
created_at: "2026-10-17T07:19:08.756123072+00:00"
id: "atelier-x68o"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-x68o
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
    id: "atelier-caek"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-q71m"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T08:48:18.175584742+00:00"
status: "done"
title: "Say a bad record's path once in record-load errors"
updated_at: "2026-10-17T08:48:18.175584742+00:00"
---

## Goal

Record-load errors that already name the record path get it again from the record-index wrapper (`crates/atelier-app/src/record_index.rs`, about line 88): `Invalid record file .atelier/issues/x.md: priority 'low' in .atelier/issues/x.md: records store P0 (critical), P1 (high), P2 (medium) or P3 (low)`. Found in review of atelier-jpok (finding atelier-q71m).

## Done when

- A record-load error names the path once, whether the inner error carries it or not.
- A test pins the priority case.

## Close Reason

The record-index wrapper now adds 'Invalid record file X' only when no message in the error chain names X, so record-load errors (queries and check) name the path once; record_index tests pin the priority and invalid-YAML cases.
