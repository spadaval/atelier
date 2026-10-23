---
created_at: "2026-10-23T15:17:21.199931065+00:00"
id: "atelier-r3oy"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
  - kind: "review"
    id: "atelier-s8kw"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Remove the unreachable reopen fix and make the fix helper rerun the gate"
updated_at: "2026-10-23T15:17:21.199931065+00:00"
---

## Goal

Two leftovers from atelier-h1nc review. (1) `reopen_command` in crates/atelier-cli/src/gate_report.rs can never print: policy validation rejects any transition leaving a done status, so a closed ancestor has no reopen transition; delete it and the matching clause in docs/product/human-cli-output.md (finding atelier-s8kw). (2) The verbatim-fix helper crates/atelier-cli/tests/support/next_steps.rs checks each printed fix exits 0 and changes state but never reruns the gate to confirm the failing check cleared, and the tests exercise only the blocked_by unlink and a todo child; rerun the gate after each fix and cover the blocks-declared unlink and backlog, blocked and review children under a closed owner (finding atelier-oy90).

## Done when

`reopen_command` and its doc clause are gone; the helper asserts the targeted check no longer fails after its fix runs, and the listed cases are covered by tests.

## Context

Review findings atelier-s8kw and atelier-oy90 on atelier-h1nc.
