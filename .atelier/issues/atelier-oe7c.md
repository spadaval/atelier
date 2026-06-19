---
created_at: "2026-06-19T03:57:22.961668505+00:00"
id: "atelier-oe7c"
issue_type: "epic"
labels:
- "review"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0jsk"
  - kind: "issue"
    id: "atelier-q199"
  - kind: "issue"
    id: "atelier-swxv"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Validate review rooms and command migration"
updated_at: "2026-06-19T05:04:14.510766693+00:00"
---

## Description

Validate the mission as a product behavior change across room mode, provider
mode, migration, command removal, documentation, and tracker health.

## Outcome

- Room-mode behavior is proven end to end, including stale approval, blocking
  finding, and local merge behavior.
- Provider-mode Forgejo parity is proven through `atelier review` commands.
- Wrong-mode command rejection, old-field rejection, `atelier pr` rejection,
  docs/help parity, and final repository checks are all covered by evidence.
- Validation findings are recorded as actionable issues instead of being fixed
  inside validation work.

## Evidence

- Dedicated validation records attach transcripts or test output for room mode,
  provider mode, wrong-mode rejection, and closeout checks.
- `cargo fmt -- --check`, focused tests, `cargo nextest run`,
  `cargo nextest run --profile extended --run-ignored=only` when relevant,
  `git diff --check`, `atelier lint`, and `atelier doctor` results are recorded.
- `atelier mission status atelier-zief` shows no open blockers or terminal gaps
  except any explicitly documented residual risk.
