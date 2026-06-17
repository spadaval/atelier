---
created_at: "2026-06-17T19:37:25.811227947+00:00"
id: "atelier-uro5"
issue_type: "task"
labels:
- "app-layer"
- "refactor"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-wpht"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Move command orchestration behind app APIs"
updated_at: "2026-06-17T19:37:25.811227947+00:00"
---

## Description

Move orchestration for representative command families behind `atelier-app`
APIs. The target workflows are status, mission, evidence, plan, and workflow
because they currently exercise the app/CLI/storage boundary most heavily.

Keep public CLI behavior stable unless a linked contract issue explicitly
changes it. This issue owns app request/outcome/view-model entrypoints and their
use from the CLI; it does not own final deletion of obsolete CLI adapters.

## Outcome

- `atelier-app` owns storage access selection, record resolution, canonical
  mutation orchestration, projection refresh, and view-model construction for
  the target workflows.
- `atelier-cli` dispatch passes parsed arguments into app APIs and renders app
  outcomes.
- Existing user-visible output remains behaviorally stable for the target
  workflows except for changes explicitly justified by tests or docs.

## Evidence

- Focused tests or transcripts prove status, mission, evidence, plan, and
  workflow commands still behave as expected.
- Search transcript shows CLI dispatch for migrated workflows no longer opens
  databases, constructs `RecordStore`, or refreshes projections directly.
- `cargo fmt -- --check` and targeted app/CLI tests pass.
