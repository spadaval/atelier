---
created_at: "2026-06-15T05:13:49.175124659+00:00"
id: "atelier-zwna"
issue_type: "task"
labels:
- "cli"
- "rust"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T08:15:15.215080071+00:00"
status: "done"
title: "Thin atelier-cli to Clap telemetry and exit-code shell"
updated_at: "2026-06-15T08:15:15.215080071+00:00"
---

## Description

Reduce the final binary crate to a thin shell that owns Clap definitions, telemetry initialization, and process exit behavior.

## Outcome

- `atelier-cli` parses arguments and delegates to `atelier-app`.
- Binary name remains `atelier`.
- Telemetry/logging setup and exit-code mapping stay at the process boundary.
- The CLI crate does not own domain mutation, Markdown parsing, SQLite query, or workflow semantics.

## Evidence

- CLI smoke and representative command transcripts prove the binary still works.
- File review shows storage, workflow, and domain behavior is not implemented in the CLI shell.
- `cargo nextest run` and `cargo fmt -- --check` pass.
