---
created_at: "2026-10-28T00:35:17.734076855+00:00"
id: "atelier-lnwf"
issue_type: "feature"
labels: []
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
status: "todo"
title: "Run the repository's validation commands in GitHub Actions on Linux and macOS"
updated_at: "2026-10-28T00:35:17.734076855+00:00"
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
