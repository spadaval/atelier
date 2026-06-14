---
created_at: "2026-06-14T05:58:17.063833002+00:00"
id: "atelier-grmn"
issue_type: "task"
labels:
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Remove duplicate claim workflow"
updated_at: "2026-06-14T06:52:18.028484850+00:00"
---

## Description

Remove the normal durable claim workflow unless a distinct assignment policy is introduced; active work owns ordinary local work association.

## Outcome

Normal work coordination uses active work rather than a parallel durable claim
system. `atelier start` establishes the local active-work association, and
issue help/docs no longer present `issue update --claim` as an ordinary step.

## Evidence

- Help/docs transcript shows the normal worker path uses `atelier start <id>`
  and does not require `issue update --claim`.
- Source diff in `src/main.rs` and CLI help removes or hides the normal claim
  path, or product docs document any retained internal assignment support as
  out of the default workflow.
- Focused tests cover `start`, repeated start behavior, `status`, and
  `abandon` without relying on `--claim`.
- Search transcript shows Agent Factory and product docs do not tell workers
  to claim with `issue update --claim`.
- `git diff --check`, `atelier lint`, and focused active-work tests pass.
