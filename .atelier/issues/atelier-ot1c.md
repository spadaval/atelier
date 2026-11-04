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
status: "in_progress"
title: "Gate merges of code into master on a passing pull request CI run"
updated_at: "2026-11-04T22:34:55.184400529+00:00"
---

## Goal

Agents merge non-record changes into `master` only after the work branch has a pull request whose CI run passed, so parallel missions do not turn `master` red. Agents run fmt and the tests covering their slice locally; the full suite on Linux and macOS runs in GitHub Actions.

## Done when

- The three sources state the same rule: push the work branch, open a PR, wait for CI to pass, then `atelier merge` and push `master`; record-only changes are exempt.
- Local validation guidance says agents run targeted tests locally and leave the full cross-platform suite to CI.

## Scope

AGENTS.md Merge Policy, docs/product/repository-contribution.md, docs/architecture/quality/standards.md. No GitHub ruleset: planning records land on `master` directly by design, and a required-check ruleset would refuse those pushes while the admin account bypasses it anyway.
