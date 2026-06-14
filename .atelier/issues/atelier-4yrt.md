---
created_at: "2026-06-14T03:51:13.832115392+00:00"
id: "atelier-4yrt"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Align root CLI identity and command hierarchy with Zen"
updated_at: "2026-06-14T08:15:50.610183028+00:00"
---

## Description

Root help still opens with issue tracker positioning and gives state management,
diagnostics, integrations, predecessor import, and maintenance comparable
prominence to ordinary mission and work flows. After the command-specific
surface changes have landed, perform the root-help integration pass so the first
impression matches Atelier as a mission/proof-oriented work system. Normal
operator paths should come first; low-level export/rebuild, raw workflow
diagnostics, import-beads, command diagnostics, and removed integrations should
be hidden or only reachable through doctor/errors/docs when needed.

## Outcome

Root help presents Atelier as a mission/proof-oriented work system. It centers
orientation, mission coordination, issue work, active work, proof, history,
worktrees, lint, and doctor. It does not list integrations, generic link,
top-level dep, import-beads, export, rebuild, or raw workflow diagnostics as
general commands.

## Evidence

- `atelier --help` transcript shows product-centered identity text, normal
  operator commands before recovery/maintenance commands, and low-level state
  or diagnostic commands absent from general help or framed as doctor-guided
  recovery.
- Transcript shows removed integrations, generic link, top-level dep,
  import-beads, export, rebuild, and raw workflow diagnostics are not presented
  as normal command families.
- `git diff --check` and `atelier lint` pass after
  the help/docs changes.

## Notes

This is an integration task for root identity and hierarchy. It should not
invent replacement command contracts that are owned by the blocker, link,
integration-removal, init/import, workflow-check, or diagnostics-boundary tasks.
