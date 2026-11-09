---
created_at: "2026-11-09T02:02:36.297141137+00:00"
id: "atelier-7o4r"
issue_type: "task"
labels:
- "forge"
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
status: "todo"
title: "Run this repository on the one-command merge: retire the hand-run routine from AGENTS.md and the skill"
updated_at: "2026-11-09T07:54:49.392536923+00:00"
---

## Goal

Run this repository on the one-command merge. Now that `atelier merge <id> --push` pushes the branch, opens the pull request, waits for CI and publishes the base (atelier-fad5, atelier-vqoy), rewrite `AGENTS.md`'s Merge Policy and the skill's `skills/agent-factory/references/atelier.md` so agents run it instead of the hand-run routine, delete `scripts/merge-ci-passed.sh`, which no policy runs since atelier-7tul, and prepare this item to be merged with `atelier merge atelier-7o4r --push` alone, so that merge can show the mission's one-command-merge criterion, which validation atelier-zz0t checks.

## Done when

- [ ] `AGENTS.md`'s Merge Policy and `skills/agent-factory/references/atelier.md` name `atelier merge <id> --push` (with `--wait`, run in the background or with `--timeout` where a harness limits foreground commands) as the way work reaches the base, and neither tells agents to run `gh pr create`, wait on checks or push the base by hand. Repositories whose policy names no forge keep `atelier merge <id>` then `atelier land --push`, and the skill says so.
- [ ] `scripts/merge-ci-passed.sh` is deleted, and no current policy, doc or test names it (`git grep -n merge-ci-passed -- . ':!.atelier' ':!docs/adr' ':!docs/strategies'` shows only historical text).
- [ ] `merge --push` can publish from this host: this checkout's `origin` is HTTPS without credentials and agents push over SSH, so either the policy's `forge.remote` or the checkout's push URL makes the push work. The choice and why are a decision note on this item, and `AGENTS.md` says what a fresh checkout needs.
- [ ] The Report says how this item will be merged for the mission's criterion: with `atelier merge atelier-7o4r --push` alone, run with the installed binary built from master after atelier-vqoy merged, and that the orchestrator runs it and quotes the full delivery transcript in a note for atelier-zz0t. The merge itself and its acceptance are atelier-zz0t's to check, after the merge.

## Scope

AGENTS.md, skills/agent-factory/references/atelier.md (and other skill files only where they describe the hand-run routine), scripts/merge-ci-passed.sh, docs that still name it, and this checkout's git config if that is the choice. Moving astromech's policy is separate backlog.

## Context

- The installed binary and the live skill checkout (`~/projects/atelier-skill`) must be moved forward after atelier-vqoy merges, before this item's merge.
- Unknowns from atelier-vqoy that this run observes: how long GitHub takes to mark the pull request merged after the base push (step 8 waits 60 seconds), and whether the 30-minute `--wait` default suits this repository's ~12-minute macOS run. Record what the run shows in the Report; file follow-ups for anything that needs a change.
