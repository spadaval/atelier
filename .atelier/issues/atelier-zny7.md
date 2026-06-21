---
created_at: "2026-06-21T16:37:30.761972151+00:00"
id: "atelier-zny7"
issue_type: "task"
labels:
- "adr"
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-62po"
  - kind: "issue"
    id: "atelier-e7t1"
  - kind: "issue"
    id: "atelier-iv2x"
  - kind: "issue"
    id: "atelier-nbhp"
  - kind: "issue"
    id: "atelier-ncq9"
  - kind: "issue"
    id: "atelier-s43l"
  - kind: "issue"
    id: "atelier-vays"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Record ADR for mission command and workflow authority"
updated_at: "2026-06-21T18:21:42.793777155+00:00"
---

## Description

Record the architecture decision that separates mission domain shape from workflow policy and command namespaces.

## Outcome

- ADR states where mission lifecycle/status policy lives.
- ADR states why retained mission behavior cannot be hardcoded in CLI command modules.
- ADR lists rejected alternatives, including keeping a mission root namespace and keeping mission-specific close/status logic.

## Evidence

- File diff shows the ADR file and its docs/adr/index.md or docs/architecture/index.md link.
- `atelier lint` passes and targeted searches show the ADR is discoverable.
