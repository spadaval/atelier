---
created_at: "2026-11-04T22:34:51.888718794+00:00"
id: "atelier-ot1c"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-ot1c
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T22:44:08.854272441+00:00"
status: "done"
title: "Gate merges of code into master on a passing pull request CI run"
updated_at: "2026-11-04T22:44:08.854272441+00:00"
---

## Goal

Agents merge non-record changes into `master` only after the work branch has a pull request whose CI run passed, so parallel missions do not turn `master` red. Agents run fmt and the tests covering their slice locally; the full suite on Linux and macOS runs in GitHub Actions.

## Done when

- The three sources state the same rule: push the work branch, open a PR, wait for CI to pass, then `atelier merge` and push `master`; record-only changes are exempt.
- Local validation guidance says agents run targeted tests locally and leave the full cross-platform suite to CI.

## Scope

AGENTS.md Merge Policy, docs/product/repository-contribution.md, docs/architecture/quality/standards.md. No GitHub ruleset: planning records land on `master` directly by design, and a required-check ruleset would refuse those pushes while the admin account bypasses it anyway.

## Close Reason

`merge_checks` now run fmt, `cargo metadata --locked`, the whitespace check and `scripts/merge-ci-passed.sh` instead of both local nextest suites. The script passes only when PR CI passed on the branch's latest code and the base gained no code the branch lacks. Record-only branches pass without CI; `workflow.yaml` and `config.toml` count as code. AGENTS.md, repository-contribution.md, standards.md and development-setup.md state the rule and the targeted-local/full-in-CI split.

### Deviations
- Added `docs/product/development-setup.md` and the policy-files-count-as-code rule after review findings.

### Decisions
- No GitHub ruleset: planning records land on `master` directly by design, a required-check ruleset would refuse them, and the admin account bypasses it anyway.

### Surprises
- `atelier merge` ran both full test suites in a fresh checkout with no build cache on every merge, and the review brief told reviewers to run them too; that was the largest local compute cost.

### Open items
- This branch merges under the old policy, so its merge runs both full local suites once.
