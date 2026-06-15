---
created_at: "2026-06-15T05:13:29.418889394+00:00"
id: "atelier-wz3t"
issue_type: "task"
labels:
- "rust"
- "workspace"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-nbni"
  - kind: "issue"
    id: "atelier-uz8g"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Create Cargo workspace and crate skeletons"
updated_at: "2026-06-15T05:13:29.418889394+00:00"
---

## Description

Convert the package layout into a Cargo workspace with the target internal crates while preserving the `atelier` binary name.

## Outcome

- Root Cargo metadata defines the workspace and expected crate members.
- Crate skeletons compile with minimal public APIs needed for subsequent extraction.
- The `atelier` binary remains available under the existing command name.
- Build scripts, package metadata, and local development docs still point to the correct binary and workspace commands.

## Evidence

- `cargo metadata --no-deps --format-version 1` lists the expected workspace members.
- `cargo fmt -- --check` and focused `cargo check` or `cargo nextest run` pass after the scaffold.
- CLI smoke transcript proves `atelier --help` still runs.
