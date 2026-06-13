---
created_at: "2026-06-13T00:01:19.768971860+00:00"
id: "atelier-sdmo"
issue_type: "task"
labels:
- "cli"
- "docs"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-trr2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T00:07:56.501606835+00:00"
status: "done"
title: "Repair signpost validation gaps"
updated_at: "2026-06-13T00:07:56.501606835+00:00"
---

## Description

Repair the signpost validation gaps found by the independent `atelier-trr2` validation pass. The stale workflow configuration documentation must describe the implemented hidden advanced/internal workflow diagnostics instead of a future normal `workflow validate` JSON surface, and root `atelier status` must satisfy or explicitly supersede the `atelier-rqvv` status contract.
- `docs/product/workflow-configuration.md` no longer describes `atelier workflow validate` as a future normal config-health JSON command; it routes config health to `atelier lint`, `atelier doctor`, and hidden advanced/internal diagnostics only when explicitly requested.
- Root `atelier status` reports local git/worktree cleanliness, tracker health, active work, active mission health/progress, ready work scoped to the active mission, immediate blockers, recent relevant activity or an explicit empty state, and intent-labeled domain next actions.
- Normal status next actions continue to avoid raw workflow-validator commands.
- Focused signpost tests cover the repaired root status output and stale workflow-configuration docs text.
- Focused CLI integration tests for root status signpost fields and workflow configuration docs drift.
- Manual transcript or evidence capture for `atelier status`, `atelier lint`, and `atelier export --check`.
- Evidence record attached to this issue; verify with `atelier evidence show <id>`
  and `atelier issue show atelier-sdmo`.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
