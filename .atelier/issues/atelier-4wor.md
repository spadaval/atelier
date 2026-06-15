---
created_at: "2026-06-15T05:11:21.464664060+00:00"
id: "atelier-4wor"
issue_type: "epic"
labels:
- "rewrite"
- "rust"
- "workspace"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fchz"
  children:
  - kind: "issue"
    id: "atelier-fjmw"
  - kind: "issue"
    id: "atelier-nbni"
  - kind: "issue"
    id: "atelier-wz3t"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Workspace scaffold and domain extraction"
updated_at: "2026-06-15T05:13:32.914496453+00:00"
---

## Description

Create the Cargo workspace shape and move stable domain/workflow code into internal crates before deeper storage and CLI rewrites. This epic should improve incremental build isolation without preserving the old Rust module API.

## Outcome

- The repository builds as a Cargo workspace while the binary remains `atelier`.
- `atelier-core` owns record IDs, domain record data types, relationships, and shared value validation without filesystem, SQLite, Clap, or telemetry dependencies.
- `atelier-workflow` owns workflow policy parsing and transition semantics on top of `atelier-core`.
- Existing repo tests and fuzz targets compile against the new internal crate APIs rather than old `atelier::db::Database` paths.

## Evidence

- Child issue proof shows workspace creation, domain extraction, and workflow extraction completed under the architecture contract.
- `cargo metadata --no-deps --format-version 1` shows the expected workspace members.
- Focused crate tests for core and workflow pass, plus `cargo fmt -- --check` and `cargo nextest run`.
