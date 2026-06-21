---
created_at: "2026-06-21T02:31:54.953611971+00:00"
id: "atelier-x8g1"
issue_type: "task"
labels:
- "docs"
- "quality"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-21T02:34:57.624918495+00:00"
status: "done"
title: "Document nextest as default Rust test runner"
updated_at: "2026-06-21T02:34:57.624918495+00:00"
---

## Description

Document the repository-specific Rust test runner default so future agents do
not fall back to `cargo test` out of general Rust habit.

## Outcome

`AGENTS.md` and the durable Rust standards both say `cargo nextest run` is the
default Rust test execution command, and both list the limited cases where
`cargo test` is acceptable with an explicit reason.

## Evidence

Documentation diff, `git diff --check`, and `atelier lint atelier-x8g1` pass.
