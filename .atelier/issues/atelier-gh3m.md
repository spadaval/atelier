---
created_at: "2026-06-14T03:51:28.546933765+00:00"
id: "atelier-gh3m"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4yrt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Fold normal workflow checks into lint and visible status surfaces"
updated_at: "2026-06-14T03:51:28.546933765+00:00"
---

## Description

`workflow check` and `workflow init` are hidden from root help and labelled
advanced/internal, but repository guidance still mentions workflow commands as
operator validation or setup paths. Remove workflow commands from the normal
operator contract. `atelier init` owns starter workflow policy creation,
`atelier lint` owns committed workflow configuration and canonical
record-health checks, `atelier issue transition <id> --options` owns issue
transition readiness, `atelier mission status` and `atelier mission audit` own
mission closeout readiness, and `atelier doctor` owns runtime/install health.
Any raw workflow diagnostic that remains is an advanced development/debug
surface, not a required handoff command.

## Outcome

Normal workflow policy setup and checks are folded into visible operator
surfaces: `atelier init` creates starter policy, `atelier lint` validates
committed workflow configuration and canonical record health, transition and
mission-status commands report readiness, and `doctor` reports runtime health.
No separate workflow command is required for ordinary setup, handoff, or
closeout.

## Evidence

- Root help, `atelier workflow --help`, repository docs, and Agent Factory
  guidance do not route ordinary setup or handoff through workflow commands.
- `atelier lint` covers workflow configuration validity, while transition,
  mission status/audit, and doctor cover readiness and runtime health.
- Help or docs transcript shows any retained raw workflow diagnostic labelled
  advanced/debug only.
- `git diff --check` and `atelier lint` pass after
  the help/docs changes.
