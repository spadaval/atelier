---
created_at: "2026-06-15T15:17:12.722675388+00:00"
id: "atelier-4j3k"
issue_type: "task"
labels:
- "cargo"
- "workspace"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-epzs"
  - kind: "issue"
    id: "atelier-vu2b"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Convert root Cargo.toml to virtual workspace"
updated_at: "2026-06-15T15:17:12.722675388+00:00"
---

## Description

Make the repository root a virtual Cargo workspace after crate extraction is complete. The product binary remains target/debug/atelier through crates/atelier-cli, and no root package/lib/bin target remains.

## Outcome

- Root `Cargo.toml` is a virtual workspace manifest with no `[package]`, `[lib]`, or root `[[bin]]` product target.
- `crates/atelier-cli` declares `[[bin]] name = "atelier"` and builds the product executable at `target/debug/atelier`.
- Workspace membership and dependency direction match the architecture contract.

## Evidence

- File-change review artifact proves root package metadata is removed and the CLI-owned binary target exists.
- `cargo metadata --no-deps --format-version 1` transcript proves the root is not a package and the expected crates are workspace members.
- `cargo build --workspace --bins` or equivalent transcript proves `target/debug/atelier` is still produced.
