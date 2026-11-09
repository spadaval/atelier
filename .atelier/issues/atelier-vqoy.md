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
status: "todo"
title: "Let merge push the work branch, open its pull request and wait for CI"
updated_at: "2026-11-09T05:08:01.964496971+00:00"
---

## Goal

Build the rest of `merge --push` and `--wait` as `docs/product/reviews-and-merge.md` describes them ("`merge --push`, step by step", steps 2 and 3, and "Waiting for CI with `--wait`"): `--push` pushes the work branch to `forge.remote` and makes sure it has a pull request into the base before the gates, and `--wait` polls CI until it finishes or a timeout passes, so no agent runs `git push` or `gh pr create` by hand. With atelier-fad5's publishing half, approved work reaches the published base in one command.

## Done when

Integration tests with a bare repository as `forge.remote` and a fake `gh` show:
- [ ] `merge <id> --push` on an approved branch with no pull request pushes the branch (never forcing), opens a pull request titled `<id>: <title>` and prints `Pull request #<n>: <url>` saying it opened it, then refuses with `ci_pending` naming the run and its link while CI is pending; a rerun reuses the pull request, and once CI passes merges and publishes. A light-path branch's pull request takes its newest commit subject as the title.
- [ ] Steps 2 and 3 run even when a later gate refuses, such as a missing approval; a rejected branch push refuses with `merge_push_branch_rejected` and the documented `git fetch <remote> <branch> && git merge --no-edit FETCH_HEAD` Next, and pushes no base.
- [ ] `merge <id> --push --wait` with CI pending then passing merges and publishes in one run, printing a line when CI's state changes; with CI failing it refuses with `ci_failed` naming the run; past `--timeout` it refuses with `ci_pending` saying how long it waited, its Next keeping `--push`; a run not started within its grace period refuses with `ci_no_run`. Poll interval and grace period are injectable in tests. `--wait` without `--push` waits, then merges without pushing.
- [ ] `--push`, `--wait` and `--timeout` refuse in combination with `--check`, `--json` or `--sync`, and `--timeout` without `--wait`.
- [ ] `ci.passed`'s Next lines name `atelier merge <id> --push --wait` and `[--push] --wait` as the refusal table documents, the interim hand-step wording is removed from the docs and the code, and `merge`'s `--help` documents `--push`, `--wait` and `--timeout`.

## Scope

crates/atelier-cli/src/commands/merge.rs and main.rs (flags and help), crates/atelier-app/src/forge.rs and ci_passed.rs, and docs/product/reviews-and-merge.md. AGENTS.md, the skill and `scripts/merge-ci-passed.sh` are atelier-7o4r's.

## Unknowns

- How long GitHub takes to show an indirect merge after the push, and so how long `merge` should look before saying the pull request is still open. The real observation is atelier-7o4r's.
- The `--wait` default timeout (documented as 30 minutes), given Claude Code's 10-minute foreground command limit and this repository's 12-minute macOS run. Agents in such harnesses run it in the background or pass `--timeout`; atelier-7o4r's real run shows whether the default needs to change.
