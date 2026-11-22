---
schema: atelier.item
schema_version: 3
id: atelier-1n49
title: Make record-only CI fast, update .atelier/ job filters, and run clippy in CI
status: ready
mission: atelier-ww7w
labels: []
priority: medium
blocked_by: []
implementers: []
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

- [ ] `.github/workflows/ci.yml`'s `changes` job no longer sets up Node/`pnpm` and no longer runs `scripts/t3-extension-release.test.sh`.
- [ ] `scripts/t3-extension-release.test.sh` still runs in CI when `t3-extension/`, `scripts/t3-extension-release*`, `.github/workflows/ci.yml`, or `scripts/ci-jobs.sh` changes (e.g., in the `extension` job or a gated job), and `scripts/ci-jobs.sh` + `scripts/ci-jobs.test.sh` cover that trigger.
- [ ] `scripts/ci-jobs.sh` replaces `.atelier/issues/* | .atelier/reviews/* | .atelier/evidence/*` with `.atelier/items/* | .atelier/missions/*` (keeping `.atelier/config.toml` triggering `rust`), and `scripts/ci-jobs.test.sh` proves both `.md` and non-`.md` files under `.atelier/items/` and `.atelier/missions/` skip `rust` while `.atelier/config.toml` runs `rust`.
- [ ] CI's `test` job runs `cargo clippy --all-targets --locked -- -D warnings`, and `cargo clippy --all-targets --locked -- -D warnings` passes locally on this branch.
- [ ] `scripts/ci-jobs.test.sh` and `scripts/t3-extension-release.test.sh` pass locally.

## Scope

In: `.github/workflows/ci.yml`, `scripts/ci-jobs.sh`, `scripts/ci-jobs.test.sh`, and any Rust lint fixes required for `cargo clippy --all-targets --locked -- -D warnings` to pass cleanly.

Out: `t3-extension/` source code, `desk/` app code, release workflows (`.github/workflows/v-release.yml`, `.github/workflows/t3-extension-release.yml`), and `.atelier/config.toml`.
