---
created_at: "2026-06-12T04:58:49.384211183+00:00"
id: "atelier-efpk"
issue_type: "epic"
labels:
- "cli"
- "rework"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-64ea"
  - kind: "issue"
    id: "atelier-auqt"
  - kind: "issue"
    id: "atelier-drfm"
  - kind: "issue"
    id: "atelier-exz1"
  attachments:
  - kind: "evidence"
    id: "atelier-cqav"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Repair and consolidate CLI command surfaces"
updated_at: "2026-06-12T21:38:50.304358458+00:00"
---

## Description

Repair the CLI command hierarchy cleanup that the previous mission claimed but
did not complete. The issue command group still exposes lifecycle, graph,
search, comments, destructive maintenance, and legacy helper commands together.
Use the closed command-surface decisions in `atelier-9jbu` and `atelier-o2a4`
as the starting point, then validate the current implementation against them.

## Outcome

- `atelier issue --help` exposes only issue lifecycle commands for creating,
  inspecting, editing compact metadata, starting through the approved start
  surface, and closing or reopening through approved lifecycle commands.
- Redundant issue creation paths are folded into `issue create` options or
  removed/hidden according to the active command classification.
- Dependency, link, search, history/activity, hierarchy/impact, deletion, and
  bulk operations have explicit homes outside the primary issue lifecycle group
  when retained.
- Replacement flags such as parented create, active-work create, remove-label,
  and blocked listing exist where the contract says they should.
- Obsolete commands either fail clearly with a replacement or are hidden during
  an explicit migration window; they are not visible in primary help.
- Tests no longer preserve the old command surface as normal behavior.

## Evidence

- CLI transcript tests prove `atelier issue --help` lists only lifecycle commands
  and omits graph, search, history, impact, destructive maintenance, and legacy
  helper commands.

- CLI transcript tests prove replacement workflows exist for retained parented
  create, active-work create, remove-label, and blocked-list behavior.

- Removed or hidden commands have explicit tests for failure, hidden-help
  behavior, or replacement guidance.

- A command inventory compares old issue subcommands with their final normal,
  moved, hidden, or removed status.

- Focused CLI integration tests exercise the command hierarchy without relying
  on old subcommands as normal behavior.

- Run `atelier export --check` and `atelier lint`.

## Notes

This is new repair work. Do not reopen the old closed command-cleanup issue
unless a human wants its historical record changed.
