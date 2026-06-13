---
created_at: "2026-06-13T03:08:12.351942595+00:00"
id: "atelier-j6v4"
issue_type: "task"
labels:
- "cli"
- "lifecycle"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T04:30:10.807923587+00:00"
status: "done"
title: "Constrain lifecycle status changes to explicit commands"
updated_at: "2026-06-13T04:30:10.807923587+00:00"
---

## Description

Issue lifecycle state is currently exposed through multiple surfaces: root `start` and `finish`, `issue close`, `issue transition --options`, and raw `issue update --status`. This makes normal operation harder to teach and can send agents toward field mutation when they need a lifecycle action with readiness context. The command model should make lifecycle actions explicit and reserve `issue update` for record metadata edits.

## Outcome

- Normal help and next actions direct lifecycle changes through explicit lifecycle commands such as `atelier start`, `atelier finish`, `atelier issue close`, reopen, or the chosen transition surface.
- `atelier issue update --help` does not present raw status mutation as the ordinary way to start, close, reopen, or otherwise transition issue lifecycle state.
- Any retained low-level status mutation is hidden, guarded, or clearly documented as maintenance/internal repair, with the same readiness constraints as the explicit lifecycle command.
- Tests prove lifecycle readiness messages remain consistent across `issue show`, `issue transition --options`, close/reopen commands, and any retained repair path.

## Evidence

- Help transcript review covers root help, `issue update --help`, `issue close --help`, `start --help`, `finish --help`, and transition/reopen guidance.
- Focused CLI tests or transcripts cover ready issue start, blocked close, missing-evidence close, closed issue reopen, and any retained low-level status mutation behavior.
- Residue scan artifact from command/help/docs search classifies remaining `--status` references as normal lifecycle, metadata filter, or repair/internal use.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.
