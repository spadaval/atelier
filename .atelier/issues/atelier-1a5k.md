---
created_at: "2026-10-17T01:10:57.986493174+00:00"
id: "atelier-1a5k"
issue_type: "task"
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
status: "backlog"
title: "Stop work parent listing and counting the parent as its own row"
updated_at: "2026-10-17T01:10:57.986493174+00:00"
---

## Goal

`work parent` inserts the parent into its own rows (crates/atelier-cli/src/commands/work.rs:474-476): fixture `work parent atelier-qhnh` repeats the parent as `shown for context atelier-qhnh …` under Parents and Missions (or Blocked once it has a blocker) and counts it as `1 context`. On master, `work parent 1ioo` says "done 12" while `issue show 1ioo` Subissues says "11 total", and the two order the children differently. `Proof gaps: 4` counts the parent and three children without naming which lack proof. The title `Work Parent atelier-qhnh - …` differs from the mission's `<id> [mission] <status> - …`.

## Done when

- The parent appears only in the header (with its status and blockers); counts cover children only, so `work parent 1ioo` and `issue show 1ioo` agree on 11.
- Children appear in the same order in both views (the shared order from work-view-ordering.md).
- Proof gaps name the IDs or give the command that lists them.
- The title reads `atelier-qhnh [feature] todo - Build the widget parser · mission atelier-j76w`.
