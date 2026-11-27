---
schema: atelier.item
schema_version: 3
id: atelier-nw1j
title: Run evals CI job only when evals or CI config change
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- cf983ef7-3317-404a-9c82-373f732b9e07
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T18:13:48.348327737Z
---

## Goal

Stop running CI's `evals` job on `skills/agent-factory/`, `crates/atelier-cli/`,
`crates/atelier-core/`, and Rust workspace config changes. The eval fixtures
call no model, do not inspect skill prose, and are decoupled from normal CLI
and skill edits; run the `evals` CI job only when `evals/`,
`.github/workflows/ci.yml`, or `scripts/ci-jobs.sh` changes.

## Done when

- [x] `needs_evals()` in `scripts/ci-jobs.sh` matches only `evals/*`,
  `.github/workflows/ci.yml`, and `scripts/ci-jobs.sh`.
- [x] `scripts/ci-jobs.test.sh` verifies that skill, CLI, core, and Cargo
  changes skip `evals` while `evals/*`, `scripts/ci-jobs.sh`, and
  `.github/workflows/ci.yml` still run it, and passes.
- [x] Comments in `scripts/ci-jobs.sh`, `.github/workflows/ci.yml`,
  `evals/agent-factory/test.sh`, and `evals/agent-factory/README.md` reflect
  the narrowed trigger.
- [x] `git diff --check` and `atelier check` pass.

## Scope

`scripts/ci-jobs.sh`, `scripts/ci-jobs.test.sh`, `.github/workflows/ci.yml`,
`evals/agent-factory/test.sh`, and `evals/agent-factory/README.md`.

## Report

Narrowed `needs_evals()` in `scripts/ci-jobs.sh` so CI's `evals` job runs only
on changes under `evals/`, `.github/workflows/ci.yml`, or `scripts/ci-jobs.sh`,
removing `skills/agent-factory/*`, `crates/atelier-cli/*`,
`crates/atelier-core/*`, `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`.
Updated `scripts/ci-jobs.test.sh` and the corresponding comments in
`scripts/ci-jobs.sh`, `.github/workflows/ci.yml`, `evals/agent-factory/test.sh`,
and `evals/agent-factory/README.md`.

### Validation

Ran `scripts/ci-jobs.test.sh`, `git diff --check`, and `atelier check`; all
passed.
