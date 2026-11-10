---
created_at: "2026-11-10T19:33:55.282106206+00:00"
id: "atelier-jq24"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-jq24
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
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-10T22:00:37.490505919+00:00"
status: "done"
title: "Add `drop` and `close`, and plan reviews of a mission with `review` and `verdict`"
updated_at: "2026-11-10T22:00:37.490505919+00:00"
---

## Goal

Let work end without being built, and let a plan be reviewed before Sumeet says go: `drop` and `close` for items and missions, and plan reviews of a mission with the same `review` and `verdict` a reviewer already uses on an item.

## Done when

- [x] `drop <id> --reason "<why>"`, run on the base, sets a draft or ready item or mission to `dropped` and stores the reason in the record's body, since the specified front matter has no field for it: as a line `Dropped: <reason>` directly under the heading of an item's Report section, or of a mission's Result section, before any other text or subsection, creating the section when the record has none. A reason is one line. A test drops an item whose Report already has text and a `### Responses` subsection, and the record still validates. It is refused without a reason and on a done record. The reader of that line (the section's first line, when it starts `Dropped: `) is built here, in `atelier-records` beside the other body readers, and this item's `status` and refusals use it. Dropping a mission changes no item; `start` on one of its items is then refused and says the mission was dropped.
- [x] Scenario test, a started item dropped on the base: `drop` writes the base's record whatever the item's branch holds. Once that is committed, `status` from the base and from the item's branch says the item was dropped and why, and `submit`, `merge` and a take-over `start` on it are each refused with the same reason and nothing changed.
- [x] `close <item> "<result>"` sets a draft or ready item with no branch to `done` and writes the result into its Report. It is refused without a result text, and for an item whose branch exists, naming `merge` as the way that item ends.
- [x] `close <mission>` sets a ready mission to `done`. It is refused, listing what is in the way, while any of its items is neither done nor dropped or its Result section is empty.
- [x] Scenario test, a mission from draft to closed: `new --mission`, two items made with `new --in`, `ready` on the mission and on one item, that item's work through `merge`, `close` refused while the other item is a draft, `drop` on it, the Result written, and `close`. A second scenario closes an item with no files on the base with a result.
- [x] `review <mission>` opens a plan review: it creates a review file under `missions/<id>/reviews/`, binds the caller as its reviewer, and prints a brief holding the mission's sections, the paths of its other documents and of its items, the path of the review file and the two `verdict` commands. Run again by the same thread it only prints the brief; run by another thread on a file with no verdict it takes the review over and says who held it. A round with every file's verdict recorded is over, and the next `review` opens the next round, whose file carries the open fix-now findings of the rounds before, as an item's does.
- [x] `verdict approve` or `verdict changes` on a plan review validates the file and commits it and nothing else; it changes no status. `approve` is refused while the file holds an open fix-now finding.
- [x] Scenario test, two blind plan reviews: two threads each run `review <mission>` and `verdict` in their own detached worktree of the same commit, and a planner cherry-picks both commits onto the base with no conflict; both files are then round 1.
- [x] `ready <mission>` prints whether the plan has a review, each latest review's verdict, and whether the mission's directory or any of its items changed since the commit reviewed. It never refuses for any of that.
- [x] `drop` and `close` make no commit: each prints the paths it wrote and a `Then:` line with the Git command that commits them. `verdict` commits, as it does for an item.
- [x] The act table gains the rows for `drop`, `close` on an item, `close` on a mission and `verdict` on a mission review, and the unit test over the table covers them from every status.
- [x] `docs/strategies/redesign-architecture.md` matches what was built: the `Dropped:` line and where a close's result is stored, the plan review's rounds, and the four rows.

## Scope

`drop`, `close`, and `review` and `verdict` on a mission, in `next/`. Dropping a deferred finding is step C. `show`, `list` and `lead` are atelier-jciw, which another implementer builds at the same time in new files of its own (`atelier-app`'s and `atelier-cli`'s view modules); it prints a record's Report or Result as text and a status of `dropped` as a status, and does not read the `Dropped:` line. Keep out of its files, and keep additions to shared files (`main.rs`, `output.rs`) to what this item needs. Nothing is copied from the old workspace. No file under `crates/` is changed.

## Context

Design: `docs/strategies/redesign-architecture.md`, Records, Status and "Where records are read and written". Behaviour: `docs/strategies/redesign-product.md`, Lifecycle (Item, Mission, "When a thread dies"), Commands and Review (Plan reviews). What a plan reviewer judges is the skill's to say, not the brief's. The commands atelier-yptz built (`new`, `set`, `ready`) set the conventions for a planning command: it writes, does not commit, and ends with a `Then:` line.

This is the second of step B's three items in `docs/strategies/redesign-2-core.md`.

## Report

All twelve Done when lines hold, each shown by a test in `next/` that I saw fail with its code mutated: the records readers and writers in `atelier-records` (`a_line_put_first_under_a_heading_is_read_back_as_the_drop_reason`, `a_dropped_record_gives_the_reason_on_the_first_line_of_its_report_or_result`, `a_missions_plan_reviews_are_read_with_it_and_checked_as_an_items_are`, `reads_a_mission_as_a_commit_holds_it_and_none_where_it_has_none`); the act table from every status and the new conditions in `atelier-app/tests/acts.rs` (`every_act_in_the_table_from_every_status_sets_the_specified_status_or_is_refused`, `closing_a_mission_needs_each_item_done_or_dropped_and_its_result_written`, `closing_an_item_needs_no_branch_named_for_it`, `a_plan_verdict_counts_its_own_file_and_earlier_rounds_not_a_blind_one`); and the scenarios in `atelier-cli/tests/planning.rs`, driving the binary: `drop_puts_its_reason_first_in_the_report_and_needs_a_reason_and_a_record_not_done` (Report with text and Responses, still valid), `a_dropped_mission_changes_no_item_and_start_on_one_says_it_was_dropped`, `a_started_item_dropped_on_the_base_is_refused_everywhere_with_its_reason`, `close_ends_an_item_with_no_branch_with_its_result_and_names_merge_for_a_started_one` (the item with no files), `a_mission_goes_from_draft_through_its_items_to_closed`, `review_of_a_mission_opens_a_plan_review_and_its_verdict_commits_only_that_file` and `two_blind_plan_reviews_cherry_pick_onto_the_base_and_ready_says_what_they_found`. All 158 tests pass, with `cargo fmt --check` and clippy clean, and the CI job's steps for `next/` pass on a scratch clone at 301d9a37. `docs/strategies/redesign-architecture.md` describes the `Dropped:` line, where a close's result goes, the plan review's rounds, where plan reviews are written and the new rows.

### Validation

The new binary driven by hand in a scratch repository (Git's commit lines left out): a mission from draft to closed, an item with no files closed on the base, a started and approved item dropped on the base and then refused at `merge`, `submit` and a take-over, and two blind plan reviews cherry-picked onto the base.

```text
  ### 1. A mission from draft to closed
  $ atelier new --mission "Greet everyone"
  Created draft mission demo-agpp, with empty sections to write.
  Its items are created with: atelier new "<title>" --in demo-agpp
  A planner makes it ready with: atelier ready demo-agpp --by <name>
  Wrote, and did not commit:
    .atelier/missions/demo-agpp/mission.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/missions/demo-agpp/mission.md && git commit -m "Plan mission demo-agpp" -- .atelier/missions/demo-agpp/mission.md

  (created items demo-z1r2 and demo-7r99 in mission demo-agpp with atelier new --in, and committed each)

  $ T3CODE_THREAD_ID=p1 atelier ready demo-agpp demo-z1r2 --by sumeet   # in repo
  Made ready, as sumeet said: demo-agpp, demo-z1r2.
  Mission demo-agpp's plan has no review.
  Wrote, and did not commit:
    .atelier/items/demo-z1r2/item.md
    .atelier/missions/demo-agpp/mission.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/items/demo-z1r2/item.md .atelier/missions/demo-agpp/mission.md && git commit -m "Ready demo-agpp, demo-z1r2" -- .atelier/items/demo-z1r2/item.md .atelier/missions/demo-agpp/mission.md
  (exit 0)

  $ T3CODE_THREAD_ID=t1 atelier start demo-z1r2   # in work
  Started demo-z1r2 on branch demo-z1r2.

  You are the implementer of demo-z1r2 (thread t1).
  Item: .atelier/items/demo-z1r2/item.md
  Title: Say hello
  Status: building
  To submit it for review:
    [ ] Done when: hello.txt says hello.
    [ ] Report: written under `## Report`
  `atelier submit` commits only the item's directory, so commit your change first.
  Then: atelier submit
  (exit 0)

  (t1 submitted; t2 reviewed and approved)
  $ T3CODE_THREAD_ID=t1 atelier merge   # in work
  Merged demo-z1r2 into main with merge commit b2307f7, which marks it done.
  (exit 0)

  $ T3CODE_THREAD_ID=p1 atelier close demo-agpp   # in repo
  `atelier close demo-agpp` refused; nothing changed.
  To close it:
    [ ] demo-7r99 is done or dropped (it is draft)
    [ ] Result: written under `## Result`
  Then: atelier close demo-agpp
  (exit 1)

  $ T3CODE_THREAD_ID=p1 atelier drop demo-7r99 --reason Nobody leaves   # in repo
  Dropped item demo-7r99: Nobody leaves
  Wrote, and did not commit:
    .atelier/items/demo-7r99/item.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/items/demo-7r99/item.md && git commit -m "Drop demo-7r99" -- .atelier/items/demo-7r99/item.md
  (exit 0)

  $ git add -- .atelier/items/demo-7r99/item.md && git commit -qm 'Drop demo-7r99' -- .atelier/items/demo-7r99/item.md

  $ T3CODE_THREAD_ID=p1 atelier close demo-agpp   # in repo
  Closed mission demo-agpp: it is done.
  Wrote, and did not commit:
    .atelier/missions/demo-agpp/mission.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/missions/demo-agpp/mission.md && git commit -m "Close demo-agpp" -- .atelier/missions/demo-agpp/mission.md
  (exit 0)

  $ git add -- .atelier/missions/demo-agpp/mission.md && git commit -qm 'Close demo-agpp' -- .atelier/missions/demo-agpp/mission.md

  ### 1b. An item with no files closed on the base
  $ T3CODE_THREAD_ID=p1 atelier close demo-y3bj   # in repo
  `atelier close demo-y3bj` needs the item's result, the text its Report gets. Nothing changed.
  Then: atelier close demo-y3bj "<what came of it>"
  (exit 1)

  $ T3CODE_THREAD_ID=p1 atelier close demo-y3bj Nothing to change: hello.txt already says hello.   # in repo
  Closed item demo-y3bj: it is done.
  Wrote, and did not commit:
    .atelier/items/demo-y3bj/item.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/items/demo-y3bj/item.md && git commit -m "Close demo-y3bj" -- .atelier/items/demo-y3bj/item.md
  (exit 0)

  $ tail -4 .atelier/items/demo-y3bj/item.md

  ## Report

  Nothing to change: hello.txt already says hello.

  ### 2. A started item dropped on the base, then refused at merge
  $ T3CODE_THREAD_ID=t1 atelier start demo-he3l   # in work2
  Started demo-he3l on branch demo-he3l.

  You are the implementer of demo-he3l (thread t1).
  Item: .atelier/items/demo-he3l/item.md
  Title: Wave
  Status: building
  Write its Goal and its Done when lines in .atelier/items/demo-he3l/item.md first: they say what the change must do, and the reviewer judges it against them.
  To submit it for review:
    [ ] Report: written under `## Report`
  `atelier submit` commits only the item's directory, so commit your change first.
  Then: atelier submit
  (exit 0)

  (t1 submitted; t2 approved it)

  $ T3CODE_THREAD_ID=p1 atelier drop demo-he3l --reason The site waves instead   # in repo
  Dropped item demo-he3l: The site waves instead
  Its branch demo-he3l is left as it is. Every command reads the base's record first, so once this is committed nobody can submit, merge or take it over.
  Wrote, and did not commit:
    .atelier/items/demo-he3l/item.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/items/demo-he3l/item.md && git commit -m "Drop demo-he3l" -- .atelier/items/demo-he3l/item.md
  (exit 0)

  $ git add -- .atelier/items/demo-he3l/item.md && git commit -qm 'Drop demo-he3l' -- .atelier/items/demo-he3l/item.md

  $ T3CODE_THREAD_ID=t3 atelier status   # in repo
  You are thread t3, on the base branch main.
  Dropped on the base while started; nothing is left to do on them:
    demo-he3l: The site waves instead
  Nothing is ready to start.
  (exit 0)

  $ T3CODE_THREAD_ID=t1 atelier status   # in work2
  You are thread t1, on the branch of demo-he3l.
  Title: Wave
  Status: dropped
  It was dropped on the base: The site waves instead
  Nothing is left to do on it.
  (exit 0)

  $ T3CODE_THREAD_ID=t1 atelier merge   # in work2
  demo-he3l is dropped, and `atelier merge` runs on an item that is approved. Nothing changed.
  It was dropped on the base: The site waves instead
  Nothing is left to do on it.
  (exit 1)

  $ T3CODE_THREAD_ID=t1 atelier submit   # in work2
  demo-he3l is dropped, and `atelier submit` runs on an item that is building, changes requested, approved. Nothing changed.
  It was dropped on the base: The site waves instead
  Nothing is left to do on it.
  (exit 1)

  $ T3CODE_THREAD_ID=t2 atelier start demo-he3l   # in repo
  demo-he3l is dropped, and `atelier start` runs on an item that is ready. Nothing changed.
  It was dropped on the base: The site waves instead
  Nothing is left to do on it.
  (exit 1)

  ### 3. Two blind plan reviews, cherry-picked onto the base
  $ T3CODE_THREAD_ID=r1 atelier review demo-whru   # in rev-a
  You are a reviewer of the plan of mission demo-whru (thread r1), round 1.
  Review the plan as commit 3eb7b57 holds it, as someone who did not write it: the mission below, its other documents and its items. Read them, and write down each problem you find. Another reviewer may review the same plan blind, in a checkout of its own.

  Mission demo-whru: Say farewell

  Its other documents:
    .atelier/missions/demo-whru/design.md
  Its items:
    .atelier/items/demo-pud2/item.md (draft): Print farewell

  Your review file: .atelier/missions/demo-whru/reviews/1-izp9.md
  Write your review there, and change no other file. Leave its front matter, between the `---` lines, as it is.
  Under `## Summary`, say what you checked and how, and what you could not check. Then add a section for each problem, numbered on from 1:

  ## Finding 1: <a one-line title>
  Recommendation: fix now
  <What is wrong, where (file and line), and how you know.>

  The Recommendation is `fix now` (it must be fixed before the mission starts), `defer` (real, but for later work) or `no action` (a note; nothing to do).

  When the file is written, record your verdict. It checks the file, then commits it alone, changing no status:
    atelier verdict changes --model <your model>   if the file holds an open fix-now finding
    atelier verdict approve --model <your model>   if it holds none
  Give the name of the model you run on, such as gpt-6.1-sol, or leave out `--model` if you do not know it. Then tell whoever started you the commit it prints, which the planner cherry-picks onto main. Your review is then done.
  (exit 0)

  $ T3CODE_THREAD_ID=r1 atelier verdict approve   # in rev-a
  Recorded your verdict on the plan of mission demo-whru, round 1: approved. Committed .atelier/missions/demo-whru/reviews/1-izp9.md alone, in commit 8766cb12f3d5fedb803328c0feaa8efaacbf5b25.
  The planner puts it on main with: git cherry-pick 8766cb12f3d5fedb803328c0feaa8efaacbf5b25
  Your review is done: tell whoever started you that commit, and stop.
  (exit 0)

  (r2 ran atelier review demo-whru in rev-b and wrote a fix-now finding into .atelier/missions/demo-whru/reviews/1-kvpl.md)
  $ T3CODE_THREAD_ID=r2 atelier verdict approve   # in rev-b
  `atelier verdict approve` refused; nothing changed.
  To record your verdict:
    [ ] No fix-now finding is open; 1-kvpl.1 is: No sizing
  A finding that stands needs `atelier verdict changes`. A carried one closes when your file says `Now: fixed` or `Now: withdrawn` for it.
  Then: atelier verdict approve (or `atelier verdict changes`, once your file holds a fix-now finding)
  (exit 1)

  $ T3CODE_THREAD_ID=r2 atelier verdict changes --model gpt-6.1-sol   # in rev-b
  Recorded your verdict on the plan of mission demo-whru, round 1: changes requested. Committed .atelier/missions/demo-whru/reviews/1-kvpl.md alone, in commit 55493ad673f38e5ebafea0e9d468ad2962479076.
  The planner puts it on main with: git cherry-pick 55493ad673f38e5ebafea0e9d468ad2962479076
  Your review is done: tell whoever started you that commit, and stop.
  (exit 0)

  $ git cherry-pick 8766cb12f3d5fedb803328c0feaa8efaacbf5b25 55493ad673f38e5ebafea0e9d468ad2962479076

  $ ls .atelier/missions/demo-whru/reviews/
  1-izp9.md
  1-kvpl.md

  $ T3CODE_THREAD_ID=p1 atelier ready demo-whru --by sumeet   # in repo
  Made ready, as sumeet said: demo-whru.
  Mission demo-whru's plan reviews, round 1:
    1-izp9 by r1: approved, of commit 3eb7b57
    1-kvpl by r2: changes requested, of commit 3eb7b57
  Since 3eb7b57, nothing in the mission's directory or its items changed.
  Wrote, and did not commit:
    .atelier/missions/demo-whru/mission.md
  Planning counts once it is committed on the base.
  Then: git add -- .atelier/missions/demo-whru/mission.md && git commit -m "Ready demo-whru" -- .atelier/missions/demo-whru/mission.md
  (exit 0)
```

### Deviations

- Five rows, not four: `verdict` on a plan review is two rows, `verdict changes` and `verdict approve`, as on an item, since each has its own condition. The architecture specification's table now has both.
- The specification's `drop` row said "any but done". `drop` is also refused on a dropped record, since there is nothing left to end and a second reason would have nowhere to go; the table now says `draft`, `ready` and the started statuses.
- The `Dropped:` reader takes the section's first line that is not blank, so a Report written by hand as `## Report`, a blank line, then `Dropped: ...` reads the same as the one `drop` writes.

### Decisions

- `drop` writes the base's record, the one in the base's checkout. An item only on its branch, as after `start --new`, gets its branch's front matter and its Goal, Done when and Scope written on the base, with the reason as its Report; its reviews, Responses and Decisions stay on the branch, so the base's record names no review the base lacks.
- A take-over `start` is refused when the item's mission is dropped at the base's tip, as a plain `start` is.
- `status` on the base lists the items dropped there while their branch still exists, with each reason; this is where "status from the base says it was dropped and why" lands. Other dropped items are not listed.
- `review <mission>` runs anywhere except on an item's branch, where `verdict` would record the item's review instead; it reviews the checkout's HEAD. `verdict` on no item's branch finds the one plan review the caller holds without a verdict in that checkout, and refuses with each path when it holds several.
- A plan verdict counts the file's own findings and the earlier rounds', not another file of the same round, which is the other blind reviewer's. On an item, where a round has one file, this is the same rule as before.
- `ready` on a mission counts as the plan the mission's directory less `reviews/`, and the directories of items in the mission now or at the reviewed commit, compared with the checkout, uncommitted changes included.
- A plan review's carried finding has no `Response:` line, since a mission has no Report.

### Surprises

- Non-test Rust grew by about 1,040 lines over the base it started from (6,318 to about 7,360 before the merge of `master`, by `scripts/count_non_test_rust.py`), about twice the expected 500. Much of it is wording in `output.rs` (about 270 lines) and the planning commands. I trimmed what repeated (the planner's facts, one refusal for plan reviews held) rather than golf further.

### Review round 1

Three fix-now findings, each fixed in its own commit with a test seen failing without the fix:

- atelier-deu5: fixed in 259d8a36. A plan verdict judges openness only over its own file and earlier rounds, so a blind file of the same round that carries a finding no longer closes it for a file that left it out (`a_plan_verdict_counts_its_own_file_and_earlier_rounds_not_a_blind_one`).
- atelier-uxtd: fixed in a4e2d4ab. Taking over an item whose mission is dropped is refused (`taking_over_an_item_needs_its_mission_not_dropped`, and a started item in `a_dropped_mission_changes_no_item_and_start_on_one_says_it_was_dropped`).
- atelier-i99x: fixed in ff0f6178. Dropping an item only on its branch writes its definition, not its Responses (`an_item_only_on_its_branch_is_dropped_with_its_definition_and_not_its_reviews`).

Then `master` was merged in (05982979, then ba18b68e), bringing atelier-jciw. Both items read a mission's review files at a commit; I kept `CommitReader::mission`, which fills `MissionDir::reviews`, and removed jciw's `CommitReader::mission_reviews` and `Repo::mission_reviews`, so `show` reads `dir.reviews`. `lead` now calls `check_mission` with `MissionFacts`, and its row is in the table test's mission acts.

### Open items

- Two clones and plan reviews: a plan review's `commit` must be in the planner's clone for `ready` to say what changed; otherwise it says so.
- Shared files atelier-jciw may also change: `atelier-cli/src/output.rs` (`overview` gained a `dropped` argument, `item` and `refusal` a `dropped` reason), `commands.rs` (`status` and `refused`), `main.rs`, `repo.rs`, and `atelier-app/src/acts.rs`.
