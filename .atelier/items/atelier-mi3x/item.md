---
schema: atelier.item
schema_version: 3
id: atelier-mi3x
title: Choose status Next Actions from the checkout's state, in lifecycle order
status: draft
mission: atelier-4pzj
labels:
- cli-output
- status
- bug
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.983373200Z
---

## Goal

`print_status_view` (crates/atelier-cli/src/commands/status.rs, the `next_actions` block) picks Next Actions from coarse state. On an owner branch it always prints the same three lines (`Inspect owner work`, `Check handoff: atelier land <id>`, `Check merge readiness: atelier merge <id> --check`), whatever the owner's Done when, review and approval say; with no owner it offers `atelier work ready` and the record-only `atelier issue transition <top> start` rather than `atelier start`. Checked 2026-11-07. An agent working an owner should be told the one step the lifecycle puts next: start, implement, tick every Done when line and write the Report, `review request`, fix and re-review, approve, `merge`.

## Done when

- [ ] Next Actions is chosen in this order, each case with a CLI test:
  1. Uncommitted `.atelier/` paths: `Commit record changes: git add .atelier && git commit`.
  2. Owner with an unticked Done when line or an empty Report: `Continue <id>: atelier work parent <id>` (or `atelier issue show <id>` for an item without children).
  3. Owner with every line ticked and a Report, but no current approval or an open finding: `Request review: atelier review request <id>`.
  4. Owner with a current approval and no open finding: `Merge: atelier merge <id>`.
  5. Base with an unreviewed code commit: the `land_base_unreviewed` recovery.
  6. No owner and ready work: `Start the top ready item: atelier start <top-id>`, naming the real ID; the `issue transition <id> start` line goes.
  7. No issues at all: `Create the first issue: atelier issue create "…"`.
- [ ] The unborn-HEAD install commit (atelier-l457) and the light-path lines (atelier-4nsl) keep their places.

## Context

The order is atelier-jnrh's: `review request` refuses while a line is unticked or the Report is empty, and `merge` marks the work done, so there is no done-owner or land-handoff step (decision atelier-mzpn). status is the SessionStart view (atelier-sdb0).
