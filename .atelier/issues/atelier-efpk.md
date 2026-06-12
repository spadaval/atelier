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

- `atelier issue --help` exposes only the agreed issue lifecycle surface.
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

- CLI transcript tests cover the reduced `atelier issue --help` output.

- CLI transcript tests cover retained replacement workflows.

- Removed or hidden commands have explicit tests for failure or hidden-help

behavior.

- Run focused CLI integration tests for issue command hierarchy.

- Run `atelier export --check` and `atelier lint`.

## Notes

This is new repair work. Do not reopen the old closed command-cleanup issue

unless a human wants its historical record changed.
