---
created_at: "2026-10-17T01:10:57.988372301+00:00"
id: "atelier-03kt"
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
closed_at: "2026-11-05T06:02:29.983449290+00:00"
status: "superseded"
title: "Point wrong-kind work mission and work parent errors at the right view"
updated_at: "2026-11-05T06:02:29.983449290+00:00"
---

## Goal

- `work parent atelier-j76w` (a mission with 10 in-scope items) errors `atelier-j76w has no children; work parent shows an issue with children …`, which is false: the check merges "is a mission" with "has no subissues" (crates/atelier-cli/src/commands/work.rs:357).
- `work mission atelier-qhnh` (a feature with children) says it "requires a mission" but does not suggest `work parent atelier-qhnh`.
- On master, `work mission sdb0` says it "requires a mission" though sdb0 advances atelier-1w8c; it should point there.

## Done when

- `work parent <mission>` says `atelier-j76w is a mission; use atelier work mission atelier-j76w`.
- `work mission <issue>` suggests `atelier work mission <mission>` for each mission the issue advances, and `atelier work parent <id>` when it has children.
- Tests cover all three cases.

## Close Reason

Already fixed on master c48378b8: wrong-kind work mission and work parent errors name the right view (crates/atelier-cli/src/commands/work.rs:334-374), pinned by tests in crates/atelier-cli/tests/cli_integration/record_refs.rs:323-370. Found by the 2026-11-05 currency check during the mission regroup.
