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

`atelier` returns an anyhow `Result` from `main`, so with `RUST_BACKTRACE` (or `RUST_LIB_BACKTRACE`) set, ordinary validation errors print a "Stack backtrace:" after the message. Users with it in their shell see noise, and seven exact-output tests fail locally for such developers. CI works around it with a workflow-level `RUST_BACKTRACE: 0`.

## Done when

With `RUST_BACKTRACE=1` exported, `cargo nextest run` passes and an invalid-value error (e.g. `atelier issue list --status bogus`) prints no backtrace; the `RUST_BACKTRACE: 0` env in `.github/workflows/ci.yml` is removed.

## Context

Found in review of atelier-lnwf (CI run 40609860117, `record_refs.rs:383`). Fix in the CLI's error printing, not by clearing the variable in tests only.
