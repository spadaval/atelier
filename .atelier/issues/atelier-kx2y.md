---
created_at: "2026-06-23T20:23:55.654489472+00:00"
id: "atelier-kx2y"
issue_type: "epic"
labels:
- "cli"
- "human-output"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3js3"
  - kind: "issue"
    id: "atelier-4wmp"
  - kind: "issue"
    id: "atelier-5sgx"
  - kind: "issue"
    id: "atelier-7fof"
  - kind: "issue"
    id: "atelier-7ze4"
  - kind: "issue"
    id: "atelier-t8ew"
  - kind: "issue"
    id: "atelier-wxox"
  - kind: "issue"
    id: "atelier-ycj9"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-c0qc"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Refresh human CLI output surfaces"
updated_at: "2026-06-23T22:34:43.008486946+00:00"
---

## Description

Coordinate the formatter, command-surface, evidence/history, role-guide, and validation slices for the human CLI output refresh. This epic replaces the invalid mission-as-parent hierarchy for the original output refresh child issues.

The mission remains the objective boundary through an advances link; this epic owns the structural child work and reviewable branch for the output-surface implementation.

## Outcome

The original human-output implementation, command-surface, evidence/history, role-guide, and validation slices are owned by this epic instead of by the mission. `atelier-c0qc` advances this epic through a typed mission link, and this epic provides the structural branch/review boundary for the output-surface work.
