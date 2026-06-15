---
created_at: "2026-06-15T15:16:21.401620485+00:00"
id: "atelier-sclf"
issue_type: "task"
labels:
- "cli"
- "rendering"
- "telemetry"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Own CLI parser dispatch tracing telemetry and rendering in atelier-cli"
updated_at: "2026-06-15T15:16:21.401620485+00:00"
---

## Description

Move the CLI-facing shell into atelier-cli so Clap definitions, parsing, tracing setup, telemetry recording, command identity, terminal rendering, exit handling, and the atelier binary target are owned by the CLI crate.

## Outcome

- `crates/atelier-cli` owns Clap command definitions, parsing, command identity, tracing setup, telemetry recording, terminal rendering, exit-code mapping, and the `[[bin]] name = "atelier"` target.
- CLI code delegates use-case orchestration to `atelier-app` through explicit request/outcome/view APIs.
- Removed commands fail as ordinary unknown Clap commands; no runtime guidance shim is preserved for old command names.

## Evidence

- Search transcript or review artifact shows Clap, tracing, telemetry, rendering, and binary target ownership under `crates/atelier-cli`.
- CLI transcripts prove `target/debug/atelier --help` and representative command help remain behaviorally stable except removed-command guidance is absent.
- Removed-command transcript proves an old command is rejected by Clap as unknown without specialized runtime cleanup guidance.
