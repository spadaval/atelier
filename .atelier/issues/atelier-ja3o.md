---
created_at: "2026-06-13T21:58:01.167376070+00:00"
id: "atelier-ja3o"
issue_type: "task"
labels:
- "architecture"
- "assignee:root"
- "stabilization"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T22:53:03.444820588+00:00"
status: "done"
title: "Remove hidden work start workflow bypass"
updated_at: "2026-06-13T22:53:03.444820588+00:00"
---

## Description

Hidden `atelier work start` remains callable after root `atelier start` moved to
workflow-backed lifecycle semantics. The hidden path records local runtime work
association through `commands::work::start` without applying the configured
issue transition, while root `start` calls `start_lifecycle` and
`workflow::transition_issue`.
- Hidden or legacy work-start entrypoints cannot bypass configured issue
  transition validators.
- Any retained runtime-only association primitive is internal to worktree/setup
  code and is not exposed as an operator command.
- Start/work guidance and tests describe one supported workflow start path.
- Help or rejected-command transcript proves `atelier work start` is absent or
  rejected as an unsupported hidden path.
- Focused CLI test or transcript proves `atelier start <issue-id>` applies the
  configured workflow transition before recording active work.
- `rg` residue search for `work start`, `start_lifecycle`, and
  `commands::work::start` classifies every retained caller.
- `cargo fmt -- --check`, focused work/workflow tests, `atelier lint`, and
  `atelier export --check` pass.
Audit evidence: `src/main.rs` dispatches root `start` through
`commands::work::start_lifecycle`, but hidden `WorkCommands::Start` dispatch
opens `runtime_db()` and calls `commands::work::start`; `src/commands/work.rs`
keeps both workflow-backed and runtime-only start helpers. Tests still exercise
the hidden path in `tests/cli_integration.rs`.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
