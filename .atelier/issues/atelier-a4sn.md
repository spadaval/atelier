---
created_at: "2026-06-12T05:04:31.932494248+00:00"
id: "atelier-a4sn"
issue_type: "task"
labels:
- "cli"
- "rework"
- "validators"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-pyre"
  - kind: "issue"
    id: "atelier-trr2"
  - kind: "issue"
    id: "atelier-ymfl"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Remove workflow validate from the normal public workflow"
updated_at: "2026-06-12T23:36:46.918917736+00:00"
---

## Description

Remove `atelier workflow validate` from the normal public/operator workflow.
Validation policy can remain an internal implementation concept, but users and
agents should interact through domain commands such as mission status, mission
closeout, issue transition options, start, close, lint, and evidence commands.

## Outcome

- Top-level help and Agent Factory guidance no longer present
  `atelier workflow validate` as a normal command.
- Mission and issue next-action output routes operators to domain surfaces
  instead of raw validator commands.
- Any remaining `atelier workflow validate` command is hidden or clearly scoped
  as an advanced/internal diagnostic, not a normal workflow command.
- Domain commands expose the useful answers previously expected from workflow
  validation: what is blocked, why, and which user-facing command fixes it.
- Tests prove normal help, mission status, issue show/transition, and closeout
  guidance do not recommend `workflow validate`.

## Evidence

- CLI transcript tests cover top-level help, mission status, mission closeout
  blockers, and issue transition/options output.

- Docs updates remove `workflow validate` from normal Agent Factory command
  lists.

- If an internal diagnostic remains, tests prove it is hidden or clearly marked
  non-normal.

- Run focused CLI/help tests plus `atelier lint`.

## Notes

The underlying validator functions may still exist as implementation details.
The product failure is exposing them as the normal mental model.
