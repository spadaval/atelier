---
schema: atelier.item
schema_version: 3
id: atelier-7tg6
title: Run the extension's fixture test in CI against the Desk built from the same commit
status: in_review
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

- [x] A CI job runs the extension's tests on Linux against an `atelier-desk` built from the same commit, with the Desk's app built from `desk/` at that commit. It runs them through one command kept in `t3-extension/`, which runs every test there that needs no real T3 Code, `test-extension.mjs` among them, so a later item adds a test by adding a file and never edits the workflow. The command fails when any test fails and names it.
- [x] That Desk serves a repository the test owns: records written by the test's setup, or committed as fixtures under `t3-extension/`, with fixed IDs, titles and statuses. No expectation in the test depends on this repository's live records, such as the status of `atelier-plr9`, or on a host named buildbot.
- [x] The job runs when a change touches `t3-extension/`, `desk/`, `crates/atelier-desk`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.github/workflows/ci.yml` or `scripts/ci-jobs.sh`, and not otherwise; `scripts/ci-jobs.sh` holds the rule and `scripts/ci-jobs.test.sh` covers each. A change to `crates/atelier-core` alone does not run it, on purpose: the fields the extension reads are defined in `atelier-desk`, and the `rust` job covers the core. `t3-extension/README.md` says so. A run whose jobs are all skipped still concludes success.
- [x] On a pull request that changes only `t3-extension/`, the job finishes in under six minutes, so `atelier merge`'s nine-minute wait holds; the Report gives the timing of three runs and says what is cached.
- [x] The Report shows the job failing on a branch that renames one field the extension reads from `GET /api/sidebar`, with the run's link, and passing without it.
- [x] The test still passes on a developer's machine against a Desk named by `DESK`, as it does now, and `t3-extension/README.md` says how to run it both ways.
- [x] `AGENTS.md` names the job with the others.

## Scope

In: `.github/workflows/ci.yml`, `scripts/ci-jobs.sh` and its test, `t3-extension/test-extension.mjs` and fixtures beside it, `t3-extension/README.md`, `AGENTS.md`.

Out: the contract test, which needs a real T3 Code; Windows and macOS runners; any change to the Desk or to what the extension does.

## Context

On this host the test passes with `BROWSER=/usr/bin/google-chrome DESK=http://127.0.0.1:3780` (26 checks, 2026-11-16); GitHub's `ubuntu-latest` image carries Chrome. `desk/e2e` fakes the Desk's API in the browser and builds no Rust, so this job is the first that needs the server and the app together. `crates/atelier-desk/src/config.rs` holds the configuration, including `embed_origins`, which must list the fixture page's origin.

## Report

**How I read `DESK`.** The second Done when line forbids any expectation on live records, and it has no exception. So `DESK` can no longer name the buildbot Desk serving this repository's records. It must name a Desk serving the fixture repository. `node t3-extension/fixture-desk.mjs` starts one anywhere this checkout is built and prints its URL. A Windows developer runs that on a Linux host and sets `DESK` to it, as before. The test checks that the Desk serves the fixture repository and stops with a message saying how to start one if it does not. I did not keep a second set of expectations for a live Desk. Dispute this here if "as it does now" meant the live buildbot Desk.

What changed:

- `t3-extension/fixture-desk.mjs` writes a Git repository named `atelier` into a temporary directory. Its records have fixed IDs, titles and statuses:
  - open mission `atelier-fm01`, led by `thread-b`, with a plan review by `remote-thread-0123456789`;
  - items `atelier-fi01` (building on its own branch, implementer `thread-a`), `atelier-fi02` (needs-human) and `atelier-fi03` (done);
  - closed mission `atelier-fm02`;
  - item `atelier-rf01`, in no mission.

  Its origin is `https://example.invalid/fixture/atelier.git` (identity `example.invalid/fixture/atelier`). It serves the repository with `atelier-desk` (`ATELIER_DESK`, else `target/debug/atelier-desk`) and the app (`DESK_APP`, else `desk/dist`) on a free loopback port. The configuration has `fetch_seconds = 0`, no `[t3]` and no forge, and keeps its state in the temporary directory, so it touches no network and no Desk already running on the host.
- `test-extension.mjs` starts that Desk unless `DESK` is set. The first proxy now forwards `GET /api/sidebar` to the real Desk instead of answering it from a fake, and the mission row, its counts, its waiting mark and its threads come from the fixture's records. The conversations name `atelier-rf01` and `atelier-fm02` in place of `atelier-plr9` and `atelier-g961`. On Linux the browser defaults to `/usr/bin/google-chrome`. On failure it also prints the last lines of the browser's log and the Desk's log.
- Some reads are still answered by the proxies, so a Desk change to them fails nothing: `/api/embed`, `/api/launch` (its rows hold IDs no record can have), the unreadable repository added to one `/api/inbox` answer, and the first Desk's 404 for `atelier-fm02`. `t3-extension/README.md` lists them.
- `t3-extension/run-tests.mjs` is the one command. It runs every `test-*.mjs` except `test-contract.mjs`, one at a time, and ends with `N of M tests failed: <names>` and exit status 1. A throwaway `test-zz-throwaway.mjs` that throws gave `1 of 2 tests failed: test-zz-throwaway.mjs` and exit status 1.
- `scripts/ci-jobs.sh` prints a third line, `extension=`, for the eight paths, and `true` when there is no base. `scripts/ci-jobs.test.sh` has 34 cases, all ok. They cover each path, plus these cases that must not run the job: `crates/atelier-core` alone, a nested `crates/a/Cargo.toml`, `scripts/ci-jobs.test.sh` and `release.yml`.
- `.github/workflows/ci.yml` has a new `extension` job. It runs `cargo build --locked -p atelier-desk`, then `pnpm install` and `pnpm build` in `desk/`, then `node t3-extension/run-tests.mjs`, with a 15-minute timeout. When it is skipped, the run still concludes success, like the other skipped jobs.
- `t3-extension/README.md` says how to run the tests both ways, when the job runs and why the core alone does not run it. `AGENTS.md` names `rust`, `desk` and `extension`.

Evidence. Both probe pull requests are drafts against this branch and will be closed. Their branches are `atelier-7tg6-rename-probe` and `atelier-7tg6-timing-probe`, and I left them on origin for a human to delete.

- Locally, `node t3-extension/run-tests.mjs` passes 26 checks in 42 s against the Desk built from this branch. With `SideMission.total` renamed to `count`, it fails with `AssertionError: Fixture missionatelier · 1/undefined done`.
- The rename in CI, #182 (https://github.com/spadaval/atelier/pull/182). It renames `total` to `count` in `GET /api/sidebar`'s missions, regenerates `desk/src/api/` and updates the Desk's own test, so `rust` and `desk` pass. Only the extension job failed, with the same assertion and `1 of 1 tests failed: test-extension.mjs`: https://github.com/spadaval/atelier/actions/runs/43945959570/job/133575975011
- The same job without the rename, on #183 (https://github.com/spadaval/atelier/pull/183), which changes only `t3-extension/run-tests.mjs`. It ran only `changes` and `extension`, and `extension` passed.
- Timing on #183, the extension job from start to end:

  | Run | Time | Rust cache | Cargo build | Tests |
  | --- | --- | --- | --- | --- |
  | Attempt 1 (cold) | 1m44s | none | 29 s | 43 s |
  | Attempt 2 (warm) | 1m33s | restored, full match | 7 s | 52 s |
  | Attempt 3 (warm) | 1m18s | restored, full match | 6 s | 43 s |

  Links: [attempt 1](https://github.com/spadaval/atelier/actions/runs/43945984126/job/133575745975), [attempt 2](https://github.com/spadaval/atelier/actions/runs/43945984126/job/133577563540), [attempt 3](https://github.com/spadaval/atelier/actions/runs/43945984126/job/133578698246).

  What is cached:
  - Swatinem/rust-cache, through `setup-rust-toolchain`, keeps `target/` and the cargo registry under the job's own key, `v0-rust-extension-…`.
  - `setup-node` keeps the pnpm store. It came from the `desk` job's existing cache key.
  - Chrome is in the runner image.

  Even with nothing in the Rust cache, the job takes under two minutes, well inside six.
