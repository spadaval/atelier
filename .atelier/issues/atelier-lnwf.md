---
created_at: "2026-10-28T00:35:17.734076855+00:00"
id: "atelier-lnwf"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-lnwf
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-28T01:22:22.677838430+00:00"
status: "done"
title: "Run the repository's validation commands in GitHub Actions on Linux and macOS"
updated_at: "2026-10-28T01:22:22.677838430+00:00"
---

## Goal

Add a CI workflow so pull requests and pushes to `master` run the documented validation commands on GitHub, on both platforms the release will ship. Nothing checks a commit today except an agent remembering to.

## Done when

A GitHub Actions run on the pushed work branch passes on ubuntu and macos for: `cargo fmt -- --check`, `cargo nextest run`, `cargo nextest run --features test-hooks`, `target/debug/atelier check`, and `git diff --check` against the base. A run on a throwaway commit that breaks formatting fails. Delivers S1.

## Scope

`.github/workflows/ci.yml`, plus whatever code or test changes macOS or a fresh runner proves necessary. Do not add clippy as a gate (about 20 existing warnings; it is not a documented check). Toolchain comes from `rust-toolchain.toml`.

## Context

Strategy: `docs/strategies/release-distribution.md`. Commands: `AGENTS.md` Command References and `docs/product/development-setup.md`. Tests shell out to Git; runners need a Git identity. Atelier has never been built on macOS.

## Unknowns

Whether tests assume this machine's Git config or Linux-only behavior.

## Close Reason

CI now runs fmt, `atelier check`, nextest (default and test-hooks) and `git diff --check` on ubuntu-latest and macos-latest for pull requests and master pushes. Run 40613374120 passes every step on both OSes, and the formatting probe run 40614244272 fails at `cargo fmt` on both (atelier-oauk, atelier-z0q5).

### Deviations
Scope included test fixes the first runs exposed, which the issue allowed: canonical temp-dir paths in four tests (macOS `/var` → `/private/var`), skipping the non-UTF-8 loose-ref cases on macOS (APFS refuses such names), a 180s nextest limit for the 32-case close-after-approval test, and running the pipefail check under bash.

### Decisions
CI sets `RUST_BACKTRACE: 0` instead of changing the CLI's error printing; the underlying defect is deferred to atelier-eb71. Clippy and a rustc-warnings gate stay out (`build-warnings: warn`). Master pushes get one concurrency group per commit so none is cancelled.

### Surprises
The suite had never run outside this machine. setup-rust-toolchain exports RUST_BACKTRACE, which broke seven exact-output tests; ubuntu-24.04's dash rejects `set -o pipefail` while this machine's newer dash build accepts it; macOS runs the suite about 3× slower (215s vs 80s).

### Open items
- atelier-eb71: backtraces in user-facing errors when RUST_BACKTRACE is set.
- The push-event `git diff --check` path (`github.event.before`) has not run yet; the first master push after merge exercises it.
- macOS timing headroom: some tests reach ~40% of the 60s default limit there.
