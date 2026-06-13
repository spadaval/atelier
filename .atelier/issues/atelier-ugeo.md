---
created_at: "2026-06-10T20:58:56.691138582+00:00"
id: "atelier-ugeo"
issue_type: "epic"
labels:
- "assignee:root"
- "cli"
- "human-output"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-u8xc"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T22:15:52.530565986+00:00"
status: "done"
title: "Upgrade mission and issue detail human views"
updated_at: "2026-06-10T22:15:52.530565986+00:00"
---

## Description

Make the primary detail views dense, scannable, and useful by default.

Why:
`atelier mission show` currently shows too little graph and evidence context, and issue detail output needs to fit into a broader consistent output system. Operators should be able to answer what this record is, why it matters, what blocks it, what advances it, what changed recently, and what command to run next.

Scope:
- Improve `atelier mission show` human output with linked work, blockers, evidence gaps, constraints, risks, progress, and next-action cues.
- Reconcile existing richer `atelier issue show` work with the new shared formatter conventions.
- Keep detail views readable in both narrow and wide terminals.
- Preserve `--json` output compatibility.

Out of scope:
- Full Mission Control TUI.
- Large persistence-model changes.
- Removing command surfaces.

## Outcome

- Mission detail output provides enough context to coordinate work without immediately falling back to multiple separate commands.
- Issue detail output follows the same visual grammar as mission detail output.
- Tests or snapshots cover representative records with hierarchy, blockers, activity, empty sections, and linked evidence.

Recommended subskill: agent-factory implement.

## Evidence

Evidence was not specified in the legacy issue record.
