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

## Notes

- 2026-11-08 [decision] Decide which jobs run with a plain shell script (scripts/ci-jobs.sh) whose test runs first in the changes job on every CI run.
  Reason: No Rust build is needed to test it, it runs locally, and it adds one output line per future job (markdown, evals).
  Rejected: workflow-level paths-ignore: GitHub creates no run, so ci.passed finds nothing and refuses the merge
  Rejected: dorny/paths-filter: a third-party action for a rule that is four case patterns, and its glob semantics are harder to test locally
- 2026-11-08 [decision] Records-only changes no longer run atelier check with the tree's binary in CI.
  Reason: atelier land judges committed records with the installed binary before any push, and a records change cannot change what the tree's binary does; the tree's binary still checks records whenever code changes.
