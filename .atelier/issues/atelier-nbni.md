---
created_at: "2026-06-15T05:13:31.193239010+00:00"
id: "atelier-nbni"
issue_type: "task"
labels:
- "domain"
- "rust"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fjmw"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Extract core domain types into atelier-core"
updated_at: "2026-06-15T05:13:31.193239010+00:00"
---

## Description

Move stable domain values and validation into `atelier-core` without carrying filesystem, SQLite, Clap, or telemetry dependencies into that crate.

## Outcome

- `atelier-core` owns record IDs, domain record structs, relationships, issue and record value validation, and shared constants that are not storage-specific.
- The crate has no direct dependency on filesystem layout, SQLite, Clap, or command telemetry.
- Existing callers compile against the new internal APIs or tracked adapters.

## Evidence

- `cargo metadata --no-deps --format-version 1` transcript shows `atelier-core` has no prohibited dependencies.
- Unit tests cover ID and value validation invariants.
- `cargo nextest run` passes for affected crates.
