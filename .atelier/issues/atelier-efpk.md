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
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair and consolidate CLI command surfaces"
updated_at: "2026-06-12T04:58:49.384211183+00:00"
---

## Description

Repair the CLI command hierarchy cleanup that the previous mission claimed but
did not complete. The issue command group still exposes lifecycle, graph,
search, comments, destructive maintenance, and legacy helper commands together.

## Outcome

- `atelier issue --help` exposes only issue lifecycle commands for creating,
  inspecting, editing compact metadata, claiming or starting where applicable,
  and closing or reopening where retained.
- Redundant issue creation paths are folded into `issue create` options or
  removed/hidden according to the new sectioned issue contract.
- Dependency, link, search, history/activity, hierarchy/impact, deletion, and
  bulk operations have explicit homes outside the primary issue lifecycle group
  where retained.
- Replacement flags such as parented create, active-work create, remove-label,
  and blocked listing exist where the contract says they should.
- Obsolete commands either fail clearly with a replacement or are hidden during
  an explicit migration window; they are not visible in primary help.
- Tests no longer preserve the old command surface as normal behavior.

## Evidence

- CLI transcript tests prove `atelier issue --help` lists only lifecycle commands
  and omits graph, search, history, impact, destructive maintenance, and legacy
  helper commands.

- CLI transcript tests prove retained replacement workflows exist for parented
  create, active-work create, remove-label, and blocked-list behavior when those
  behaviors remain in the target surface.

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
