---
created_at: "2026-10-17T01:10:57.966093266+00:00"
id: "atelier-xw8r"
issue_type: "bug"
labels:
- "cli-output"
- "start"
priority: "P2"
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
closed_at: "2026-11-11T03:55:41.724747358+00:00"
status: "superseded"
title: "Point start hints at atelier start, not the record-only transition"
updated_at: "2026-11-11T03:55:41.724747358+00:00"
---

## Goal

The `issue transition` help says "To begin work, use `atelier start <id>` … `start` here only changes the record". Yet the hints point at the record-only transition:
- `issue show`'s Transition Readiness prints `start: allowed - to in_progress / atelier issue transition atelier-43m1 start` (`crates/atelier-cli/src/commands/issue.rs:284-323`).
- The options view prints the same (`workflow.rs` `transition_command`).
- A refused `atelier start sdb0` prints `Command: atelier issue transition atelier-sdb0 start`.

Following them moves the item to in_progress without a branch or `workflow_branch`, and nothing warns. In astromech `issue transition … start` ran 63 times against `atelier start` 31 times. Separately, `atelier start 1w8c` on a mission says `Start one of its issues instead.` and doesn't name them.

## Done when

- [ ] Every place the `start` transition is offered as an action shows `atelier start <id>`.
- [ ] `issue transition <id> start` run outside the item's owner branch prints a warning that no branch was recorded, and names `atelier start <id>`.
- [ ] `atelier start <mission>` lists the mission's ready items (ID and title, up to 5), or says none is ready.
- [ ] CLI tests cover the show hint, the warning and the mission error.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new tool has no record-only `issue transition`; `status` names `atelier start <id>`, and `start` on a mission points at `atelier status` (run in a scratch clone).
