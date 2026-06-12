---
created_at: "2026-06-12T04:51:43.133985401+00:00"
id: "atelier-0j6e"
issue_type: "task"
labels:
- "lint"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-n1ys"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Enforce required issue sections in lint and workflow gates"
updated_at: "2026-06-12T04:51:43.133985401+00:00"
---

## Description

Make required issue structure enforceable. Missing or empty required sections
should not be a cosmetic problem; they should block lint and prevent agents from
starting or closing work that has no defined desired outcome.

## Outcome

- `atelier lint` and `atelier lint <id>` fail when an issue is missing required
  issue sections or has empty required sections.
- `atelier work start <id>` refuses to start structurally invalid issues.
- Issue closeout and mission closeout paths cannot pass while linked
  implementation work has invalid issue structure.
- Lint messages name the issue ID, missing or empty section, and the Markdown
  file to edit.
- Enforcement uses parsed section data, not brittle string searches.

## Evidence

- CLI integration tests prove lint fails for missing Outcome, empty Outcome, and
  duplicate recognized headings.
- CLI integration tests prove start fails for a structurally invalid issue.
- Mission or issue closeout tests prove invalid linked work blocks closeout.
- Run focused lint/workflow tests.

## Notes

Do not make checkbox edits the completion mechanism. Required sections define
the work and proof expectations; actual proof stays in command output, evidence
records, tests, transcripts, or review.
