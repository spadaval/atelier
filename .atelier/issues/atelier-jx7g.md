---
created_at: "2026-10-17T07:17:24.325142041+00:00"
id: "atelier-jx7g"
issue_type: "bug"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-jx7g
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
closed_at: "2026-10-17T08:53:23.325390987+00:00"
status: "done"
title: "Use the right article for configured issue types in wrong-kind errors"
updated_at: "2026-10-17T08:53:23.325390987+00:00"
---

## Goal

`work mission <id>` on a non-mission says `"{id} is a {issue_type}, not a mission"` (`crates/atelier-cli/src/commands/work.rs`, about line 400). Issue types come from `.atelier/workflow.yaml`, so a custom type such as `incident` or `epic` prints "is a incident", the same grammar slip atelier-qrwr fixed for "a evidence". Found in review of atelier-qrwr (finding atelier-jlms).

## Done when

- Wrong-kind errors choose "a" or "an" from the type word, through the shared helper qrwr added.
- A test covers a vowel-initial custom type.

## Close Reason

work mission and the advances link-type error now choose a or an through record_ref::with_article, shared with wrong_kind; tests cover the custom incident type (evidence atelier-5zdf, atelier-a6id, atelier-pvs7)
