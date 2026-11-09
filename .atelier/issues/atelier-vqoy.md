---
created_at: "2026-11-09T02:02:35.262010794+00:00"
id: "atelier-vqoy"
issue_type: "feature"
labels:
- "forge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
  - kind: "issue"
    id: "atelier-7tul"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fad5"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let merge push the work branch, open its pull request and wait for CI"
updated_at: "2026-11-09T02:02:35.262010794+00:00"
---

## Goal

With a forge in policy, `atelier merge <id>` pushes the work branch and makes sure it has a pull request before checking `ci.passed`, and `--wait` polls CI until it finishes or a timeout passes, so no agent runs `git push` or `gh pr create` by hand. Together with `merge --push` (atelier-fad5), approved work reaches the published base in one command.

## Done when

- [ ] Integration tests with a bare origin and a fake `gh`: `merge <id>` on an approved branch with no pull request pushes the branch, opens one, and refuses with CI's state and the run link while it is pending; a rerun after CI passes merges.
- [ ] `merge <id> --wait` with CI pending then passing merges in one run; with CI failing it refuses naming the run; past its timeout it refuses saying CI is still running.
- [ ] With a fake `gh`, after `merge <id> --push` publishes the base, `merge` checks the pull request reports merged and says so; when it does not, `merge` says the base was published and the pull request is still open, and closes nothing.

## Scope

Provisional until atelier-7tul settles the provider interface; plan it in detail then.

## Unknowns

- How long GitHub takes to show an indirect merge after the push, and so how long `merge` should look before saying the pull request is still open. The real observation is atelier-7o4r's.
- The `--wait` default timeout, given Claude Code's 10-minute command limit and this repository's 12-minute macOS run.
