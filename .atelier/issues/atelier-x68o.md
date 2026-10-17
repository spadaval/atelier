---
created_at: "2026-10-17T07:19:08.756123072+00:00"
id: "atelier-x68o"
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
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Say a bad record's path once in record-load errors"
updated_at: "2026-10-17T08:41:50.703070286+00:00"
---

## Goal

Record-load errors that already name the record path get it again from the record-index wrapper (`crates/atelier-app/src/record_index.rs`, about line 88): `Invalid record file .atelier/issues/x.md: priority 'low' in .atelier/issues/x.md: records store P0 (critical), P1 (high), P2 (medium) or P3 (low)`. Found in review of atelier-jpok (finding atelier-q71m).

## Done when

- A record-load error names the path once, whether the inner error carries it or not.
- A test pins the priority case.
