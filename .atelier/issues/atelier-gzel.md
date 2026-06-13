---
created_at: "2026-06-13T23:12:00.253045347+00:00"
id: "atelier-gzel"
issue_type: "task"
labels:
- "dependencies"
- "quality"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Triage cargo machete signal-hook finding"
updated_at: "2026-06-13T23:32:01.042208323+00:00"
---

## Description

`cargo machete` reports `signal-hook` as an unused dependency in `Cargo.toml`. Determine whether the dependency is truly unused, remove it if safe, or add a justified `cargo-machete` ignore entry if it is required indirectly.

## Outcome

- The `signal-hook` finding from `cargo machete` is resolved by a manifest cleanup or a documented ignore with rationale.
- `cargo machete` no longer reports an unowned `signal-hook` finding.

## Evidence

- `cargo machete` transcript shows the finding is gone or intentionally ignored.
- `target/debug/atelier lint` and `target/debug/atelier export --check` pass after the manifest or config update.
