---
created_at: "2026-06-15T01:15:02.315449901+00:00"
id: "atelier-m2nh"
issue_type: "validation"
labels:
- "test"
- "validation"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T01:25:13.964604675+00:00"
status: "done"
title: "Validate simplified mission workflow behavior"
updated_at: "2026-06-15T01:25:13.964604675+00:00"
---

## Description

Validate the simplified mission workflow end to end against the repository's active mission state and focused regression coverage.

## Outcome

- Focused tests or command transcripts prove the simplified mission validation workflow accepts proved linked work without requiring duplicate mission proof.
- Validation still proves negative cases: open linked work, open blockers, missing child proof, lint failures, stale projection, or dirty worktree state remain visible closeout blockers.
- The validation record names the exact commands run and the mission/status output lines that demonstrate the new behavior.

## Evidence

- `target/debug/atelier lint` passes.
- Focused `cargo test` or `cargo nextest run` output covers the mission status/closeout behavior changed by this epic.
- Evidence record attached to atelier-m2nh includes the relevant `target/debug/atelier mission status atelier-rxpr` closeout lines.
