---
created_at: "2026-06-20T17:02:20.200248152+00:00"
id: "atelier-n25m"
issue_type: "task"
labels:
- "command-surface"
- "refactor"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-db6z"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T19:31:38.739677822+00:00"
status: "done"
title: "Extract objective status view models from mission and status rendering"
updated_at: "2026-06-20T19:31:38.739677822+00:00"
---

## Description

Current state: `crates/atelier-cli/src/commands/mission.rs`,
`crates/atelier-cli/src/commands/status.rs`, and
`crates/atelier-cli/src/commands/tree.rs` each build their own issue ordering,
blocker summaries, proof-gap summaries, mission/objective progress rows, and
next-action text. The mission collapse will otherwise copy more of this logic
into type-aware issue status views.

Desired state: objective/status snapshot construction is shared behind a
reusable view model. CLI surfaces such as root status, type-aware issue status,
and any retained mission-status replacement render that model instead of
recomputing mission work, blocker, evidence, and readiness state independently.

Reason: the useful part of `mission status` is the status model, not the
mission command namespace. Extracting the model lets the command-collapse work
preserve the useful view while deleting duplicated mission-specific rendering.

Non-scope: this issue should not decide the public command name for the status
view. That contract is owned by `atelier-e071`.

## Outcome

- A shared objective/status view model owns work buckets, blocker summaries,
  proof/evidence gaps, terminal-readiness signals, and next-action inputs.
- Root status and the replacement objective/issue status view render from the
  same model.
- Duplicate local helpers for mission issue collection, blocker ordering,
  proof-gap calculation, and work ordering are deleted or reduced to thin
  adapters.

## Evidence

- Focused tests or transcripts prove root status and the replacement objective
  status view still show ready/blocked work, blockers, evidence gaps, and
  terminal readiness for a representative mission-shaped objective.
- `rg "mission_issue_ids|open_blockers|work_order_row_for_issue|proof_context"
  crates/atelier-cli/src/commands` shows the duplicated mission/status/tree
  helper families were removed or consolidated.
- `cargo fmt -- --check`, focused Rust tests, `target/debug/atelier lint`, and
  `git diff --check` pass or have recorded follow-up blockers.
