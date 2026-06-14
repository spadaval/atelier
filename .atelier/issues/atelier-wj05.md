---
created_at: "2026-06-13T23:10:38.230151438+00:00"
id: "atelier-wj05"
issue_type: "task"
labels:
- "architecture"
- "assignee:root"
- "refactor"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T00:16:50.020146279+00:00"
status: "done"
title: "Split workflow policy parser and validation hotspots"
updated_at: "2026-06-14T00:16:50.020146279+00:00"
---

## Description

The large-function scan reports dense workflow policy parsing and validation
functions. Extract parser and validator structure so policy changes are easier
to review without folding the work into unrelated command cleanup.

## Outcome

- `src/workflow_policy.rs` `parse_workflows` and `validate_policy` are split
  into cohesive parser or validator helpers, or explicitly justified if
  retained.
- `src/commands/workflow.rs` `transition_issue` is classified with the same
  workflow-policy boundary or moved to a smaller command adapter.
- Workflow policy diagnostics, transition validation, and operator guidance
  remain stable.

## Evidence

- Code review artifact or diff names the parser, validator, and command-adapter
  boundaries.
- Focused workflow transition or policy test transcript passes.
- Command transcripts for `cargo fmt -- --check`, `target/debug/atelier lint`,
  and `target/debug/atelier export --check` pass.
