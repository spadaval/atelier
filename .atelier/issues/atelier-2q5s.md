---
created_at: "2026-06-15T05:11:26.860310564+00:00"
id: "atelier-2q5s"
issue_type: "epic"
labels:
- "app-layer"
- "cli"
- "rewrite"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fchz"
  children:
  - kind: "issue"
    id: "atelier-14z2"
  - kind: "issue"
    id: "atelier-vv2i"
  - kind: "issue"
    id: "atelier-zwna"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Split application layer and CLI shell"
updated_at: "2026-06-15T05:13:49.175124659+00:00"
---

## Description

Separate command orchestration from Clap parsing by introducing an application layer and thinning the CLI shell. Preserve visible command intent, help/docs alignment, and the human-first output contract.

## Outcome

- `atelier-app` owns use-case orchestration, command handlers, human output view models, and coordination between records, SQLite, workflow, and runtime services.
- `atelier-cli` owns Clap definitions, telemetry setup, exit-code handling, and thin delegation into `atelier-app`.
- Oversized command handlers are split by product job and view model ownership while keeping help-visible command jobs stable.
- No old command aliases, fallback shims, or public compatibility facades are introduced.

## Evidence

- Child issue proof shows application ports, handler/view-model extraction, and thin CLI shell wiring.
- CLI/help/docs parity checks cover visible workflow surfaces.
- Representative command transcripts for status, issue, mission, evidence, lint, doctor, and export check remain behaviorally stable.
