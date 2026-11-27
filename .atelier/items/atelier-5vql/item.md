---
schema: atelier.item
schema_version: 3
id: atelier-5vql
title: Drop whitespace check and trim redundant CI steps
status: approved
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- 5ef8378e-a0ad-45ca-9c94-6dca52211f77
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T19:55:07.398765584Z
---

## Goal

Stop failing CI and merge checks on `git diff --check` whitespace nits (such as trailing blank lines in reviewer markdown files), since `cargo fmt`, `pnpm typecheck`, and `atelier check` already cover code and records.

## Done when

- [x] `.github/workflows/ci.yml` no longer runs `git diff --check` in the `changes` job.
- [x] `.atelier/config.toml` no longer lists `git diff --check` as a `merge_check`.
- [x] `AGENTS.md` no longer lists `git diff --check` in its Command References.
- [x] `scripts/ci-jobs.test.sh` and `atelier check` pass.

## Scope

`.github/workflows/ci.yml`, `.atelier/config.toml`, and `AGENTS.md`.

## Report

### What changed

- Removed the `git diff --check` step and its mention in the `changes` job comment in `.github/workflows/ci.yml`.
- Removed the `git diff --check` `[[merge_check]]` entry from `.atelier/config.toml`.
- Removed the `Whitespace: git diff --check` line from `AGENTS.md`.

### Verification

- Ran `scripts/ci-jobs.test.sh` and `atelier check` on the branch.
