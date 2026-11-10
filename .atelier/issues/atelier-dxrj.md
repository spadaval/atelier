---
created_at: "2026-11-10T21:30:47.812872344+00:00"
id: "atelier-dxrj"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-dxrj
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
    id: "atelier-jciw"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jq24"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Settle deferred findings: `list --deferred`, `new --from` and `drop` on a finding"
updated_at: "2026-11-10T22:32:38.206384464+00:00"
---

## Goal

Let deferred findings be settled instead of piling up. A finding a reviewer recommended deferring, or a decider deferred, stays in its review file when its item merges. This item lists the ones nobody has settled and gives the two ways to settle one: take it into a new item, or drop it with a reason.

## Done when

- [x] `list --deferred` prints each deferred finding of a done item that has neither a `Taken:` nor a `Dropped by` line, with a name that identifies the item and the finding in one argument, and the finding's title. It says so when there is none. A deferred finding is one whose recommendation is defer or that a decision deferred.
- [x] `new "<title>" --from <finding>` creates a draft item whose Goal quotes the finding and names where it came from, and writes `Taken: <new item> by <thread>` into the finding, in one run: if either write cannot be made, neither is. `--from` takes the name `list --deferred` prints.
- [x] `drop <finding> --reason "<why>"` writes `Dropped by <thread>: <why>` into the finding. A reason is required.
- [x] Both are refused, with nothing written, for a finding that is already settled, a finding of an item that is not done, a finding that is not deferred, and a name no review has; each refusal says which.
- [x] Both work where they are run. On the base they are planning: no commit, the paths written, and a `Then:` line with the Git command that commits them. On an item's own branch, run by its implementer, they write in that checkout, and the write counts as a change outside the item's own directory: a test settles a finding on the branch of an approved item, and `merge` then says what changed and to run `submit`.
- [x] After either, `list --deferred` no longer shows the finding, and `show` on the finding's item prints the settlement with it.
- [x] The act table gains the row for settling a finding, and the unit test over the table covers it.
- [x] `docs/strategies/redesign-architecture.md` matches what was built.

## Scope

`list --deferred`, `new --from` and `drop` on a finding, in `next/`. The planning commands refuse to run off the base today; these two are the only ones that may run on an item's branch. `escalate` and `decide` are atelier-axfa, built at the same time by another implementer: keep additions to shared files (`main.rs`, `output.rs`, `acts.rs`) to what this item needs, beside the code they extend. Who runs these and when (the gardener's procedure) is the skill's. `log`, which will show a settlement, is mission 4. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records (the `Taken:` and `Dropped by` lines, which `atelier-core` already parses and writes as `Settlement`), the act table's row "`new --from`, `drop` on a finding", and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Commands (Planner), "Where records change" and Review ("Deferred findings stay in their review file"). The planning commands are in `atelier-cli`'s `plan.rs`, `drop` on an item or mission is atelier-jq24's, and `list` is atelier-jciw's.

This is the second of step C's three items in `docs/strategies/redesign-2-core.md`.

## Notes

- 2026-11-10 [note] Handoff: approved by Sol (sol-dxrj-r2, atelier-1hjh) at 610b0031 after two rounds; three round-1 fix-now findings fixed and resolved. Outcome and proof are in the Report.

## Report

Every Done when line holds. `list --deferred` names each unsettled deferred finding as `<item>/<finding>` (`atelier-cli` test `deferred::each_deferred_finding_of_a_done_item_is_listed_until_taken_or_dropped_once`, which also covers `new --from`'s Goal and `Taken:` line, `drop`'s `Dropped by` line and required reason, a finding a Decisions entry deferred, the empty list, the printed commit line and `show` before and after the commit). Each refusal, for both commands, with nothing written: `deferred::settling_is_refused_with_nothing_written_and_says_why`. Neither write without the other: `deferred::new_from_writes_neither_the_item_nor_the_taken_line_when_one_cannot_be_written` (the command runs under `ulimit -f` with SIGXFSZ ignored, so the long review's write fails part way; the review is unchanged, the new item is removed and the command exits 2). A settlement on an approved item's branch, then `merge` naming the changed review file and `Then: atelier submit`, `show` there printing it, a non-implementer refused there, and an item done on the base after the branch was made not listed there: `deferred::a_settlement_on_an_approved_items_branch_is_a_change_its_merge_sends_back_to_review`. The act table's row: `acts::every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused` and `acts::a_finding_is_settled_only_when_its_item_is_done_and_it_is_deferred_and_unsettled`. The deferred rule: `atelier-core` `review::tests::a_finding_is_deferred_by_its_reviewer_or_a_decision_until_a_later_round_carries_it`; where the line goes: `atelier-records` `body::tests::a_settlement_goes_right_after_the_recommendation_where_a_fence_cannot_hide_it`. Each new test was seen to fail under a mutation of the rule it covers (deferred rule, carried findings, insertion point, each refusal, the row's From, the rollback, the branch holder check). `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` (176 tests) pass in `next/`; the CI job's steps (`run_next_job.py`) pass on 6e3fbe1c in a scratch clone.

### Decisions

- A finding across items is named `<item>/<finding>`, as in `atelier-06jc/2-m2bn.1`; `drop` takes a finding when its argument holds a `/`.
- The settlement line is written right after the finding's `Recommendation:` line, the one line the parser guarantees is outside a fence, so a reviewer's unclosed fence cannot hide it.
- `is_deferred` became a free function in `atelier-core` beside `is_open`, replacing `Finding::is_deferred`: a decision defers a finding by a `Decided: defer` line or by a Decisions entry naming it with `Outcome: defer` (an item escalated and decided while approved merges with no later round), and a finding a later round carries is not deferred, so a decided finding carried as `Decided: defer` is listed once, under its latest name.
- `list --deferred` reads the checkout's items, as `new --from` and `drop` do, not the base's last commit as plain `list` does. Reading the base's commit listed, on a branch made before an item merged, findings that `drop` there refuses, and kept showing a finding settled on the base until it was committed. The specification's `list` bullet says so.
- `--deferred` takes no other `list` flag (clap refuses the combination).
- The row is `Act::Settle`, from `done`, role planner; the finding's own conditions (in a review, deferred, not settled) are new `Condition`s checked by `acts::check_settle` after the row, since `Facts` carries no finding and `status` never offers the act. Where it may run (the base, or the branch of a started item the caller holds) is checked in the CLI, like `off_base`.
- `save` in `plan.rs` gained a `save_with` that also writes review files and removes an item directory it created when a later write fails, saying so, or naming what it could not remove. `ReviewFile::write` now writes a file beside the review and renames it over it, so a write that fails part way leaves the review as it was (round 1, atelier-yb6u). The planning output's last sentence names the branch's item when it ran there.
- `show` reads a done item from the base's last commit, as before, and overlays each settlement the checkout holds, so a settlement shows where it was made, before it is committed and on an implementer's branch (round 1, atelier-4e2b).
- A finding heading's number is read in one spelling, as a finding name's is: `## Finding 01:` is refused, so every name `list --deferred` prints can be found by `new --from` and `drop` (round 1, atelier-ureq).

### Surprises

- The item's Context says `atelier-core` already writes `Settlement`; it only parsed it. The writer is `ReviewFile::settle` in `atelier-records`.

### Validation

The new binary driven by hand in a scratch repository, on the base:

```text
  $ atelier list --deferred
  Deferred findings of done items that nobody has settled:
    demo-d0ne/1-aaaa.1: The greeting ignores the locale
    demo-d0ne/1-aaaa.2: Old wording in the help
  Settle each once: take it into a new item, or drop it with a reason.
  Then: atelier new "<title>" --from demo-d0ne/1-aaaa.1, or atelier drop demo-d0ne/1-aaaa.1 --reason "<why>"
  [exit 0]
  $ atelier new Greet in the user's locale --from demo-d0ne/1-aaaa.1
  Created draft item demo-jk1c from finding demo-d0ne/1-aaaa.1: its Goal quotes the finding, and its Done when and Scope are empty to write.
  Marked finding demo-d0ne/1-aaaa.1 taken into demo-jk1c.
  A planner makes demo-jk1c ready with: atelier ready demo-jk1c --by <name>
  Wrote, and did not commit:
    .atelier/items/demo-jk1c/item.md
    .atelier/items/demo-d0ne/reviews/1-aaaa.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/items/demo-jk1c/item.md .atelier/items/demo-d0ne/reviews/1-aaaa.md && git commit -m "Take finding demo-d0ne/1-aaaa.1 into demo-jk1c" -- .atelier/items/demo-jk1c/item.md .atelier/items/demo-d0ne/reviews/1-aaaa.md
  [exit 0]
  $ atelier drop demo-d0ne/1-aaaa.2
  `atelier drop` needs a reason, on one line. Nothing changed.
  Then: atelier drop demo-d0ne/1-aaaa.2 --reason "<why it will not be done>"
  [exit 1]
  $ atelier drop demo-d0ne/1-aaaa.2 --reason The help was rewritten.
  Dropped finding demo-d0ne/1-aaaa.2: The help was rewritten.
  Wrote, and did not commit:
    .atelier/items/demo-d0ne/reviews/1-aaaa.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/items/demo-d0ne/reviews/1-aaaa.md && git commit -m "Drop finding demo-d0ne/1-aaaa.2" -- .atelier/items/demo-d0ne/reviews/1-aaaa.md
  [exit 0]
  $ atelier list --deferred
  No deferred finding of a done item is left to settle.
  [exit 0]
  # refusals
  $ atelier drop demo-d0ne/1-aaaa.1 --reason again
  `atelier drop demo-d0ne/1-aaaa.1` refused; nothing changed.
    [ ] Finding demo-d0ne/1-aaaa.1 is not settled yet (it was taken into demo-jk1c by t9)
  `atelier list --deferred` lists the deferred findings nobody has settled.
  [exit 1]
  $ atelier new Again --from demo-d0ne/1-aaaa.2
  `atelier new --from demo-d0ne/1-aaaa.2` refused; nothing changed.
    [ ] Finding demo-d0ne/1-aaaa.2 is not settled yet (it was dropped by t9: The help was rewritten.)
  `atelier list --deferred` lists the deferred findings nobody has settled.
  [exit 1]
  $ atelier drop demo-d0ne/1-aaaa.3 --reason x
  `atelier drop demo-d0ne/1-aaaa.3` refused; nothing changed.
    [ ] Finding demo-d0ne/1-aaaa.3 is deferred, by its recommendation or by a decision (it is not)
  `atelier list --deferred` lists the deferred findings nobody has settled.
  [exit 1]
  $ atelier drop demo-0pen/1-bbbb.1 --reason x
  `atelier drop demo-0pen/1-bbbb.1` refused; nothing changed.
    [ ] demo-0pen is done (it is in review; only a done item's findings are settled)
  `atelier list --deferred` lists the deferred findings nobody has settled.
  [exit 1]
  $ atelier drop demo-d0ne/1-aaaa.7 --reason x
  `atelier drop demo-d0ne/1-aaaa.7` refused; nothing changed.
    [ ] A review of demo-d0ne has a finding 1-aaaa.7 (none does)
  `atelier list --deferred` lists the deferred findings nobody has settled.
  [exit 1]
  $ atelier new x --from demo-n0ne/1-aaaa.1
  `atelier new --from demo-n0ne/1-aaaa.1` refused; nothing changed.
    [ ] A review of demo-n0ne has a finding 1-aaaa.1 (none does)
  `atelier list --deferred` lists the deferred findings nobody has settled.
  [exit 1]
  $ atelier new x --from demo-d0ne
  `atelier new --from demo-d0ne` refused; nothing changed.
  `demo-d0ne` is not a finding's name, <item>/<review>.<number>, as `atelier list --deferred` prints it.
  [exit 1]
  # commit with the printed lines, then show
  $ sh -c git add -- .atelier/items/demo-d0ne/reviews/1-aaaa.md .atelier/items/*/item.md && git commit -qm Settle
  [exit 0]
  $ atelier show demo-d0ne
  Item demo-d0ne: Greet the user
  Status: done
  Mission: none
  Labels: none
  Priority: high
  Blocked by: none
  Not made ready yet

  ## Goal

  Greet the user.

  ## Done when

  - [x] It works.

  ## Report

  Done.

  Reviews:
    Round 1, review 1-aaaa by t2, of commit abc1234: approved
      1-aaaa.1 (defer, taken into demo-jk1c by t9): The greeting ignores the locale
      1-aaaa.2 (defer, dropped by t9: The help was rewritten.): Old wording in the help
      1-aaaa.3 (no action): A typo
  [exit 0]
```

On the branch of an approved item:

```text
  $ env T3CODE_THREAD_ID=t1 atelier start demo-ab12
  Started demo-ab12 on branch demo-ab12.

  You are the implementer of demo-ab12 (thread t1).
  Item: .atelier/items/demo-ab12/item.md
  Title: Say goodbye
  Status: building
  To submit it for review:
    [ ] Done when: It works.
    [ ] Report: written under `## Report`
  `atelier submit` commits only the item's directory, so commit your change first.
  Then: atelier submit
  [exit 0]
  $ (t1 ticks Done when, writes the Report) atelier submit
  [exit 0]
  $ (t2) atelier review demo-ab12; atelier verdict approve
  Recorded your verdict on demo-ab12, round 1: approved. Committed .atelier/items/demo-ab12/reviews/1-ub5b.md with it.
  demo-ab12 is back with its implementer, t1, to merge. Your review is done: say so to whoever started you, and stop.
  $ env T3CODE_THREAD_ID=t3 atelier drop demo-d0ne/1-aaaa.4 --reason x
  `atelier drop` settles a finding on the base branch main, or on the branch of an item you implement, and this checkout is on branch demo-ab12. Nothing changed.
  Run it in a checkout of main, as after `git switch main`.
  [exit 1]
  $ env T3CODE_THREAD_ID=t1 atelier list --deferred
  Deferred findings of done items that nobody has settled:
    demo-d0ne/1-aaaa.4: Goodbye is hard-coded
  Settle each once: take it into a new item, or drop it with a reason.
  Then: atelier new "<title>" --from demo-d0ne/1-aaaa.4, or atelier drop demo-d0ne/1-aaaa.4 --reason "<why>"
  [exit 0]
  $ env T3CODE_THREAD_ID=t1 atelier drop demo-d0ne/1-aaaa.4 --reason Goodbye moved to the config.
  Dropped finding demo-d0ne/1-aaaa.4: Goodbye moved to the config.
  Wrote, and did not commit:
    .atelier/items/demo-d0ne/reviews/1-aaaa.md
  Once committed, it reaches the base when demo-ab12 merges. It is outside demo-ab12's directory, so a review of demo-ab12 must see it: an approval from before it is stale, and `atelier merge` will say to run `atelier submit`.
  Then: git add -- .atelier/items/demo-d0ne/reviews/1-aaaa.md && git commit -m "Drop finding demo-d0ne/1-aaaa.4" -- .atelier/items/demo-d0ne/reviews/1-aaaa.md
  [exit 0]
  $ sh -c git add -- .atelier/items/demo-d0ne/reviews/1-aaaa.md && git commit -m "Drop finding demo-d0ne/1-aaaa.4" -q -- .atelier/items/demo-d0ne/reviews/1-aaaa.md
  [exit 0]
  $ env T3CODE_THREAD_ID=t1 atelier list --deferred
  No deferred finding of a done item is left to settle.
  [exit 0]
  $ env T3CODE_THREAD_ID=t1 atelier merge
  `atelier merge` refused; nothing changed.
  To merge it:
    [ ] The approval still covers the head: since f8b880d, which round 1 reviewed, nothing changed outside this item's directory, nor its Goal, Done when or Scope (changed: .atelier/items/demo-d0ne/reviews/1-aaaa.md)
  What changed needs a fresh review, so send it back to review.
  Then: atelier submit
  [exit 1]
```

### Open items

- A finding already settled on the base and settled differently on a branch conflicts when the branch merges, as the specification says; no test drives that Git conflict.
