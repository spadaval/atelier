---
created_at: "2026-11-09T02:02:36.297141137+00:00"
id: "atelier-7o4r"
issue_type: "task"
labels:
- "forge"
workflow_branch:
  branch_base: master
  work_branch: atelier-7o4r
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
  - kind: "issue"
    id: "atelier-fad5"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-vqoy"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Run this repository on the one-command merge: retire the hand-run routine from AGENTS.md and the skill"
updated_at: "2026-11-09T07:55:46.097163745+00:00"
---

## Goal

Run this repository on the one-command merge. Now that `atelier merge <id> --push` pushes the branch, opens the pull request, waits for CI and publishes the base (atelier-fad5, atelier-vqoy), rewrite `AGENTS.md`'s Merge Policy and the skill's `skills/agent-factory/references/atelier.md` so agents run it instead of the hand-run routine, delete `scripts/merge-ci-passed.sh`, which no policy runs since atelier-7tul, and prepare this item to be merged with `atelier merge atelier-7o4r --push` alone, so that merge can show the mission's one-command-merge criterion, which validation atelier-zz0t checks.

## Done when

- [x] `AGENTS.md`'s Merge Policy and `skills/agent-factory/references/atelier.md` name `atelier merge <id> --push` (with `--wait`, run in the background or with `--timeout` where a harness limits foreground commands) as the way work reaches the base, and neither tells agents to run `gh pr create`, wait on checks or push the base by hand. Repositories whose policy names no forge keep `atelier merge <id>` then `atelier land --push`, and the skill says so.
- [x] `scripts/merge-ci-passed.sh` is deleted, and no current policy, doc or test names it (`git grep -n merge-ci-passed -- . ':!.atelier' ':!docs/adr' ':!docs/strategies'` shows only historical text).
- [x] `merge --push` can publish from this host: this checkout's `origin` is HTTPS without credentials and agents push over SSH, so either the policy's `forge.remote` or the checkout's push URL makes the push work. The choice and why are a decision note on this item, and `AGENTS.md` says what a fresh checkout needs.
- [x] The Report says how this item will be merged for the mission's criterion: with `atelier merge atelier-7o4r --push` alone, run with the installed binary built from master after atelier-vqoy merged, and that the orchestrator runs it and quotes the full delivery transcript in a note for atelier-zz0t. The merge itself and its acceptance are atelier-zz0t's to check, after the merge.

## Scope

AGENTS.md, skills/agent-factory/references/atelier.md (and other skill files only where they describe the hand-run routine), scripts/merge-ci-passed.sh, docs that still name it, and this checkout's git config if that is the choice. Moving astromech's policy is separate backlog.

## Context

- The installed binary and the live skill checkout (`~/projects/atelier-skill`) must be moved forward after atelier-vqoy merges, before this item's merge.
- Unknowns from atelier-vqoy that this run observes: how long GitHub takes to mark the pull request merged after the base push (step 8 waits 60 seconds), and whether the 30-minute `--wait` default suits this repository's ~12-minute macOS run. Record what the run shows in the Report; file follow-ups for anything that needs a change.

## Report

Agents in this repository now publish reviewed work with `atelier merge <id> --push --wait` alone. `AGENTS.md`'s Merge Policy names it as the one route to `master`, says not to push branches, open pull requests, wait on checks or push `master` by hand, and says to run it in the background or with `--timeout` under a harness limit. `skills/agent-factory/references/atelier.md` gives both paths: a policy with a `forge` block uses `merge <id> --push --wait`, and one without keeps `atelier merge <id>` then `atelier land --push`. `scripts/merge-ci-passed.sh` is deleted; `git grep -n merge-ci-passed -- . ':!.atelier' ':!docs/adr' ':!docs/strategies'` prints nothing, and `git grep -n -E "gh pr create|gh pr merge|git push" AGENTS.md skills/` prints nothing. This host pushes through `remote.origin.pushurl` set to SSH (decision note); `git remote -v` shows the SSH push URL in this worktree and the master worktree, and a read-only `git ls-remote --heads git@github.com:spadaval/atelier.git master` returned master's head. `AGENTS.md` names that command, or `gh auth setup-git`, for a fresh checkout. No push of any kind was made from this branch.

Merge plan for the mission's criterion: the orchestrator merges this item with `atelier merge atelier-7o4r --push` alone (with `--wait`), run from this work checkout with the installed binary built from master after atelier-vqoy merged (`atelier 0.3.1+4425a012` at the time of writing, master's head on origin). That one command makes the branch push, opens the pull request, waits for CI, merges and pushes `master`; no one pushes or opens the pull request by hand first. The orchestrator quotes the full delivery transcript in a note for atelier-zz0t, including how long GitHub took to mark the pull request merged and how long CI took against the 30-minute `--wait` default (the Context's unknowns). The merge itself and its acceptance are atelier-zz0t's to check, after the merge.

### Deviations

- Beyond the two named files, stale text that described the old routine or named the script was fixed where it lives: `docs/product/repository-contribution.md` (said this policy names no forge yet), `docs/product/reviews-and-merge.md` and `docs/product/workflow-configuration.md` (named the script as an example), the `forge` comment in `.atelier/workflow.yaml` (said origin only names the repository), and a comment in `crates/atelier-cli/tests/forge_ci.rs`. No Rust behaviour changed.
- AGENTS.md now says a record-only branch needs no CI run instead of "skip the pull request": `merge --push` opens a pull request for every branch it pushes, and `ci.passed` passes a record-only branch without a run.

### Decisions

- Push URL, not policy: see the 2026-11-09 decision note.

### Open items

- Other skill files were checked (`grep` for `gh pr`, `git push`, `land --push`, `pull request`, `merge-ci-passed` under `skills/`); none described the hand-run routine, so none changed. `procedures/install.md` still only says to put tests in merge checks when there is no CI; it does not mention naming a forge. Not in scope.
- The live skill checkout (`~/projects/atelier-skill`) must move to master after this merges to carry the new `references/atelier.md`.

## Notes

- 2026-11-09 [decision] merge --push publishes from this host through the checkout's push URL, not the policy: `git config remote.origin.pushurl git@github.com:spadaval/atelier.git` (repository-local, shared by every worktree; fetches stay HTTPS). forge.remote stays `origin`. Verified with `git remote -v` in this worktree and the master worktree, and a read-only `git ls-remote --heads git@github.com:spadaval/atelier.git master`, which returned master's head. AGENTS.md names the one command a fresh checkout needs, or `gh auth setup-git` instead.
  Reason: Credentials belong to the host, not the repository: putting an SSH URL in forge.remote would force SSH on every host and CI-less clone, while the push URL keeps the committed policy portable and fixes only this host.
