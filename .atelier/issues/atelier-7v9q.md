---
created_at: "2026-10-28T00:35:17.889723554+00:00"
id: "atelier-7v9q"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lnwf"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Publish tagged releases with dist: Linux and macOS archives, checksums, shell installer"
updated_at: "2026-10-28T00:35:17.889723554+00:00"
---

## Goal

Configure `dist` (axodotdev cargo-dist) so pushing a `v<version>` tag publishes a GitHub Release, and a pull request touching the release setup builds the same artifacts without publishing. Fix package metadata that would mislead a release.

## Done when

`dist plan` lists archives for x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, aarch64-apple-darwin and x86_64-apple-darwin, checksums, and a shell installer. A PR run of the generated release workflow uploads those artifacts, and the Linux x86_64 archive's `atelier --version` prints the workspace version with the build commit. `Cargo.toml`'s `repository` and `homepage` name `https://github.com/spadaval/atelier`. Delivers S2.

## Scope

`dist` config in `Cargo.toml` (or `dist-workspace.toml`), the generated `.github/workflows/release.yml` (unedited where possible), package metadata, `crates/atelier-cli/build.rs` only if CI shows a wrong version suffix. No tag is pushed and nothing is published in this issue.

## Context

Strategy: `docs/strategies/release-distribution.md` (Shape: generated workflow, installer-only upgrades, no Windows/crates.io/Homebrew). dist docs: https://axodotdev.github.io/cargo-dist/. Binary is `atelier` in package `atelier-cli`. `build.rs` emits `ATELIER_VERSION` from `git rev-parse`; its `rerun-if-changed` paths point at `crates/.git`, which does not exist.

## Unknowns

Whether dist's PR mode is `upload` by config, and how a shallow checkout affects the version suffix.
