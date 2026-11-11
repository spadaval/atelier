---
created_at: "2026-11-11T00:57:39.599582995+00:00"
id: "atelier-efsl"
issue_type: "bug"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-efsl
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-nafc"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T01:15:13.714012887+00:00"
status: "done"
title: "Show only startable items under Ready to start, and have `merge` say its checks ran: two fumbles the rehearsal found"
updated_at: "2026-11-11T01:15:13.714012887+00:00"
---

## Goal

Fix two places where the new binary's output misled on real data. The rehearsal on five converted repositories (atelier-nafc) found them:

- `status` on the base lists every ready item under "Ready to start", each with its unmet conditions, whether or not `start` would accept it. In one converted repository 29 items stood under that heading and one could start. An agent told to run `status` and do what it says reads that heading as work it can pick up.
- `merge` prints nothing about the base's merge checks when they pass, so an agent cannot tell from a successful merge that any check ran.

## Done when

- [x] `status` on the base lists under "Ready to start" only the items `start` would accept now. Ready items that cannot start yet are not listed one by one: one line counts them by cause (a blocker that is not done, a mission that is not ready) and names `atelier list` as where to see them. A test builds one item that can start, one that waits on a blocker and one in a draft mission, and checks the heading holds the first alone and the line counts the other two.
- [x] For a caller that holds no item, the `Then:` line that follows names an item that can start; when no ready item can start, `status` says so and offers no `start` on a ready item. What `status` prints for a caller that holds an item, including the `start` that returns it to that item's branch, is unchanged.
- [x] A `merge` that runs the base's merge checks and makes the merge commit says in one line how many checks from the base's `config.toml` ran on the merged tree and that they passed, before the line that reports the merge. With no check configured it says so in that line's place. The line claims only what happened in that run: when `ATELIER_SKIP` passed a failing check, the skip's own line stands and no line says the checks passed; and a run that finds the item already merged, and only pushes, prints nothing about checks. A test covers each of the four.
- [x] The fresh-agent scenario's fixture still reads as before where one item can start: the `next-loop` grader's tests pass, and the Report shows `status` on that fixture's base.
- [x] `docs/strategies/redesign-architecture.md` and `docs/strategies/redesign-product.md` match what was built where they describe these two outputs.

## Scope

What `status` on the base and a successful `merge` print, in `next/`. No flag, no config option, no change to what `start` or `merge` accept, and no change to `list`. No file under `crates/` is changed.

## Context

The rehearsal's Report (`atelier issue show atelier-nafc --full`) has both findings. The base's `status` is built in `atelier-app`'s `view.rs` and printed from `atelier-cli`'s `output.rs`; `start`'s conditions are in the act table, `atelier-app`'s `acts.rs`, which is what decides whether an item can start. `merge` is `atelier-cli`'s `merge.rs`. Behaviour: `docs/strategies/redesign-product.md`, Commands ("What `status` prints") and Merge.

## Notes

- 2026-11-10 [note] Handoff: approved by gpt-6.1-sol (atelier-37rs) at 06dd907f in one review round, no findings; not merged, the orchestrator merges. Proof and transcripts are in the Report.

## Report

All five Done when lines hold. `status` on the base lists under "Ready to start" only the items `Entry::startable` accepts, which is `start`'s row of the act table as `list --ready` already read it, and counts the rest in one line by cause, naming `atelier list`. `planning::status_on_the_base_lists_only_what_can_start_and_counts_the_rest_by_cause` builds one item that can start, one blocked and one in a draft mission, then has t1 start the first and checks a thread holding nothing gets "No ready item can start now." and no `start`, while t1 still gets `Then: atelier start demo-a1aa (switches to its branch)`. A successful `merge` prints one checks line before "Merged …": `merge::merge_says_how_many_merge_checks_passed_or_that_none_is_configured` covers two checks and none, `merge::an_approved_item_merges_in_one_commit_that_marks_it_done` covers one check and, run again on the merged item, no checks line; `merge::with_a_skip_a_failing_merge_check_or_uncommitted_change_does_not_stop_the_merge` now compares the whole output, so a checks line after a skip fails it (seen failing with the line forced in). The `next-loop` grader's 10 tests pass, and the fixture's base `status` is as the eval recorded. Both specifications describe the two outputs. All passed after the last change: `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo nextest run` in `next/` (225 of 225), and the CI job's steps for `next/` in a scratch clone (`run_next_job.py`, all steps passed).

### Decisions

- The count line names a third cause the Done when does not: a ready item with a branch of its name already, which `start` also refuses (`Condition::BranchExists`). Without it such an item would drop out of both the heading and the count. `scenarios::start_refuses_a_ready_item_whose_blocker_is_not_done_or_whose_branch_exists` checks it.
- An item with two causes is counted under each; the line's first number is the number of items.
- At the orchestrator's request, both specifications' `last_changed` lines were set to the values it gave, in the commit that edits them.

### Transcript

Built from this branch (`next/target/debug/atelier`), each in a scratch repository; the line about Git hooks that opens each `status` is left out.

```text
-- One item can start, one waits on a blocker, one is in a draft mission
$ atelier status    # as t1
You are thread t1, on the base branch main.
Ready to start:
  demo-a1aa (high): Say hello
2 other ready items cannot start yet: 1 waits on a blocker that is not done, 1 is in a mission that is not ready. See them with: atelier list
Then: atelier start demo-a1aa

-- Nothing can start: t1 has started demo-a1aa; t2, then t1, on the base
$ atelier status    # as t2
You are thread t2, on the base branch main.
2 ready items cannot start yet: 1 waits on a blocker that is not done, 1 is in a mission that is not ready. See them with: atelier list
Held by other threads:
  demo-a1aa (building, held by t1): Say hello
No ready item can start now.
$ atelier status    # as t1
You are thread t1, on the base branch main.
You hold:
  demo-a1aa (building): Say hello
2 ready items cannot start yet: 1 waits on a blocker that is not done, 1 is in a mission that is not ready. See them with: atelier list
Then: atelier start demo-a1aa (switches to its branch)

-- The next-loop fixture's base (setup.sh)
$ atelier status    # as t1
You are thread t1, on the base branch main.
Ready to start:
  slug-k3fa (high): Make slugify collapse runs of punctuation into one hyphen
Then: atelier start slug-k3fa

-- merge: two checks configured, both pass
$ atelier merge
Ran 2 merge checks from main's config.toml on the merged tree; each passed.
Merged demo-ab12 into main with merge commit 50a101c, which marks it done.

-- merge: no check configured
$ atelier merge
main's config.toml names no merge check, so none ran.
Merged demo-ab12 into main with merge commit 25c4cb7, which marks it done.

-- merge: a failing check passed with ATELIER_SKIP
$ ATELIER_SKIP="the check is known broken" atelier merge
Skipped with ATELIER_SKIP: The merge check `exit 3` passes (it failed: exit 3): the check is known broken
Merged demo-ab12 into main with merge commit 8255264, which marks it done.

-- merge: run again on the item already merged (two checks configured)
$ atelier merge
demo-ab12 is already merged into main; nothing is left to do on it.
```
