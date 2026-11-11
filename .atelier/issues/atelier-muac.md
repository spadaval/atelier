---
created_at: "2026-11-08T07:07:49.458337839+00:00"
id: "atelier-muac"
issue_type: "bug"
labels:
- "agent-factory"
- "setup"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-wgan"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:55.722152438+00:00"
status: "declined"
title: "Bound the update check's wait for output, not only for its process"
updated_at: "2026-11-11T03:55:55.722152438+00:00"
---

## Goal

`freshness::run()` times out only the wait for the child process. A grandchild that keeps stdout or stderr open after the child exits makes `doctor` or the background refresh hang past the 5 s and 10 s limits (review atelier-wgan). No current command triggers it.

## Done when

- [ ] `run()` stops waiting for output at the same deadline, or kills the process group after a normal exit too, and a unit test with `sh -c 'sleep 30 &'` returns within the timeout.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It bounds the update check that `doctor` and the background refresh run, and the What is removed table removes `doctor`; the new binary has no update check (`grep -rli freshness next/crates` finds only an unrelated file).
