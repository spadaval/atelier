---
schema: atelier.item
schema_version: 3
id: atelier-1n49
title: Make record-only CI fast, update .atelier/ job filters, and run clippy in CI
status: in_review
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Aa7ab9ade-d27f-4fdc-9a10-37446bba3ca2%3Adelegate-task%3Ab9fff5cb-693e-4627-a2af-c8f9d40c5b6d
ready:
  by: Sumeet, in conversation on 2026-11-22
  at: 2026-11-22T05:33:31.467134267Z
trivial: null
caused_by: null
created_at: 2026-11-22T05:30:53.560385164Z
---

## Goal

Make record-only PRs finish CI quickly, remove obsolete 0.4 record paths from `scripts/ci-jobs.sh`, and enforce `cargo clippy` in CI's Rust job.

Today `.github/workflows/ci.yml`'s `changes` job installs Node and `pnpm` and runs `scripts/t3-extension-release.test.sh` on every PR — even one-line `backlog/*` record changes. Meanwhile `scripts/ci-jobs.sh` still excludes the deleted 0.4 paths (`.atelier/issues/*`, `.atelier/reviews/*`, `.atelier/evidence/*`) instead of `.atelier/items/*` and `.atelier/missions/*`, so non-Markdown validation artifacts (like screenshots) trigger the full Rust job. Finally, CI's `test` job omits `cargo clippy`, even though `AGENTS.md` requires it.

Covers `atelier-i70l`.

## Done when

- [x] `.github/workflows/ci.yml`'s `changes` job no longer sets up Node/`pnpm` and no longer runs `scripts/t3-extension-release.test.sh`.
- [x] `scripts/t3-extension-release.test.sh` still runs in CI when `t3-extension/`, `scripts/t3-extension-release*`, `.github/workflows/ci.yml`, or `scripts/ci-jobs.sh` changes (e.g., in the `extension` job or a gated job), and `scripts/ci-jobs.sh` + `scripts/ci-jobs.test.sh` cover that trigger.
- [x] `scripts/ci-jobs.sh` replaces `.atelier/issues/* | .atelier/reviews/* | .atelier/evidence/*` with `.atelier/items/* | .atelier/missions/*` (keeping `.atelier/config.toml` triggering `rust`), and `scripts/ci-jobs.test.sh` proves both `.md` and non-`.md` files under `.atelier/items/` and `.atelier/missions/` skip `rust` while `.atelier/config.toml` runs `rust`.
- [x] CI's `test` job runs `cargo clippy --all-targets --locked -- -D warnings`, and `cargo clippy --all-targets --locked -- -D warnings` passes locally on this branch.
- [x] `scripts/ci-jobs.test.sh` and `scripts/t3-extension-release.test.sh` pass locally.

## Scope

In: `.github/workflows/ci.yml`, `scripts/ci-jobs.sh`, `scripts/ci-jobs.test.sh`, and any Rust lint fixes required for `cargo clippy --all-targets --locked -- -D warnings` to pass cleanly.

Out: `t3-extension/` source code, `desk/` app code, release workflows (`.github/workflows/v-release.yml`, `.github/workflows/t3-extension-release.yml`), and `.atelier/config.toml`.

## Report

A record-only pull request no longer pays for Node, pnpm and the extension's
release test, a screenshot under `.atelier/items/` no longer starts the Rust
job, and clippy now fails CI. All in 63c004d10.

- `changes` keeps checkout, `scripts/ci-jobs.test.sh`, `scripts/ci-jobs.sh`
  and `git diff --check`. In run 44996155457 the removed steps took 18 of
  that job's 26 seconds.
- `scripts/t3-extension-release.test.sh` runs in the `extension` job, which
  `scripts/ci-jobs.sh` now also selects for `scripts/t3-extension-release*`.
- `rust` skips `.atelier/items/*` and `.atelier/missions/*` in place of the
  three 0.4 paths, and still runs for `.atelier/config.toml`.
  `.atelier/runtime/*` stays, as the item did not name it.
- The `test` job runs clippy after `cargo fmt`. It was already clean, so no
  Rust changed.
- `AGENTS.md`, `docs/design.md` and the release test's header now say the
  same. `docs/design.md` had said CI has two jobs; it has four.

One effect outside CI: `t3-extension-release.sh check` reads the extension
rule from `ci-jobs.sh`, so a merge that changes only the release script now
counts as a commit the extension job must have tested. On today's master the
old and new rules both pick 6850e181e, so no release is stranded.

Checks run here:

- `scripts/ci-jobs.test.sh`: passes. Run against master's `ci-jobs.sh`, its
  four new artifact and release-script cases fail.
- `scripts/t3-extension-release.test.sh`: passes, also from a depth-1 clone,
  which is what the `extension` job checks out. It needs a working pnpm
  12.18.0 on PATH. This host's Corepack 0.34 shim cannot start pnpm 12, so
  the runs used the vite-plus pnpm 12.18.0 through PATH.
- `cargo clippy -j 4 --all-targets --locked -- -D warnings`: passes. With a
  temporary `v.len() == 0` in a test and CI's `CARGO_BUILD_WARNINGS=warn`, it
  exits 101.
- `cargo fmt -- --check`, `git diff --check` and `atelier check`: pass.
- The Rust suite was left to CI, as no Rust changed.

Left for the lead:

- `t3-extension/README.md` ("In CI") lists the extension job's paths and
  steps without the release script and its test. This item may not edit
  `t3-extension/`.
- `scripts/ci-jobs.sh` ignores a failed `git diff`: with an unresolvable head
  it prints every job `false` and exits 0. CI passes `HEAD`, so this is
  latent, and it predates this change.
