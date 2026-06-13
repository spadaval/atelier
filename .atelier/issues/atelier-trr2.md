---
created_at: "2026-06-12T20:29:46.679510166+00:00"
id: "atelier-trr2"
issue_type: "validation"
labels:
- "cli"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-wpyb"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Validate workflow signpost surfaces end to end"
updated_at: "2026-06-13T00:25:49.979408942+00:00"
---

## Description

Validate the workflow signpost surfaces after status, start/finish, transition,
history, prime, and workflow-validator public-surface repairs land.

## Outcome

- Root status, mission status, mission show, start, finish/current-work,
  transition/options, history, and prime surfaces each have clear
  non-overlapping responsibilities.
- Normal next actions route users to domain surfaces, not raw workflow-validator
  commands.
- Help, docs, tests, and Agent Factory guidance agree on the implemented
  signpost surfaces.
- Historical decision records `atelier-rqvv`, `atelier-v02t`, `atelier-vr9g`,
  `atelier-hggl`, and `atelier-bzts` are either satisfied or explicitly
  superseded by a new tracker item.

## Evidence

- Positive and negative CLI transcripts for the major signpost surfaces.
- Manual docs/help/Agent Factory parity check transcript or evidence record.
- Focused CLI integration tests for status, start/finish, transition, history,
  prime, and raw-workflow-validator absence from normal guidance.
- `atelier lint` and `atelier export --check`.
