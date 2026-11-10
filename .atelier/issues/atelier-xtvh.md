---
created_at: "2026-11-10T17:36:30.562553857+00:00"
id: "atelier-xtvh"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-syyj"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Start CI at `submit` and have `merge` wait up to 9 minutes for it: CI is nearly all of the approval-to-merge wait"
updated_at: "2026-11-10T17:36:30.562553857+00:00"
---

## Goal

Take CI out of the wait between approval and merge. Sumeet decided two changes to the core loop on 2026-11-10, relayed by the mission's planner from the thread "Further cut Agent Factory overhead":

1. Where a forge is configured, `submit` pushes the branch and opens its pull request, so CI runs while the review does. Today CI starts at `merge`, after approval. On the 25 Atelier items merged since 2026-11-09 the median CI run is 5.5 minutes and a quarter take over 16, and CI is nearly all of the approval-to-merge wait.
2. `merge` waits for CI, for a bounded time. This reverses "`merge` never waits". The specification left the waiting to the harness, which watches the pull request. But the thread that runs `merge` is the implementer, almost always a delegated child: T3 does not let a delegated child watch a pull request, it loses wake-ups in delegated chains, and `sleep` is blocked. An implementer told "not yet" can only rerun `merge` in a loop or end its turn with an approved item nobody is driving.

The first change saves nothing unless a CI run started at `submit` still counts at `merge`. `verdict approve` commits a review file, so the head moves past the commit that run tested, and `merge` today asks the forge for runs on the branch tip only (`pull_request_runs(workflow, &head)` in `next/crates/atelier-cli/src/merge.rs`). So the rule for which run counts changes too.

## Done when

- [ ] With a forge configured, against a local bare `origin` and the test double for `gh`: `submit` makes its commit, then pushes the branch and opens its pull request, and prints the pull request with the reviewer's handoff line. A later `submit` on the same item pushes again and reuses the pull request. With no forge, `submit` pushes nothing and prints what it prints today.
- [ ] A push or a pull request that fails at `submit` prints a warning that names what failed and says `merge` will push; the item is still `in_review` and `submit` exits 0.
- [ ] A CI run counts for `merge` when it ran on the head, or on an earlier commit of the branch from which the head differs only inside the item's own directory. The latest finished run among those commits decides. Each as its own test: the workflow passed on the `submit` commit and `verdict approve` added its commit, so `merge` merges without waiting, also while a newer run on the head is still going; the workflow passed on a commit after which a file outside the item's directory changed, so that run does not count; the latest finished run failed and none is running, so `merge` refuses and says CI failed.
- [ ] Before it waits, `merge` pushes the branch and opens the pull request only when it must: the remote branch is missing, or its tip is not a commit whose run would count, as after a failed push at `submit`. Otherwise it pushes nothing until the merge commit, so `merge` never cancels a run that is in progress. Tests cover both: after a `submit` that pushed, a `merge` run while CI is going leaves the remote branch where it was; after a `submit` whose push failed, `merge` pushes and then waits.
- [ ] While no run that counts has finished, or the latest finished one failed and a newer one is running, `merge` says it is waiting for the pull request's CI for up to 9 minutes and that it may be interrupted and run again, and polls. It goes on to merge as soon as the run passes and refuses as soon as it fails. At the bound it exits "not yet", with the exit status and output that has today, and nothing changed since the wait began.
- [ ] A poll that fails, as when the network or `gh` errors, counts as not finished: `merge` keeps polling until the bound and then exits "not yet", never a refusal. The bound covers the polls themselves: a test whose `gh` double never returns still sees `merge` exit "not yet" at the bound, with the stalled process ended.
- [ ] From the start of the wait to its end `merge` writes nothing and holds no lock. Tests: a `merge` killed during the wait leaves the branch, the base, the checkout and the remote as they were when the wait began; a `merge` of another item, started while the first waits, completes.
- [ ] Nothing checked before the wait is trusted after it. Once CI has passed, `merge` takes the lock and checks everything on the tips as they then stand: the config from the base's tip, the approval, the Decisions, that a passing run still counts for the head, and the merge checks on the merged tree. Tests, each with a change made during the wait: the base gains an unrelated commit, and the merge completes on the new base; the base gains a merge check that fails, and the merge refuses with that check's output; the branch gains a commit outside the item's directory, and the merge refuses as stale and says to run `submit`.
- [ ] `merge` gains no flag and `config.toml` no option: the bound and the poll interval are constants, and the tests shorten them only through a hook that release builds do not compile in. With a forge that names no CI workflow, `submit` still pushes and opens the pull request, and `merge` waits for nothing.
- [ ] `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md` say what was built, each with a new `last_changed`. Product: the `submit` and `merge` rows of the Implementer table, the Merge section's third check, which says CI passed on the head, and its paragraph that says the harness does the waiting, and the "What is removed" row for `merge`'s flags. Architecture: the `submit` row of the act table, "The merge checks pass on the head", the order of the Merge steps (the wait now comes before the lock and the checks) and the paragraph after them, the forge section's account of when the lock is held, the Testing line for `merge` with CI still running, the Tried and removed row "Waiting for CI inside `merge`" (which now says what came back, bounded and with no flag, and why), and the Decisions row "`merge` never waits", replaced by two rows dated 2026-11-10 for these two decisions.

## Scope

`submit`, `merge` and the forge code in `next/`, their tests, and the two specifications. No new flag, config option or dependency: `--wait`, `--timeout` and `--push` do not come back. Nothing is copied from the old workspace: the polling in `merge_wait.rs` served flags this design does not have, and the loop here is written new. No file under `crates/` is changed. `ATELIER_SKIP` belongs to step D. `status` still looks up no CI. The fresh-agent scenario (atelier-67nn) uses a fixture with no forge and does not wait on this item.

## Context

Design: `docs/strategies/redesign-architecture.md`, Merge, The forge and Status. Behaviour: `docs/strategies/redesign-product.md`, Commands and Merge. The item that built `merge` is atelier-syyj; read its Report for what `merge` prints and its exit statuses. This repository's CI workflow cancels a pull request's running CI when a new commit is pushed to it (`concurrency` in `.github/workflows/ci.yml`), so when `merge` pushes a head that is ahead of the remote branch, the run on the older commit may end cancelled; a cancelled run is not a finished pass or fail.

The 9 minutes sit under Claude Code's 10-minute limit on a foreground command. A harness with a shorter limit kills the command mid-wait, which is harmless by the line above.
