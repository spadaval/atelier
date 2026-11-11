---
created_at: "2026-10-28T01:22:01.510371883+00:00"
id: "atelier-eb71"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lnwf"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep backtraces out of user-facing errors when RUST_BACKTRACE is set"
updated_at: "2026-10-28T01:22:01.510371883+00:00"
---

## Goal

CI's workflow sets `RUST_BACKTRACE: 0` (`.github/workflows/ci.yml`, line 21) to keep backtraces out of exact-output tests. The new binary already prints no backtrace with `RUST_BACKTRACE=1` set (checked 2026-11-11 in a scratch clone: an unknown ID and an invalid `--priority` each printed only their message), so the workaround may now hide nothing.

## Done when

- [ ] With `RUST_BACKTRACE=1` exported, the new workspace's tests pass, and the workflow-level `RUST_BACKTRACE: 0` is removed from CI.

## Context

Found in review of atelier-lnwf (CI run 40609860117, `record_refs.rs:383`). Fix in the CLI's error printing, not by clearing the variable in tests only.

Rewritten for the new tool in the cutover sort (atelier-5zre): the error printing it asked for is done in the new binary; what remains is the CI workaround and the test run under the variable.
