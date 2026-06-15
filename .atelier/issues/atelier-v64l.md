---
created_at: "2026-06-15T05:13:51.032041681+00:00"
id: "atelier-v64l"
issue_type: "task"
labels:
- "cli"
- "tests"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Split CLI integration tests by workflow surface"
updated_at: "2026-06-15T05:13:51.032041681+00:00"
---

## Description

Split the large CLI integration test file into focused workflow or fixture groups while preserving coverage for visible operator behavior.

## Outcome

- CLI integration tests are grouped by workflow surface or scenario rather than one oversized file.
- Shared helpers live in a test support module with clear ownership.
- Tests cover stable command intent without overfitting incidental formatting.

## Evidence

- Test file inventory shows the large integration file was split into focused modules.
- `cargo nextest run` passes.
- Representative CLI transcript/golden tests still cover status, issue, mission, evidence, lint, doctor, and export check.
