---
created_at: "2026-10-28T00:35:17.889723554+00:00"
id: "atelier-7v9q"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-7v9q
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
closed_at: "2026-10-28T01:52:47.287808184+00:00"
status: "done"
title: "Publish tagged releases with dist: Linux and macOS archives, checksums, shell installer"
updated_at: "2026-10-28T01:52:47.287808184+00:00"
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

## Close Reason

dist 0.33.1 is configured and its generated release workflow is committed unedited. A PR-mode run (40616335699) built and uploaded the four Linux and macOS archives, their checksums, sha256.sum, source.tar.gz and atelier-cli-installer.sh without publishing, and the x86_64 Linux binary reports `0.2.0+<commit>` (atelier-0ibu). Package metadata names spadaval/atelier.

### Deviations
Added `/dist-manifest.json` to `.gitignore`. The first PR run printed `0.2.0+363cf20-dirty` because the generated workflow writes that file into the checkout before build.rs checks `git status`. build.rs is unchanged.

### Decisions
- `pr-run-mode = "upload"`: every PR builds every release artifact. A path filter would mean editing the generated workflow.
- `install-path = "CARGO_HOME"`, so an installer upgrade replaces a `cargo install` binary instead of being shadowed by it.
- `install-updater = false`, per the strategy's no-self-update rule.
- Removed the Windows target that `dist init -y` added.

### Surprises
`dist init -y` adds x86_64-pc-windows-msvc even when `-t` names only other targets. Artifacts are named after the package (`atelier-cli-*`), not the binary.

### Open items
- atelier-vy5i: tag only a master commit with green CI (the release workflow does not gate on CI).
- atelier-b99t: build.rs rerun-if-changed path.
- The tag-push publish path is unexercised until the first tag.
- The Linux binaries need glibc 2.35 or newer.
