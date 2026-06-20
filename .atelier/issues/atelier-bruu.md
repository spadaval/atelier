---
created_at: "2026-06-20T17:02:11.230262584+00:00"
id: "atelier-bruu"
issue_type: "task"
labels:
- "command-surface"
- "refactor"
priority: "P3"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-439j"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T20:22:43.072095942+00:00"
status: "done"
title: "Split issue command implementation out of agent_factory.rs"
updated_at: "2026-06-20T20:22:43.072095942+00:00"
---

## Description

Current state: issue create, show, list, search, dependency, lint, doctor,
export, and rebuild behavior lives in `crates/atelier-cli/src/commands/agent_factory.rs`.
That module name no longer describes the implementation it owns, and the
mission-as-type work will add more issue rendering and creation behavior in the
same area if the boundary is left unchanged.

Desired state: issue command behavior lives behind issue-named CLI modules with
small, cohesive responsibilities such as create/update, show/render, queue/list,
dependency operations, and maintenance. Any remaining Agent Factory-specific
guidance is either removed or kept in a narrowly named module.

Reason: the command-collapse mission should not deepen an obsolete module
boundary. Moving issue behavior out of `agent_factory.rs` makes the mission
type work easier to review and prevents future issue features from hiding
behind Agent Factory terminology.

Non-scope: this issue should not change issue lifecycle behavior, command
names, output contracts, or workflow policy semantics except where a mechanical
module move requires updated paths.

## Outcome

- Root issue command dispatch calls issue-focused modules rather than
  `commands::agent_factory` for normal issue behavior.
- `agent_factory.rs` is deleted or reduced to narrowly scoped Agent Factory
  guidance that is still intentionally public.
- Existing issue create, show, list, search, dependency, lint, doctor, export,
  and rebuild behavior remains covered by the same tests or focused
  replacements.

## Evidence

- `rg "commands::agent_factory" crates/atelier-cli/src/main.rs
  crates/atelier-cli/src/commands` shows no normal issue command dispatch
  through the old module name.
- Focused CLI tests for issue create/show/list/search/dependency behavior pass.
- `cargo fmt -- --check`, focused Rust tests, `target/debug/atelier lint`, and
  `git diff --check` pass or have recorded follow-up blockers.
