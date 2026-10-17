---
created_at: "2026-10-17T06:22:17.873203992+00:00"
id: "atelier-31l2"
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
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Order work mission and work parent rows by the shared work order"
updated_at: "2026-10-17T08:41:50.273867789+00:00"
---

## Goal

`docs/product/work-view-ordering.md` says `work mission` and `work parent` order rows by visible blockers, then row state, priority, newest update and ID. `ScopedWork::new` (`crates/atelier-cli/src/commands/work.rs`, about lines 508-534) never applies that order: `work mission atelier-o3lm --all` lists rows by ID, with the active row among the backlog rows. Found in review of atelier-i2qq (finding atelier-tltm); already true on master.

## Done when

- `work mission <id>` and `work parent <id>`, with and without `--all`, list each section in `work_order::ordered_work_indices` order.
- A CLI test pins the order of a mixed `--all` list.

## Notes

- 2026-10-17 [note] atelier-ksx6 (merged 7e3339d8) now lists work mission and work parent rows in the shared work order, parent row first, and its session test pins the order of ready items. Check the remaining Done when (a CLI test pinning a mixed --all list) before closing or superseding this.
