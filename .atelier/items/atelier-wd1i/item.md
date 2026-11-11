---
schema: atelier.item
schema_version: 3
id: atelier-wd1i
title: Run the cutover on this host
status: done
mission: atelier-qr5s
labels:
- redesign
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-11T04:46:58.576919592Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:09.915045297Z
---

## Goal

Make the change of tool once, on this host, by running the reviewed runbook. Nothing in this item is decided on the day: the judgment was reviewed in atelier-u1ko, and this item is three commands, what must be true before them, and what shows they worked.

## Done when

- [ ] **Sumeet named the time**, and it is recorded as a dated note on mission atelier-qr5s before anything below starts.
- [ ] **Quiet first, checked before the merge.** `scripts/cutover/cutover.sh preflight`, run from the switch's own checkout, passes: in atelier, astromech, demiurge, life-on-rails and cosplay-studio every worktree is clean and each base is at its remote's tip; no thread other than the cutover's own is running in any of them; and no item is in progress except atelier-u1ko itself, which stays in progress until its merge marks it done, with its implementer and reviewers finished. A stop here is not worked around.
- [ ] **The switch is merged from its own checkout**, with `atelier merge atelier-u1ko --push --wait --timeout 9` under the installed old binary, run again if CI is still pending. If master gained code since the switch's CI ran, bringing it in (`atelier merge --sync`) and its CI run come first, and a sync that conflicts stops the cutover before it starts.
- [ ] **`scripts/cutover/cutover.sh` ran to its end on this host**, straight after that merge, from the worktree that has `master`. Its own preflight passed with no item in progress. Its step 7 passed before its step 8 closed anything: in each repository's main worktree and base worktree `atelier check` passes, `atelier status` runs and the count of items and missions is the conversion's; each base is at its remote's tip; `atelier --version` is the new tool; the skill every agent loads is the new text; `atelier-serve` is stopped and disabled; and CI on atelier's `master` passed on the conversion commit with the tree's binary. The script's closing report is kept in the mission's Result: for each repository the conversion commit, the counts, `--verify` passing, the count of relative links that still resolve, and where each worktree was left.
- [ ] **This item was closed with `atelier close`**, the first act the new tool recorded, by the runbook's last step, after step 7 passed.
- [ ] **If the script stopped at or before step 7:** `scripts/cutover/rollback.sh` was run, and nobody chose by hand which ref to reset or advance: the rollback reads each remote's tip to tell what was published, reverts what was, and brings every real clone to where its remote then is, as atelier-u1ko's trials exercised, among them a push the remote accepted and reported as failed and a stop after one product repository's push. Its result was checked as those trials checked it: where nothing of a repository was pushed, its base is the recorded commit; where something was pushed, its base holds a revert of each pushed commit, its tree is the recorded tree, and local and remote agree; every worktree has checked out what it had; each hooks directory is as it was; the fallback release is installed and its `check` passes; the skill checkout is back; and the old site answers. The mission gets a dated note saying which step failed and what it showed. The cutover is not retried in the same sitting: the failure is diagnosed first.
- [ ] **If something fails after step 7**, the cutover stands. A failed push of the closing commit is retried by running the script again, and anything else is step H's to fix (atelier-2mlh). Nobody chooses on the day between rolling back and going on: step 7 is the line.

## Scope

This item changes no file by hand. The runbook's commits are the only direct pushes, and the mission's Authority allows them. It is never started: do not run `atelier start` on it, since the runbook stops while any item is in progress, and `atelier close` ends an item that was never started. It runs on the base.

Not this item's: fixing what first use breaks and removing the fallback branch of CI's `check` step (atelier-2mlh); the docs (atelier-6w8d); bringing forward a worktree the runbook left on an old-layout branch, which its owner does as `scripts/cutover/README.md` says; and the work host, which Sumeet moves over later with `cutover.sh adopt` and which must not fetch a converted repository before then.

## Context

Mission atelier-qr5s, step G of `.atelier/missions/atelier-qr5s/` (Shape: "the window is short, and Sumeet opens it"; Authority; Risks). Protocol: `docs/strategies/redesign-architecture.md`, Transition, part 4, with its Rollback, as atelier-u1ko rewrites them. The page to read on the day is `scripts/cutover/README.md`, which atelier-u1ko writes: what must be quiet, the commands, how long each step took in the sandbox, what the checkouts look like afterwards, and when to roll back.

Between the switch's merge and the end of the runbook, the tree's binary cannot read this repository's records, and a reinstall from the tree before step 3 would break every repository on the host. Nothing else is started in that window, and it is one thread's work from the preflight to the last line of the report.

From the cutover until the Desk's mission page is live, Sumeet reads a mission with `atelier show <mission>` and its files; there is no brief or debrief page.

## Unknowns

- When no work is in flight in any of the five repositories and no other thread is running in them. Sumeet's naming the time settles it.

## Report

The cutover ran on this host with scripts/cutover/cutover.sh; its report is in the mission's Result.
