---
created_at: "2026-06-09T19:46:37.730769567+00:00"
id: "atelier-001o"
issue_type: "task"
labels:
- "feature"
- "mission-control"
- "projection"
- "tui"
- "ui"
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-001w"
  - kind: "issue"
    id: "atelier-001x"
  - kind: "issue"
    id: "atelier-001y"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T19:54:14.390172050+00:00"
status: "done"
title: "Add Mission Control terminal UI"
updated_at: "2026-06-11T19:54:14.390172050+00:00"
---

## Description

Add an optional terminal UI for browsing Atelier state after Mission Control projection data is stable. The UI should consume existing command/projection APIs instead of owning a separate state model.

## Outcome

The TUI can load from deterministic Mission Control projection output, browse plan/backlog/mission views without corrupting tracker state, handles missing projection fields predictably, and has fixture-backed rendering or state tests for representative mission, backlog, blocker, plan-drift, and validation-failure states. Docs state that the CLI and JSON remain the primary agent interface.

## Evidence

- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`

## Notes

### Scope

- Provide a TUI entry point such as `atelier ui` or `atelier mission-control tui`.
- Show active missions, milestone checkpoint progress, backlog/ready work, blockers, plan drift, evidence gaps, workflow validator failures, branches/worktrees, and recent artifact updates from the Mission Control projection.
- Include keyboard navigation, filtering/search, record detail views, and copyable IDs/commands for agent handoff.
- Keep mutating operations narrow at first, such as claim/open/show commands, unless workflow validation and confirmation behavior are explicit.
- Degrade cleanly when mission, plan, evidence, workflow, or worktree records are not implemented yet.
