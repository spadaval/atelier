---
created_at: "2026-11-09T04:37:32.233325132+00:00"
id: "atelier-2js4"
issue_type: "task"
labels:
- "ci"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Skip CI's Rust jobs when a change touches nothing they read, cutting ~88 of 90 master-push runs"
updated_at: "2026-11-09T04:37:32.233325132+00:00"
---

## Goal

A Rust build cannot test a Markdown or tracker-record change, but CI runs the full Linux and macOS matrix on every PR and every push to master. Since 2026-10-15, 88 of 90 master-push runs and about one PR in five changed nothing the Rust jobs read. Each CI job should run only when the change touches paths that job tests, while every change still gets a PR, review and a successful CI run.

## Done when

- [ ] A change touching only Markdown outside `crates/` and `docs/strategies/`, or only tracker records, skips the Rust jobs and its CI run concludes success.
- [ ] Any other change, including `docs/strategies/*.md` (parsed by a test), anything under `crates/`, `.atelier/workflow.yaml` and `.github/`, runs the Rust jobs.
- [ ] When the base cannot be determined (new branch, zero `before` SHA, missing commit), the Rust jobs run.
- [ ] The path rule lives in one script with a test that covers each case above.
- [ ] `git diff --check` still runs on every PR and master push.

## Scope

- `.github/workflows/ci.yml` gains a first job that reports, per job, whether that job is relevant (one output per job: `rust` now; later jobs such as Markdown checks or evals add their own).
- The Rust test matrix runs only when `rust` is relevant. A run whose jobs are skipped still concludes success, so `scripts/merge-ci-passed.sh` and the merge policy stay unchanged.
- The `git diff --check` step runs on every change.
- Out of scope: letting any branch other than records-only skip its PR.
