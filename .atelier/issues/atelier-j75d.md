---
created_at: "2026-06-17T19:37:13.106579561+00:00"
id: "atelier-j75d"
issue_type: "epic"
labels:
- "app-layer"
- "architecture"
- "audit"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-98mo"
  children:
  - kind: "issue"
    id: "atelier-nm00"
  - kind: "issue"
    id: "atelier-uro5"
  - kind: "issue"
    id: "atelier-wpht"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Reconcile app/CLI boundary with live code"
updated_at: "2026-06-17T23:27:46.749901900+00:00"
---

## Description

Reconcile the app/CLI split with the live code. Closed tracker work claims the
CLI shell is thin and app APIs own use-case orchestration, but current command
dispatch still selects storage modes, opens projections, resolves records, and
calls command modules that mix mutation, projection refresh, and rendering.

This epic owns follow-up work only. Do not reopen closed app-layer epics; record
why their closeout proof was insufficient and replace the live boundary with one
that can be searched and tested.

## Outcome

- `atelier-app` exposes request, outcome, and view-model APIs for status,
  mission, evidence, plan, and workflow use cases.
- `atelier-cli` owns Clap parsing, telemetry, exit handling, and terminal
  rendering, without selecting storage access modes or performing use-case
  orchestration for migrated workflows.
- CLI command modules are either thin renderers/adapters or removed after app
  APIs replace them.
- A contract audit note explains the live-code contradiction with the closed
  app-layer tracker claim and names the proof required before this epic closes.

## Evidence

- Search transcript proves migrated CLI paths no longer call
  `command_storage`, `projection_query_db`, `canonical_mutation_db`,
  `RecordStore::new`, `Database::open`, or projection refresh helpers directly.
- Search transcript proves `atelier-app` use-case code does not call
  `println!` or `eprintln!`.
- Focused CLI tests or transcripts cover representative status, mission,
  evidence, plan, and workflow surfaces after migration.
- `atelier lint`, `atelier doctor`, and `git diff --check` pass.
