---
created_at: "2026-06-09T20:44:28.986051251+00:00"
id: "atelier-0022"
issue_type: "task"
labels:
- "cli"
- "links"
- "mission"
- "mission-view"
- "task"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add mission progress view over linked work graph"
updated_at: "2026-06-10T15:05:38.612806202+00:00"
---

## Description

Add a mission-centered read view that summarizes completed work, remaining ready/backlog work, blockers, checkpoint progress, plans, and evidence for one mission.

## Outcome

A mission view shows what has been done and what remains for a mission using first-class records and typed links; JSON output is deterministic; fixtures or tests cover a mission with linked milestones, plans, completed issues, ready issues, blocked issues, and evidence gaps; the view degrades clearly when links or first-class records are absent.

## Evidence

- `cargo fmt -- --check`

- `cargo test` or a named focused substitute

- `git diff --check`

- `atelier lint`

- `atelier export --check`

- `atelier doctor`

## Notes

### Scope

- Provide a command such as `atelier mission view <record-id>` or an equivalent `mission show --progress` mode after first-class mission records and typed links exist.
- Resolve linked epics/issues through explicit mission, milestone, plan, and evidence links rather than by treating the mission as an issue parent.
- Group output into done, ready, blocked, backlog, validation/evidence gaps, plans, and checkpoint progress.
- Emit stable JSON suitable for Mission Control projection and TUI consumption.
- Keep the view read-only; mutation remains in `mission`, `issue`, `plan`, `evidence`, and `link` commands.
- Use the single project-scoped random record ID form throughout output and examples.
