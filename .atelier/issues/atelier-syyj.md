---
created_at: "2026-11-10T00:56:07.273072817+00:00"
id: "atelier-syyj"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-syyj
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5cuc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add `merge` on the new records, with `--trivial` and no waiting"
updated_at: "2026-11-10T17:05:38.869913537+00:00"
---

## Goal

Finish the loop: an approved item reaches the base in one merge commit that marks it done, after three checks an agent could already see in `status`.

## Done when

- [x] Scenario test, no forge: on an approved item, the new binary's `atelier merge` makes one merge commit on the base in which the item's status is `done`.
- [x] Staleness, each as its own test: after the reviewed commit, a change to a source file, to `.atelier/config.toml`, to another item's directory, or to this item's Goal, Done when or Scope makes `merge` change nothing and say what changed since round 1 and to run `submit`; `status` says the same. A change to this item's Report, or a new review file, does not block the merge.
- [x] A configured merge check that fails on the merged tree stops the merge and prints its command and the end of its output. The config is read from the base's tip: a test removes the check from `config.toml` on the work branch, and it still runs.
- [x] An unanswered entry in the item's Decisions section stops the merge and is named.
- [x] The merge protects itself against inputs that move while its checks run. Each as its own test, using a merge check that waits: the base gains a commit during the checks; the branch gains a commit during the checks; the checkout changes during the checks. In each the merge writes nothing and says to run it again. Two merges of different items started at once never interleave their writes: the second waits for the first or refuses.
- [x] `merge --trivial "<why>"` on a building item with ticked Done when lines and a Report merges with no review and stores the reason in `trivial`. It is refused on an item that has a review file.
- [x] With a forge that names a CI workflow, against the existing test doubles for `gh`: the first run pushes the branch and opens the pull request; while CI has not finished on the head it prints the pull request, changes nothing else, and exits with a status of its own that is neither success nor a refusal. Run again once CI has passed, it merges and pushes the base. Run again after CI failed, it refuses and says so.
- [x] `merge` accepts no flag but `--trivial`, and no code path sleeps or polls.
- [x] The merge commit's message carries the trailer the pre-push hook will recognise an Atelier merge by.
- [x] With a forge that names no CI workflow: one run pushes the branch, opens the pull request, merges and pushes the base.
- [x] The Merge section of `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`crates/atelier-app/src/forge.rs` is copied into the new workspace with one change: it takes the new config's forge section, whose CI workflow is optional, where today it takes `workflow_policy::ForgeConfig` with a required workflow name; its contract tests are copied with it and pass. The merged-tree checkout, the merge commit and the push are extracted from `merge.rs` and `merge_push.rs` onto the config and item types, as copies in the new workspace, trimmed to what the new command calls. Copied with them, from `merge.rs`, `mutation_lock` and `committed_snapshot.rs`: the lock that serialises Atelier's writers, and the confirmation, after the checks and before any write, that the branch tip, the base tip and the checkout are unchanged. The checks of file modes and index flags in that code are not carried over. No file under `crates/` is changed. Their contract tests are rewritten against the new command. Nothing else of today's merge is carried over: not `gate_report`, `review_gate`, the light path or base exemptions. The Git hooks and `ATELIER_SKIP` belong to step D; this item's checks need not honour the skip yet. Today's `merge_wait.rs` and `merge_sync.rs` are not carried over.

## Context

Design: `docs/strategies/redesign-architecture.md`, Merge and Status. Behaviour: `docs/strategies/redesign-product.md`, Merge.

## Report

`atelier merge` is built in `next/`: on an approved item it makes one merge commit on the base that sets the item `done`, after the act's conditions, the merged tree's merge checks and, with a forge, CI on the head. Each Done when line has its test in `next/crates/atelier-cli/tests/merge.rs`, all passing: the plain merge with its trailer (`an_approved_item_merges_in_one_commit_that_marks_it_done`); staleness, one test each for a source file, `config.toml`, another item and this item's Goal, Done when or Scope, with `status` checked against the refusal, and one where a Report change and a new review file still merge; a failing check from the base's config that the branch removed from its own (`a_failing_merge_check_on_the_base_tip_...`); an unanswered question; the base, the branch and the checkout moving during a waiting check, and two merges at once; `--trivial`, and its refusal on an item with a review file; the forge with a CI workflow (not yet, exit 3; failed, refused; passed, merged and pushed) and without one; and `merge_takes_no_flag_but_trivial`. The reviewer who runs `merge` is refused by its role (added to `a_reviewer_that_runs_an_implementers_command_is_told_its_role`). Beyond the Done when: uncommitted changes, a conflict, a merge killed during its checks, and a base push refused then finished by a second run.

### Validation

Run after the last change: `cargo fmt -- --check`, `cargo clippy --all-targets` (no warnings) and `cargo nextest run` in `next/`, 100 tests passed. CI's `next` job steps, run as written by `run_next_job.py` in a scratch clone of the branch (with `crates/` deleted): all passed. `git diff --check` is clean and `git diff --stat master...HEAD -- crates/` is empty.

Each new test was seen to fail for its reason by breaking the code it guards and running it: no lock fails the two-merges test (the second merge ran while the first held the lock); no confirmation after the checks fails all three moving-input tests; no stale condition fails the four staleness tests; reading `config.toml` from the checkout fails the failing-check test; dropping the unanswered, review-file, uncommitted and sweep conditions fails their tests; treating unfinished CI as passed fails the forge test; dropping the trailer fails the plain test.

The new binary driven by hand in a scratch repository with no forge, the base checked out in one worktree and the item worked in another:

```text
$ T3CODE_THREAD_ID=t1 atelier start demo-ab12
Started demo-ab12 on branch demo-ab12.
(t1 writes hello.txt and commits it, ticks Done when, writes the Report)
$ T3CODE_THREAD_ID=t1 atelier submit
Submitted demo-ab12 for review.
Start a reviewer with: Run `atelier review demo-ab12` and follow what it prints.
[exit 0]
$ T3CODE_THREAD_ID=t2 atelier review demo-ab12
You are the reviewer of demo-ab12 (thread t2), round 1.
(brief elided; t2 writes its Summary)
$ T3CODE_THREAD_ID=t2 atelier verdict approve
Recorded your verdict on demo-ab12, round 1: approved. Committed .atelier/items/demo-ab12/reviews/1-0v9d.md with it.
demo-ab12 is back with its implementer, t1, to merge. Your review is done: say so to whoever started you, and stop.
[exit 0]
$ T3CODE_THREAD_ID=t1 atelier status
You are the implementer of demo-ab12 (thread t1).
Item: .atelier/items/demo-ab12/item.md
Title: Say hello
Status: approved
Then: atelier merge
[exit 0]
$ T3CODE_THREAD_ID=t1 atelier merge
Merged demo-ab12 into main with merge commit 8756d2f, which marks it done.
[exit 0]
(in the base checkout, which merge fast-forwarded)
$ T3CODE_THREAD_ID=t1 atelier status
You are thread t1, on the base branch main.
Nothing is ready to start.
[exit 0]
$ git log -1 --format=%B
Merge demo-ab12: Say hello

Merges branch demo-ab12 into main and marks demo-ab12 done.
These merge checks passed on this tree:
- test "$(cat hello.txt)" = hello

Atelier-Merge: demo-ab12
```

The same loop with a forge naming `ci.yml`, the `gh` double from the tests on `PATH`, and `origin` a GitHub URL whose pushes go to a local bare repository:

```text
$ T3CODE_THREAD_ID=t1 atelier merge
Pushed branch demo-ab12 to origin. Pull request #7: https://github.com/owner/repo/pull/7
CI has not finished on 35c770a, the branch's head: no run of it has started. Nothing else changed.
Run `atelier merge` again once ci.yml finishes.
[exit 3]
(CI starts on the head)
$ T3CODE_THREAD_ID=t1 atelier merge
Pushed branch demo-ab12 to origin. Pull request #7: https://github.com/owner/repo/pull/7
CI has not finished on 35c770a, the branch's head: it is in progress (https://github.com/owner/repo/actions/runs/11). Nothing else changed.
Run `atelier merge` again once ci.yml finishes.
[exit 3]
(CI passes)
$ T3CODE_THREAD_ID=t1 atelier merge
Pushed branch demo-ab12 to origin. Pull request #7: https://github.com/owner/repo/pull/7
Merged demo-ab12 into main with merge commit d666245, which marks it done.
Pushed main to origin.
[exit 0]
$ T3CODE_THREAD_ID=t1 atelier merge
demo-ab12 is already merged into main; nothing is left to do on it.
Pushed main to origin.
[exit 0]
```

The pull request was opened once (one POST in the double's log), and the remote's `main` was the merge commit afterwards.

### Copied code

- `crates/atelier-app/src/forge.rs` to `next/crates/atelier-cli/src/forge.rs`, with its URL contract test. Its one interface change: `Github::open` takes the new config's `Forge`, whose `ci_workflow` is optional, and `pull_request_runs` takes the workflow. Trimmed: `newest_push_run`, `has_pull_request`, `pull_request_merged`, `CiRun`'s head and pull request fields, and `ForgeError`'s codes and wording, which moved to `output.rs`. It lives in `atelier-cli` because it runs `gh` and Git, which `atelier-app` does not.
- From `merge.rs`: `MergedCheckout` and its sweep (trimmed: the creation lock and each checkout's own lock file, which the merge lock now covers), the merge commit on the merged tree in a scratch index (folded into `git::commit_file`, which now takes a tree and parents), the fast-forward of the base's checkout with hooks off, and the confirmation after the checks. Not copied: gates, records fences, the light path, retries on a moved base, branch deletion and its config cleanup, the untracked-overwrite check (a refused fast-forward leaves the base as it was and the error says so).
- From `merge_push.rs`: the branch push (now of the checked head, not the ref), finding or opening the pull request, and the base push. Not copied: the upstream setting, the fetch and fast-forward of the local base from the forge, the land-style judging of base commits, the forge-changed checks, and the wait for GitHub to mark the pull request merged, which polled.
- From `mutation_lock.rs`: the exclusive file lock, now std's `File::lock` on `atelier-merge.lock` in the common Git directory, held for the whole merge. Trimmed: the shared form, re-entry and its thread-local bookkeeping. `fs2` is not needed.
- From `committed_snapshot.rs`: only its idea, a checkout whose tracked files match its commit, as `git status`. Its file-mode, index-flag and record-byte checks are not carried over.

### Decisions

- The lock is taken first and held for the whole merge, not only around the writes as today's merge does. A second merge then waits and reads the base the first left, so two approved items merge one after the other instead of the second being refused. The cost is that a merge waits for another's checks; it prints that it is waiting.
- A clean checkout of tracked files is a condition of `merge` in the act table, so `status` lists it. The item's record on its branch is read from the checkout, and merging takes the committed one, so the two must agree.
- An approved item with no approving review in its latest round is refused (`NoApproval`), so a hand-edited status cannot merge.
- The branch is kept after the merge. It is merged, so the base's record is the item's from then on, and deleting it is not in the item.
- Exit status 3 means "not yet". A refusal is 1 and an error 2, as before.

### Surprises

A failing test that held a merge in its waiting check left the merge and its shell running, which hung nextest. The held merge now releases itself when dropped, and the waiting check also ends when the scratch directory goes.

### Open items

- `status` does not list the configured merge checks or CI; they are seen when `merge` runs, so an approved item's `status` says only `Then: atelier merge` when nothing else is unmet.
