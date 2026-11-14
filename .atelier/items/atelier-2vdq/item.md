---
schema: atelier.item
schema_version: 3
id: atelier-2vdq
title: Plan a mission on its branch and make two approving plan reviews its only gate, with no draft status and no go
status: approved
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ob9u
- atelier-dciw
implementers:
- thread:delegated-task:command%3Amcp%3A93b1b51f-2095-4d1f-af23-677aa6ba570d%3Adelegate-task%3Aa32c13c1-0ffe-486e-9f49-dd9564a14dc3
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-14T00:14:10.322444756Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.672218464Z
---

## Goal

Give a mission's plan the same flow as any change, and take Sumeet out of the
path. Today a plan is a draft on the base, its reviewers work in detached
checkouts and their files are cherry-picked over, and nothing can start until
`ready --by` records his go-ahead. After this item the plan lives on the
mission's branch with its reviews beside it, merges once two reviewers
approve, and is agreed by being on the base.

## Done when

- [x] `start --mission` records the calling thread as a planner of the mission, and a thread that runs `submit` on the mission's branch without being one takes the plan over and is added. The act table has rows for `submit` and `merge` of a mission's branch: a planner of it, in a checkout on that branch, from proposed, which means status `open` on a branch that has not merged. Both refuse a mission dropped on its branch, with a test that a dropped plan holding two approvals does not merge, with a forge and without.
- [x] `atelier submit` on a mission's branch pushes it and opens its pull request, and opens the next round after changes.
- [x] `atelier review <mission>` refuses a planner of the mission, creates the review file for the commit the checkout is at, and prints the brief. `verdict` writes the verdict into that file. Neither commits: the reviewer commits the file and pushes it to the mission's branch with Git, and the brief says so and gives the commands, including the one for a push the other reviewer got ahead of. Test: two reviewers of one round, each in a checkout at the submitted commit in its own clone, both land their files on the mission's branch with no conflict.
- [x] `atelier merge` refuses a branch that adds a mission, or changes a mission's plan (its Why, Outcome, Success criteria, Scope or Authority, or any other file in its directory but `notes.md` and its reviews), unless two reviews of the current round, from threads that are not its planners, approve the plan its head holds, and says what is missing. The plan is compared by section of `mission.md`, so the Result section and the front matter are not part of it. While the mission is not on the base, its items are part of it: changing, adding, removing or reassigning one makes the approvals stale. A commit that adds only review files does not.
- [x] A branch that changes only a mission's `lead`, `notes.md`, Result or status, or adds an item to an open mission, merges on `atelier check` alone. Tests: on a proposed plan, Success criteria changed after approval is refused, `design.md` beside the mission changed after approval is refused, and an initial item's Done when changed after approval is refused; on an open mission, a Why change is refused, and a note, a Result-only change and an added item each merge without a review.
- [x] A mission's stored status is `open`, `done` or `dropped`, with no `ready` field. One whose status is `open`, read from a branch that has not merged, is shown as proposed; one dropped there is shown as dropped. `lead` and `close` run from `open` on the base. `ready` refuses a mission and says a mission needs no go-ahead.
- [x] `start` and a take-over ask of an item's mission only that it is not dropped, which they keep refusing. An item's `merge` refuses too once its mission is dropped, and names the stop.
- [x] `drop` of a mission writes the record, as any `drop` does, and stops nothing until it is published: `status` and `show` say so while it is not. A planner publishes it with Git and, for an open mission, `submit` and `merge` by the record-only path; for a proposed plan, a push and closing its pull request. Tests cover a stop of a proposed plan, and a stop of an open mission with one ready item and one started item, read from a second clone after a fetch.
- [x] A mission's Why names whose direction it started from, and `atelier check` says so when the section is empty.
- [x] `lead` writes the `lead` field in the checkout it runs in, and it reaches the base by the record-only path.
- [x] The binary reads a mission record in the old format: `status: ready` reads as `open`, `status: draft` on the base reads as proposed and cannot be led or have its items started, and the `ready` field is ignored. A test reads each open mission record of the five repositories as they stand today.
- [x] atelier-lkf5 is dropped as moot, or the Report shows it still happens.

## Scope

In: `crates/atelier-core` (`mission.rs`), `crates/atelier-app` (`plan.rs`, `acts.rs`, `decide.rs`), `crates/atelier-cli` (`plan.rs`, `review.rs`, `merge.rs`), their tests.

Out: converting any repository's records (atelier-ql64); the Desk's "say go" (atelier-v958); how a merge finishes with a forge (atelier-t5am); items' `draft` and `ready`, which stay.

## Context

`docs/product.md` lines 109 to 130 define a mission's statuses and `ready` today. The skill's rule of two blind reviews is `skills/agent-factory/roles/planner.md`.

## Report

A mission's plan now goes through its own branch like any change, and
`merge` lets it onto the base once two plan reviews by threads that are
not its planners approve the plan its head holds. Nobody says go.

What changed, by Done when line:

- Planners. `Mission` has a `planners` field (`Threads`, default empty).
  `start --mission` writes the caller as the first. `submit` on a plan's
  branch (`commands.rs`, `planned_here`) adds a thread that is not one,
  with the commit message "Take over the plan of X from Y". The act table
  has `SubmitMission` (from proposed, binds, writes an old `draft` as
  `open`) and `MergeMission` (from `open`, needs a planner). A plan
  dropped on its branch is refused by both, with and without a forge
  (`a_plan_reviewed_by_a_planner_or_twice_by_one_thread_or_dropped_does_not_merge`,
  `a_plan_dropped_on_its_branch_with_two_approvals_does_not_merge_and_the_stop_counts_once_pushed`).
- `submit` on a plan's branch pushes it and opens its pull request through
  the existing `share`, names the round it opens (the latest while a
  review of it has no verdict, else the next) and prints a handoff for two
  reviewers, each in a checkout of its own at the submitted commit. It
  refuses while the fetched copy of the branch has commits this checkout
  lacks, naming `git pull --no-rebase`.
- `review <mission>` refuses a planner (a rule). The brief says no command
  commits or pushes the review, and `verdict` prints the commit line and
  then the push, plus the one to run when the other reviewer pushed first:
  `git pull --rebase <remote> <branch> && git push ...` with a forge;
  with none, a fast-forward of the checkout that has the branch, or `git
  push .`. `two_reviewers_in_clones_of_their_own_land_their_files_on_the_plans_branch_and_it_merges`
  runs both reviewers in clones of their own and lands both files.
- The gate (`merge.rs` `read_records`, `plans_changed`; `plan.rs`
  `plan_diff`, `approvals`). A plan is compared by section of
  `mission.md` (Why, Outcome, Success criteria, Scope, Authority), plus
  every other file in its directory but `notes.md` and `reviews/`, plus,
  while the mission is not on the base, every path of an item of it at
  either commit. Only the latest round counts; a review counts when it
  approves, no planner is among its reviewers, its reviewer has not been
  counted already, its commit is in the clone, and the plan at that
  commit is the plan at the head. The refusal lists each review of the
  round and why it does or does not count. It is a rule, as an item's
  missing approval is.
- Tests for the gate: `a_proposed_plan_merges_on_two_approvals_of_its_head_and_any_change_to_it_after_makes_them_stale`
  (Success criteria, `design.md`, an item's Done when; a note and a Result
  change merge) and
  `an_agreed_missions_record_changes_merge_on_check_and_its_plan_needs_two_reviews`
  (a note, a Result, an added item and the lead merge on `check`; a Why
  change waits for two reviews and merges with them).
- Status. `MissionStatus` is `Open`, `Done`, `Dropped`, and a read-only
  `Draft`; `ready` reads as `open` (serde alias) and the `ready` field is
  read and never written. The act table, which is written in an item's
  statuses, writes a mission's `open` as `ready` (`MissionStatus::status`);
  every place that prints a mission's status uses its own names. `show`
  and `list` say proposed, agreed, in progress, done or dropped, and
  `show` lists the planners. `lead` and `close` refuse a mission that the
  base's last commit does not hold open (`not_agreed`). `ready` refuses a
  mission and says it needs no go-ahead. `log` prints a mission's move as
  `open -> done`.
- `start` and a take-over refuse an item whose mission is dropped, an old
  draft, or not on the base; an item's `merge` refuses once its mission is
  dropped (`Condition::MissionNotOpen`). A stop counts once it is on the
  base, this clone's or the forge's as last fetched (`Repo::links`,
  `Repo::published_stops`), so a clone that has fetched refuses before its
  own base moves.
- `drop` of a mission writes the record and says how it is published: for
  a proposed plan, push and `gh pr close`, never merged; for an agreed
  one, `submit` and `merge`. `show` and `status` say "Stop not published"
  until then. `a_stop_of_an_agreed_mission_refuses_its_ready_and_started_items_in_a_clone_that_has_fetched`
  covers the open mission with a ready and a started item.
- `atelier check` says when a mission's Why is empty. It is a note, not a
  problem: `start --mission` makes an empty Why, and a problem would refuse
  that first commit and every record write while it stays empty.
- `lead` was already written in the checkout; it is now refused on a
  proposed mission, and its record-only merge is tested.
- Old form: `every_open_mission_record_of_the_five_repositories_reads_in_the_old_form`
  reads the 16 open mission records of atelier, astromech, cosplay-studio,
  demiurge and life-on-rails as their bases held them today. This
  repository is public and two of those are private or a company's, so the
  fixtures keep each record's front matter as written and replace its
  title, labels, threads and body. I also ran the new binary's `list`, and
  `show` of every mission, in each of the five live checkouts: all exit 0.
- atelier-lkf5 still happens. In a scratch repository, with an item's
  `mission:` set only in the index and the working file put back, `review
  <mission>` printed no line saying the checkout changes the plan. This
  item did not change that warning (`plan_changed`). It matters less now:
  the reviewer works in a checkout at the submitted commit, and `merge`
  judges the committed head with `plan_diff`.

Round 1 found three paths the gate missed, each now refused and tested:
an item's branch that changes a plan merged on its code review
(`an_items_branch_that_changes_a_plan_does_not_merge_on_its_code_review`;
it is refused and named the backlog branch an amendment goes on, as
`docs/product.md` says); an old draft on the base written open merged
with no review
(`an_old_draft_on_the_base_is_agreed_only_by_two_plan_reviews`); and
`show` in a clone that had fetched a published stop still called it
unpublished (now covered in the cross-clone stop test).

After round 2 approved it, CI's Desk browser tests failed: two of them
expected a mission's status pill to read "Ready", and it now reads
"Open". Only those two expectations changed (`desk/e2e/acts.spec.ts`,
`desk/e2e/pages.spec.ts`); `pnpm e2e` passes locally, 52 of 52.

Outside the Done when lines:

- `submit` on a backlog branch that changes an agreed mission's plan adds
  the caller to that mission's planners too, as `docs/product.md` says of
  an amendment; the agreed-mission test covers it.
- The Desk changed only as far as compiling needs: `MissionStatus` in
  `desk/src/api` is regenerated, `can_ready` is always false (atelier-v958
  removes the Desk's go), and two comparisons and the e2e fixtures say
  `open` for `ready`.
- Not done: the skill's `roles/planner.md` still tells a planner to
  cherry-pick review files onto the base and wait for Sumeet's go. It is
  outside this item's Scope and needs an item of its own.
- The installed `atelier` is older than atelier-ob9u (it has no `start
  --backlog`). Once this merges, reinstall before planning with it.


### Responses

- 1-lxxo.1: fixed in 6ba389903
- 1-lxxo.2: fixed in 6ba389903
- 1-lxxo.3: fixed in 6ba389903
