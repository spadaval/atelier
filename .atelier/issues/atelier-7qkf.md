---
created_at: "2026-06-20T17:02:29.261003524+00:00"
id: "atelier-7qkf"
issue_type: "task"
labels:
- "refactor"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-djoq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T20:09:53.162271772+00:00"
status: "done"
title: "Extract workflow transition planning and execution from the CLI command module"
updated_at: "2026-06-20T20:09:53.162271772+00:00"
---

## Description

Current state: `crates/atelier-cli/src/commands/workflow.rs` is a large CLI
module that mixes transition option planning, validator evaluation, branch and
review action execution, git dirty-state parsing, workflow output rendering,
and mission-specific helper logic. Mission close/focus collapse and review
validation cleanup will both touch this file.

Desired state: transition planning, validation, action execution, and git/review
preflight logic move behind cohesive app or workflow modules. The CLI command
module is left as a thin adapter for argument handling and human-readable
printing.

Reason: mission behavior should move into workflow decisions, but that will be
hard to review while workflow policy, side effects, and formatting are
interleaved in one command file. Extracting the transition engine clarifies the
boundary between configured workflow behavior and CLI presentation.

Non-scope: this issue should not introduce a Turing-complete workflow language,
new validators, compatibility aliases, or alternate transition semantics.

## Outcome

- Transition option planning and transition execution are callable without
  depending on CLI printing helpers.
- Built-in validator evaluation and action preflight/execution have named
  module boundaries that can be reused by issue, status, and closeout surfaces.
- Mission-specific workflow helpers are either generalized or deleted as part
  of the mission-as-type command collapse.

## Evidence

- Focused workflow tests prove transition options, blocked-transition
  explanations, branch actions, review actions, and close behavior still match
  the existing supported behavior.
- `rg` or code review shows CLI printing no longer owns transition planning or
  side-effect execution.
- `cargo fmt -- --check`, focused Rust tests, `target/debug/atelier lint`, and
  `git diff --check` pass or have recorded follow-up blockers.
