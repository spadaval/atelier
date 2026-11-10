---
created_at: "2026-11-10T17:36:30.562553857+00:00"
id: "atelier-xtvh"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-xtvh
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
status: "in_progress"
title: "Start CI at `submit` and have `merge` wait up to 9 minutes for it: CI is nearly all of the approval-to-merge wait"
updated_at: "2026-11-10T18:41:39.747175125+00:00"
---

## Goal

Take CI out of the wait between approval and merge. Sumeet decided two changes to the core loop on 2026-11-10, relayed by the mission's planner from the thread "Further cut Agent Factory overhead":

1. Where a forge is configured, `submit` pushes the branch and opens its pull request, so CI runs while the review does. Today CI starts at `merge`, after approval. On the 25 Atelier items merged since 2026-11-09 the median CI run is 5.5 minutes and a quarter take over 16, and CI is nearly all of the approval-to-merge wait.
2. `merge` waits for CI, for a bounded time. This reverses "`merge` never waits". The specification left the waiting to the harness, which watches the pull request. But the thread that runs `merge` is the implementer, almost always a delegated child: T3 does not let a delegated child watch a pull request, it loses wake-ups in delegated chains, and `sleep` is blocked. An implementer told "not yet" can only rerun `merge` in a loop or end its turn with an approved item nobody is driving.

The first change saves nothing unless a CI run started at `submit` still counts at `merge`. `verdict approve` commits a review file, so the head moves past the commit that run tested, and `merge` today asks the forge for runs on the branch tip only (`pull_request_runs(workflow, &head)` in `next/crates/atelier-cli/src/merge.rs`). So the rule for which run counts changes too.

## Done when

- [x] With a forge configured, against a local bare `origin` and the test double for `gh`: `submit` makes its commit, then pushes the branch and opens its pull request, and prints the pull request with the reviewer's handoff line. A later `submit` on the same item pushes again and reuses the pull request. With no forge, `submit` pushes nothing and prints what it prints today.
- [x] A push or a pull request that fails at `submit` prints a warning that names what failed and says `merge` will push; the item is still `in_review` and `submit` exits 0.
- [x] A CI run counts for `merge` when it ran on the head, or on an earlier commit of the branch from which the head differs only inside the item's own directory. The latest finished run among those commits decides. Each as its own test: the workflow passed on the `submit` commit and `verdict approve` added its commit, so `merge` merges without waiting, also while a newer run on the head is still going; the workflow passed on a commit after which a file outside the item's directory changed, so that run does not count; the latest finished run failed and none is running, so `merge` refuses and says CI failed.
- [x] Before it waits, `merge` does two things, each only when it must. It pushes the branch when the remote branch is missing or its tip is not a commit whose run would count, as after a failed push at `submit`; otherwise it pushes nothing until the merge commit, so `merge` never cancels a run that is in progress. And it opens the pull request when the branch has none, whether or not it pushed. Tests: after a `submit` that pushed and opened the pull request, a `merge` run while CI is going leaves the remote branch where it was and opens nothing; after a `submit` whose push failed, `merge` pushes, opens the pull request and then waits; after a `submit` that pushed but could not open the pull request, `merge` opens it without pushing and then waits.
- [x] While no run that counts has finished, or the latest finished one failed and a newer one is running, `merge` says it is waiting for the pull request's CI for up to 9 minutes and that it may be interrupted and run again, and polls. It goes on to merge as soon as the run passes and refuses as soon as it fails. At the bound it exits "not yet", with the exit status and output that has today, and nothing changed since the wait began.
- [x] A poll that fails, as when the network or `gh` errors, counts as not finished: `merge` keeps polling until the bound and then exits "not yet", never a refusal. The bound covers the polls themselves: a test whose `gh` double never returns still sees `merge` exit "not yet" at the bound, with the stalled process ended.
- [x] From the start of the wait to its end `merge` writes nothing and holds no lock. Tests: a `merge` killed during the wait leaves the branch, the base, the checkout and the remote as they were when the wait began; a `merge` of another item, started while the first waits, completes.
- [x] Nothing checked before the wait is trusted after it. Once CI has passed, `merge` takes the lock and checks everything on the tips as they then stand: the config from the base's tip, the approval, the Decisions, that a passing run still counts for the head, and the merge checks on the merged tree. Tests, each with a change made during the wait: the base gains an unrelated commit, and the merge completes on the new base; the base gains a merge check that fails, and the merge refuses with that check's output; the branch gains a commit outside the item's directory, and the merge refuses as stale and says to run `submit`.
- [x] `merge` gains no flag and `config.toml` no option: the bound and the poll interval are constants, and the tests shorten them only through a hook that release builds do not compile in. With a forge that names no CI workflow, `submit` still pushes and opens the pull request, and `merge` waits for nothing.
- [x] `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md` say what was built, each with a new `last_changed`. Product: the `submit` and `merge` rows of the Implementer table, the Merge section's third check, which says CI passed on the head, and its paragraph that says the harness does the waiting, and the "What is removed" row for `merge`'s flags. Architecture: the `submit` row of the act table, "The merge checks pass on the head", the order of the Merge steps (the wait now comes before the lock and the checks) and the paragraph after them, the forge section's account of when the lock is held, the Testing line for `merge` with CI still running, the Tried and removed row "Waiting for CI inside `merge`" (which now says what came back, bounded and with no flag, and why), and the Decisions row "`merge` never waits", replaced by two rows dated 2026-11-10 for these two decisions.

## Scope

`submit`, `merge` and the forge code in `next/`, their tests, and the two specifications. No new flag, config option or dependency: `--wait`, `--timeout` and `--push` do not come back. Nothing is copied from the old workspace: the polling in `merge_wait.rs` served flags this design does not have, and the loop here is written new. No file under `crates/` is changed. `ATELIER_SKIP` belongs to step D. `status` still looks up no CI. The fresh-agent scenario (atelier-67nn) uses a fixture with no forge and does not wait on this item.

## Context

Design: `docs/strategies/redesign-architecture.md`, Merge, The forge and Status. Behaviour: `docs/strategies/redesign-product.md`, Commands and Merge. The item that built `merge` is atelier-syyj; read its Report for what `merge` prints and its exit statuses. This repository's CI workflow cancels a pull request's running CI when a new commit is pushed to it (`concurrency` in `.github/workflows/ci.yml`), so when `merge` pushes a head that is ahead of the remote branch, the run on the older commit may end cancelled; a cancelled run is not a finished pass or fail.

The 9 minutes sit under Claude Code's 10-minute limit on a foreground command. A harness with a shorter limit kills the command mid-wait, which is harmless by the line above.

## Report

All ten Done when lines hold, each proved by tests in `next/crates/atelier-cli/tests/merge.rs`, all of which pass (121 of 121 in `next/`, `cargo nextest run`). `submit` with a forge commits, then pushes and opens the pull request, and pushes again and reuses it on a later `submit` (`with_a_forge_submit_pushes_the_branch_and_opens_its_pull_request`). Without a forge it prints what it printed before (the exact-output check in `scenarios.rs`, `submit_refuses_with_the_status_list_and_then_commits_the_report_in_review`, is unchanged and passes). A failed push or pull request at `submit` is a warning with exit 0 and the item in review (`a_push_that_fails_at_submit_is_a_warning_and_merge_pushes_and_opens_before_it_waits`, `a_pull_request_that_fails_at_submit_is_a_warning_and_merge_opens_it_without_pushing`; these also cover `merge` pushing and opening, or opening without pushing, before it waits). Which runs count: `a_run_that_passed_on_the_submit_commit_counts_after_the_approval` (no wait, a newer run on the head still going, the remote branch left alone), `a_run_on_a_commit_before_a_change_outside_the_items_directory_does_not_count` (also a run on a commit the clone lacks), `when_the_latest_finished_run_failed_and_none_is_running_merge_refuses`, and `a_cancelled_run_is_not_a_finished_one`. The wait: `merge_waits_while_a_newer_run_is_going_and_merges_as_soon_as_it_passes`, `merge_refuses_as_soon_as_the_run_it_waits_for_fails`, `at_the_bound_merge_exits_not_yet_and_changes_nothing`, `a_poll_that_fails_counts_as_not_finished`, `a_gh_that_never_answers_is_ended_at_the_bound` (the waiting notice is out while the first poll is stalled, and the stalled `gh` is gone afterwards). A `gh` that exits but leaves a process holding its output does not hold `merge` past the bound, since its output goes to temporary files rather than pipes (`a_process_gh_leaves_holding_its_output_does_not_hold_up_the_bound`). No lock and no writes during the wait: `a_merge_killed_during_the_wait_leaves_everything_as_it_was`, `another_items_merge_completes_while_one_waits`; and a checkout switched to another approved item during the wait merges neither (`a_checkout_switched_to_another_item_during_the_wait_merges_neither`). Everything judged again under the lock: `a_base_that_gains_a_commit_during_the_wait_is_merged_onto`, `a_merge_check_the_base_gains_during_the_wait_is_run`, `a_branch_that_gains_a_commit_outside_the_item_during_the_wait_is_stale`. No CI workflow: `with_no_ci_workflow_submit_pushes_and_opens_and_merge_waits_for_nothing`. `merge_takes_no_flag_but_trivial` still passes. Each new rule was broken by hand in turn (counting by path, by ancestry, a missing commit, the failed-and-newer-running rule, cancelled as finished, an unconditional push, no pull request when not pushing, an unbounded `gh`, a failed poll refusing, the lock held through the wait, `submit` not pushing, an unbounded wait, no CI check under the lock), and a test failed for that reason each time.

What `merge` prints with a forge: `Pull request #7: <url>` (after `Pushed branch <id> to origin. ` when it pushed), then, before the first look at CI (which may itself take the whole bound), `Waiting up to 9 minutes for ci.yml to finish on pull request #7. Nothing is written while it waits, so this may be interrupted and `atelier merge` run again.`; these go out as they happen. Then the merge lines as before (exit 0), the CI failure as a refusal on stderr starting `` `atelier merge` refused: CI failed: `` (exit 1), or at the bound today's "CI has not finished" text on stdout (exit 3), naming why the forge did not answer when a poll failed. The bound is 9 minutes and the poll every 15 seconds; debug builds shorten both through `ATELIER_TEST_CI_WAIT=<wait ms>,<poll ms>`.

A hand run (first at 2a970de8, run again at 5b6f8fb2 with the same output but for commit IDs) of the new binary, a local bare `origin` (pushes rewritten by `pushInsteadOf`) and the test double for `gh` first on `PATH`; the binary path is shortened to `atelier`:

```text
$ T3CODE_THREAD_ID=implementer-1 atelier submit
Submitted demo-ab12 for review.
Pushed branch demo-ab12 to origin. Pull request #7: https://github.com/owner/repo/pull/7
Start a reviewer with: Run `atelier review demo-ab12` and follow what it prints.
[exit 0]
# HEAD bcfb65c; origin/demo-ab12 is bcfb65c
$ T3CODE_THREAD_ID=reviewer-2 atelier verdict approve --model gpt-6.1-sol
Recorded your verdict on demo-ab12, round 1: approved. Committed .atelier/items/demo-ab12/reviews/1-pz63.md with it.
demo-ab12 is back with its implementer, implementer-1, to merge. Your review is done: say so to whoever started you, and stop.
[exit 0]
# HEAD d2b58ef (the verdict commit); origin/demo-ab12 is still bcfb65c
# the double now answers: run 1 on bcfb65c in_progress
$ ATELIER_TEST_CI_WAIT=20000,2000 T3CODE_THREAD_ID=implementer-1 atelier merge
Pull request #7: https://github.com/owner/repo/pull/7
Waiting up to 20 seconds for ci.yml to finish on pull request #7. Nothing is written while it waits, so this may be interrupted and `atelier merge` run again.
CI has not finished on d2b58ef, the branch's head: it is in progress (https://github.com/owner/repo/actions/runs/1). Nothing else changed.
Run `atelier merge` again once ci.yml finishes.
[exit 3]
# took 20.1s; origin/demo-ab12 is bcfb65c; main is f2c55c4, origin/main f2c55c4
$ ATELIER_TEST_CI_WAIT=120000,2000 T3CODE_THREAD_ID=implementer-1 atelier merge   (in the background)
# after 6s: still running: True; origin/demo-ab12 is bcfb65c
# the double now answers: run 1 on bcfb65c completed/success
Pull request #7: https://github.com/owner/repo/pull/7
Waiting up to 2 minutes for ci.yml to finish on pull request #7. Nothing is written while it waits, so this may be interrupted and `atelier merge` run again.
Merged demo-ab12 into main with merge commit 399abef, which marks it done.
Pushed main to origin.
[exit 0] after 6.2s
# main is 399abef, origin/main 399abef; origin/demo-ab12 is bcfb65c
```

The specifications: product, the `submit` and `merge` rows of the Implementer table, the Merge section's third check and the paragraph after it, and the "What is removed" row for `merge`'s flags. Architecture, the `submit` row of the act table, "The merge checks pass", the Merge steps (now eight, the wait before the lock) and the paragraph after them, the forge section's account of the lock and of what is new, the Testing line, the Tried and removed row, and the Decisions row "`merge` never waits", replaced by two rows dated 2026-11-10. Both have a new `last_changed`.

### Decisions

- `merge` asks the forge for the runs of the branch's pull requests (`branch=<id>`, `event=pull_request`), one call per poll, and judges each run's `head_sha` locally, where the old code asked for the runs of one commit. Asking per commit would cost a call for each commit of the branch on every poll.
- The forge's copy of the branch is read with `git ls-remote` against the remote's push URL, where `merge` pushes, rather than the fetch URL.
- `merge` reads the item and checks the act's conditions, and refuses a conflict, before it pushes or waits, so an unapproved or conflicting item is refused at once rather than after 9 minutes (CI does not run on a conflicting pull request). All of it is read and checked again under the lock.
- After the merge, only the base is pushed, not the branch: the forge's copy of the branch stays at the commit CI ran on, and the merge commit has the head as its second parent.

### Open items

- Asking for runs with `branch=` on the `pull_request` event is how `gh run list --branch` works, but it was not tried against GitHub, since nothing here may reach the network.
- The look at CI under the lock, and `submit`'s pushes and `gh` calls, have no time bound, as before this item; only the wait's polls do. The reviewer's deferred finding atelier-qu4a, on the look under the lock, is filed as atelier-3iyd.
- A poll ended at the bound kills the `gh` process it started, not processes that one started; those are left to end on their own, and no longer hold `merge` up.
- `tempfile` moved from the CLI's dev-dependencies to its dependencies, for `gh`'s output files; it is already in the workspace's dependencies and `Cargo.lock`.

### Review

Three rounds by gpt-6.1-sol. Round 1: atelier-0lkh (the waiting notice could be missed when the first poll stalled), fixed in f1ff5ad4. Round 2: atelier-bywd (a checkout switched to another approved item during the wait merged that item) and atelier-fvc4 (a process left holding `gh`'s output pipes kept `merge` past its bound), both fixed in 5b6f8fb2. Round 3 approved e4fe00ed (atelier-qtpf), with atelier-qu4a deferred to atelier-3iyd.
