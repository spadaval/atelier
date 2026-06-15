---
created_at: "2026-06-15T03:16:19.596144127+00:00"
id: "atelier-3scd"
issue_type: "task"
labels:
- "cli"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-k95l"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T03:26:03.868996940+00:00"
status: "done"
title: "Update docs for role-specific man guidance"
updated_at: "2026-06-15T03:26:03.868996940+00:00"
---

## Description

Revise the accepted guidance ownership and CLI product docs so
`atelier man <role>` owns role-specific tactical guidance and `atelier prime`
is removed.

## Outcome

- ADR 0006, product CLI docs, command audit docs, Agent Factory binding, and
  validation guidance describe `man` as the role guide surface.
- Prime is no longer documented as an active command or onboarding path.
- Manager is documented as the CLI role class, with orchestrator as a specific
  Agent Factory agent type in that class.

## Evidence

- File content check: relevant docs contain `atelier man worker`,
  `atelier man reviewer`, `atelier man manager`, and `atelier man admin`
  guidance.
- Text search: active command guidance no longer references
  `atelier prime`.
- Command-surface check: docs/help drift validation reports clear.
