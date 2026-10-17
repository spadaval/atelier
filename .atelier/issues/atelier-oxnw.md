---
created_at: "2026-10-17T01:10:57.966699407+00:00"
id: "atelier-oxnw"
issue_type: "feature"
labels:
- "cli-output"
- "start"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make the refused start report one line per blocker"
updated_at: "2026-10-17T01:10:57.966699407+00:00"
---

## Goal

A refused `atelier start sdb0` prints 17 lines (`ensure_transition_allowed` and `print_transition_attempt`, `crates/atelier-cli/src/commands/workflow.rs:89-117,382-398`). The blocker path `atelier-sdb0 -> atelier-bw0b` appears three times (Validators, Blockers, Error), with a doubled `Hint: Next: atelier issue show atelier-bw0b`, a wrong `Command: atelier issue transition atelier-sdb0 start`, and `Description: Start work.`. It never says what atelier-bw0b is or where it stands, so the agent then runs `issue show bw0b`. `issue transition <id> <name>` shows the same report when the transition is blocked.

## Done when

- A blocked transition or start prints one line per blocker with ID, status, title, and branch when it is in flight on a local branch, e.g. `start refused: blocked by atelier-bw0b [todo, in flight on task/atelier-bw0b] "Record ADR 0022…"`.
- The `Error:` line doesn't repeat it.
- `--verbose` keeps the validator detail.
- CLI tests cover a direct blocker and a blocker inherited via the parent.

## Context

atelier-7n5u computes in-flight branches for ready work; reuse it if it has landed.
