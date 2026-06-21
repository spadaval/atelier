---
created_at: "2026-06-21T16:37:30.766454867+00:00"
id: "atelier-62po"
issue_type: "feature"
labels:
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2kfb"
  - kind: "issue"
    id: "atelier-fyc9"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-21T18:54:40.574151059+00:00"
status: "done"
title: "Make issue show/status own mission objective views"
updated_at: "2026-06-21T18:54:40.574151059+00:00"
---

## Description

Ensure `atelier issue show <mission-id>` and `atelier issue status <mission-id>` provide all useful mission view behavior before removing mission-specific reads.

## Outcome

- Issue show renders mission intent, constraints, risks, validation, linked work, blockers, evidence, history pointers, and file path.
- Issue status renders ready work, blocked work, proof gaps, validator failures, terminal readiness, and next commands.
- Status output is type-aware without exposing a separate mission workflow engine.

## Evidence

- CLI transcript covers rich show, compact status, verbose/status-drilldown if retained, and quiet mode.
- Focused tests cover mission/objective records and ordinary issues.
