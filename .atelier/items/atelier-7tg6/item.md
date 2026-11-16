---
schema: atelier.item
schema_version: 3
id: atelier-7tg6
title: Run the extension's fixture test in CI against the Desk built from the same commit
status: building
mission: atelier-m8ye
labels: []
priority: medium
blocked_by:
- atelier-n4g7
implementers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3A70562b0f-0e5a-45cd-aa86-f4f7bf101722
ready:
  by: 'sumeet, in conversation on 2026-11-16: do this, and the rest of the mission'
  at: 2026-11-16T19:26:24.749789035Z
trivial: null
caused_by: null
created_at: 2026-11-16T19:25:26.849401234Z
---

## Goal

Make a change to the Desk that breaks the extension fail in the pull request
that makes it. The extension's fixture test runs today only by hand, on
Windows, against whichever Desk is live on buildbot.

## Done when

- [ ] A CI job runs the extension's tests on Linux against an `atelier-desk` built from the same commit, with the Desk's app built from `desk/` at that commit. It runs them through one command kept in `t3-extension/`, which runs every test there that needs no real T3 Code, `test-extension.mjs` among them, so a later item adds a test by adding a file and never edits the workflow. The command fails when any test fails and names it.
- [ ] That Desk serves a repository the test owns: records written by the test's setup, or committed as fixtures under `t3-extension/`, with fixed IDs, titles and statuses. No expectation in the test depends on this repository's live records, such as the status of `atelier-plr9`, or on a host named buildbot.
- [ ] The job runs when a change touches `t3-extension/`, `desk/`, `crates/atelier-desk`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.github/workflows/ci.yml` or `scripts/ci-jobs.sh`, and not otherwise; `scripts/ci-jobs.sh` holds the rule and `scripts/ci-jobs.test.sh` covers each. A change to `crates/atelier-core` alone does not run it, on purpose: the fields the extension reads are defined in `atelier-desk`, and the `rust` job covers the core. `t3-extension/README.md` says so. A run whose jobs are all skipped still concludes success.
- [ ] On a pull request that changes only `t3-extension/`, the job finishes in under six minutes, so `atelier merge`'s nine-minute wait holds; the Report gives the timing of three runs and says what is cached.
- [ ] The Report shows the job failing on a branch that renames one field the extension reads from `GET /api/sidebar`, with the run's link, and passing without it.
- [ ] The test still passes on a developer's machine against a Desk named by `DESK`, as it does now, and `t3-extension/README.md` says how to run it both ways.
- [ ] `AGENTS.md` names the job with the others.

## Scope

In: `.github/workflows/ci.yml`, `scripts/ci-jobs.sh` and its test, `t3-extension/test-extension.mjs` and fixtures beside it, `t3-extension/README.md`, `AGENTS.md`.

Out: the contract test, which needs a real T3 Code; Windows and macOS runners; any change to the Desk or to what the extension does.

## Context

On this host the test passes with `BROWSER=/usr/bin/google-chrome DESK=http://127.0.0.1:3780` (26 checks, 2026-11-16); GitHub's `ubuntu-latest` image carries Chrome. `desk/e2e` fakes the Desk's API in the browser and builds no Rust, so this job is the first that needs the server and the app together. `crates/atelier-desk/src/config.rs` holds the configuration, including `embed_origins`, which must list the fixture page's origin.
