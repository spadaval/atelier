---
created_at: "2026-10-22T21:18:32.206690950+00:00"
id: "atelier-x5j3"
issue_type: "bug"
labels:
- "start"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make the start lease write and status refresh safe"
updated_at: "2026-10-22T22:53:59.289189965+00:00"
---

## Goal

Two edge cases in the session lease from atelier-81e5 (finding atelier-jsho, review of atelier-yim7). `start` writes the lease after its records are committed, so a failed lease write reports start as failed although it succeeded. `status` refreshes a lease without checking who holds it, so it can race a `--take-over` and write the old holder back.

## Done when

A failed lease write after a successful start is a warning, not a failure. `status` refreshes only a lease its own worktree holds, rechecked at write time. Tests cover both.

## Notes

- 2026-10-22 [decision] Serialize lease claim and refresh with an flock on leases/.lock, and re-read the holder under it
  Reason: Done-when asks the refresh be rechecked at write time; a plain recheck still leaves a window, whereas claim and refresh under one lock make the holder read and the write atomic against --take-over. A refresh that cannot take the lock does nothing. fs2 moves from dev- to normal dependency of atelier-cli, already in the workspace.
  Rejected: Recheck without a lock (narrower window, not closed); compare-and-swap by rename (no atomic conditional rename)
