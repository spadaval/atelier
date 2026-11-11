---
schema: atelier.item
schema_version: 3
id: atelier-eb71
title: Keep backtraces out of user-facing errors when RUST_BACKTRACE is set
status: draft
mission: null
labels:
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-28T01:22:01.510371883Z
---

## Goal

`atelier` returns an anyhow `Result` from `main`, so with `RUST_BACKTRACE` (or `RUST_LIB_BACKTRACE`) set, ordinary validation errors print a "Stack backtrace:" after the message. Users with it in their shell see noise, and seven exact-output tests fail locally for such developers. CI works around it with a workflow-level `RUST_BACKTRACE: 0`.

## Done when

- [ ] With `RUST_BACKTRACE=1` exported, `cargo nextest run` passes and an invalid-value error (e.g. `atelier issue list --status bogus`) prints no backtrace; the `RUST_BACKTRACE: 0` env in `.github/workflows/ci.yml` is removed.

## Context

Found in review of atelier-lnwf (CI run 40609860117, `record_refs.rs:383`). Fix in the CLI's error printing, not by clearing the variable in tests only.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lnwf (Run the repository's validation commands in GitHub Actions on Linux and macOS)
