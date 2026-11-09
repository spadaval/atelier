---
created_at: "2026-11-09T04:37:32.233325132+00:00"
id: "atelier-2js4"
issue_type: "task"
labels:
- "ci"
workflow_branch:
  branch_base: master
  work_branch: atelier-2js4
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Skip CI's Rust jobs when a change touches nothing they read, about half of master pushes"
updated_at: "2026-11-09T04:39:55.536358938+00:00"
---

## Goal

A Rust build cannot test a Markdown or tracker-record change, but CI runs the full Linux and macOS matrix on every PR and every push to master. Since 2026-10-15, 64 of 130 master pushes touched only tracker records, about 10 more merged a Markdown-only PR, and 10 of 50 merged PRs were Markdown-only. The other 16 Markdown-only pushes were direct strategy commits, which a test parses, so they keep running Rust. Each CI job should run only when the change touches paths that job tests, while every change still gets a PR, review and a successful CI run.

## Done when

- [x] A change touching only Markdown outside `crates/` and `docs/strategies/`, or only tracker records, skips the Rust jobs and its CI run concludes success.
- [x] Any other change, including `docs/strategies/*.md` (parsed by a test), anything under `crates/`, `.atelier/workflow.yaml` and `.github/`, runs the Rust jobs.
- [x] When the base cannot be determined (new branch, zero `before` SHA, missing commit), the Rust jobs run.
- [x] The path rule lives in one script with a test that covers each case above.
- [x] `git diff --check` still runs on every PR and master push.

## Scope

- `.github/workflows/ci.yml` gains a first job that reports, per job, whether that job is relevant (one output per job: `rust` now; later jobs such as Markdown checks or evals add their own).
- The Rust test matrix runs only when `rust` is relevant. A run whose jobs are skipped still concludes success, so the `ci.passed` merge check and the merge policy stay unchanged.
- The `git diff --check` step runs on every change.
- Out of scope: letting any branch other than records-only skip its PR.

## Notes

- 2026-11-08 [decision] Decide which jobs run with a plain shell script (scripts/ci-jobs.sh) whose test runs first in the changes job on every CI run.
  Reason: No Rust build is needed to test it, it runs locally, and it adds one output line per future job (markdown, evals).
  Rejected: workflow-level paths-ignore: GitHub creates no run, so ci.passed finds nothing and refuses the merge
  Rejected: dorny/paths-filter: a third-party action for a rule that is four case patterns, and its glob semantics are harder to test locally
- 2026-11-08 [decision] Records-only changes no longer run atelier check with the tree's binary in CI.
  Reason: atelier land judges committed records with the installed binary before any push, and a records change cannot change what the tree's binary does; the tree's binary still checks records whenever code changes.

## Report

CI now skips its Rust jobs on changes they cannot test, and every change still gets a pull request and a passing run. Probe PR #66 changed only `docs/index.md`. Its run 42563164522 passed `changes`, skipped the test matrix, and concluded success. This PR's own run, 42563164829 on cdc7e018, ran both platforms because it changes `.github/workflows/ci.yml`, and passed. The path rule is in `scripts/ci-jobs.sh`. `scripts/ci-jobs.test.sh` covers 16 cases, one or more for each Done when line: records-only, Markdown at the root and in `docs/`, strategies with and without records, Markdown under `crates/`, the workflow policy, `.github/`, scripts, mixed changes, a rename out of `crates/`, and empty, zero and missing bases. It passes locally and as the first step of `changes` in both runs. I broke the crates/strategies rule, `--no-renames` and the base guard one at a time, and the test failed for the first two. The base guard is redundant, since `git rev-parse` already rejects those bases. `git diff --check` moved into `changes`, which runs on every PR and push.

### Deviations

The Scope named `scripts/merge-ci-passed.sh` as the merge check. `master` now uses the built-in `ci.passed` (ADR 0031), so I reworded that line. Both read only the run's conclusion.

### Decisions

There are two decision notes above. The first covers why this is a shell script rather than `paths-ignore` or a third-party action. The second covers records-only changes, which no longer run `atelier check` with the tree's binary in CI because `atelier land` checks records before every push.

### Open items

- Records-only skipping and the master `push` path have been seen only in the script test, not in a live run. The first record push to `master` after merge will show both.
- If a code push makes `master` red and a records-only or Markdown push follows it, that run is green. Then `merge`'s base note, which reads only the newest `master` run, stops warning about the failure. Code still reaches `master` only after its PR passes, so this needs a bad merge combination first.
- The probe branch `atelier-2js4-probe` is still on the remote. Only the human deletes remote branches.
