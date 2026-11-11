---
created_at: "2026-11-11T01:39:45.504060362+00:00"
id: "atelier-8myy"
issue_type: "bug"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-8myy
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Fix what the validator found: a plan review of text it does not record, a broken branch that hides a drop, and nine smaller gaps"
updated_at: "2026-11-11T01:42:57.160475815+00:00"
---

## Goal

Close the gaps an independent validator found between the two specifications and the new binary, before the next mission switches real repositories to it. The validator built `master` at `0e11b9bb` and ran 216 commands against it. Two of what it found are defects that matter at once:

- A plan review shows the reviewer the checkout's text and records `HEAD`. With an uncommitted edit to a mission, `review <mission>` printed "Review the plan as commit 6c02006 holds it" above the uncommitted text, and `verdict approve` then recorded that commit. An approval can name a plan the reviewer never saw.
- A terminal status on the base does not win when the item's branch record is broken. An item was dropped on the base and its surviving branch then held a record with `status: broken`; `status`, `list`, `show`, `submit`, `merge` and `start` all exited 2 with a parse error, and neither orientation nor the drop's reason appeared. The branch record is read before the base's terminal record is looked at.

The rest are smaller: two places where the binary prints less than the specifications promise, and several where a document describes something the binary, by decision, does not do.

## Done when

- [x] `review <mission>` shows the plan as the commit it records holds it: the mission's sections and the paths of its documents and items come from that commit. When the checkout holds uncommitted changes to the mission's directory or its items, the brief says so in one line that names them and says the review does not cover them. A test commits a plan, edits it without committing, and checks the brief holds the committed text and that line.
- [x] A terminal status on the base wins whatever the item's branch holds, including a record that does not parse or validate. A test drops a started item on the base, commits a broken record on its branch, and checks that none of these exits 2: `status` from the base and from the branch, `show`, `submit`, `merge` and `start` each say the item was dropped and why; plain `list` succeeds and leaves the item out, as it does every finished item; and `list --done` shows it as dropped. For an item that is done on the base while a branch of its name holds a broken record, the same commands succeed and say it is done, with no reason to give.
- [x] A mission lead's `status` says what each started item of its mission needs: the unmet conditions the item's holder would see, a question that waits on a decider, and since when an approved item has been approved. A test with a building item that has an unticked Done when line and no Report checks the lead sees both.
- [x] `review <id>` run where the item's branch is not checked out names the checkout that has it, as `decide` does.
- [x] `docs/strategies/redesign-product.md` says what the binary does, outside its Words, Lifecycle and Review sections: the `status` example under "What `status` prints" lists only unmet conditions and names the base's merge checks instead of when they last passed; the `show` row says an item's reviews are summarised by reviewer, verdict and finding, with each finding's text in its review file; the Merge section's rule for which CI run decides includes the case of a failed run with a newer one still running, as the architecture specification states it; and the sentence on an approved item that has not merged says it shows since when, from the verdict's commit.
- [x] `docs/strategies/redesign-architecture.md` gains a short account, under Status, of which rules of the product specification's Review section the binary enforces and which it only shows and leaves to the skill. Enforced: an author never reviews or decides its own item; a reviewer of a finding, in any round that carried it, never decides it; `verdict approve` is refused while a fix-now finding is open; a decided finding cannot be reopened. Shown and not enforced: that a rebuttal goes to a fresh reviewer and gets one try, that escalation is required after three rounds without an approval, and that a mission's lead is the one who decides. It also says the review brief carries no merge check results, since nothing records them.

## Scope

The four changes to `next/` above and the two documents. The product specification's Words, Lifecycle and Review sections are not edited: whether the binary should enforce the Review section's three rules and the lead as decider, and whether the brief should carry merge check results, are the owner's to decide, and this item changes neither the documents' Review text nor the binary's behaviour on them. No flag, config option, record field or dependency. No file under `crates/` is changed.

## Context

The validator's report is in the mission's notes (`atelier issue show atelier-fcb7`), with the commands it ran for each finding. Plan reviews are in `next/crates/atelier-cli/src/review.rs`; where an item's record is chosen between the base and its branch is `next/crates/atelier-cli/src/repo.rs` and `atelier-app`'s `acts.rs`; the lead's view is `atelier-app`'s `view.rs` and `atelier-cli`'s `view_output.rs`. The commit reader that reads records at a commit without a checkout is `atelier-records`' `at_commit.rs`. Design: `docs/strategies/redesign-architecture.md`, "Where records are read and written" ("A terminal status on the base wins").

## Report

All six Done when lines hold. Each code defect has a test that fails on `master`'s code for the reason the validator gave and passes here (run with `master`'s `src/` checked out over this branch's tests): `planning::a_plan_review_shows_the_plan_as_its_commit_holds_it_and_names_what_is_not_committed` (on master the brief shows the uncommitted Outcome), `planning::a_terminal_status_on_the_base_wins_over_a_branch_record_that_does_not_parse` (on master `status` on the branch exits 2), `views::lead_takes_a_ready_mission_and_hands_each_item_that_can_start_to_another_thread` (on master the lead sees no unmet conditions under the building item) and `scenarios::review_runs_on_an_item_in_review_in_the_checkout_on_its_branch` (on master the refusal names no checkout). The lead's questions and since-when lines were already covered by `decisions::a_missions_lead_decides_its_items_questions_and_sees_each_and_since_when_one_is_approved`, which still passes. `cargo fmt -- --check`, `cargo clippy --all-targets --locked -- -D warnings` and `cargo nextest run --locked` (227 passed) pass in `next/`; both eval graders' tests pass (`next-loop` 10, `next-loop-changes` 9, with `NEXT_ATELIER` set); the CI job's steps for `next/` pass in a scratch clone of this branch at 78b1be97 (`run_next_job.py`: all steps passed, 227 tests). Non-test Rust in `next/` went from 11,190 to 11,279 lines.

What changed:

- `Repo::locate_at` reads an item's branch record only when the base's record is not done or dropped (`Located::terminal`), and `merge` does not re-read it at the head then; `show` reads the checkout's settlements of a done item except on its own unmerged branch. So a broken branch record no longer hides a terminal status, and `check` on the branch still reports it.
- `review <mission>` reads the mission, its documents (`git::files_at`, an `ls-tree` of the commit) and its items from the commit the review records, and adds one line naming the plan's paths the checkout or its index changes from it (`plan::plan_changed`, which `ready` already used; `git::changed_since` now adds the index's differences, and an item whose checkout record does not parse is judged by the commit). A mission no commit holds is refused, with the Git command that commits it; that refusal is new.
- `Entry::needs` holds, for a started item another thread holds, the unmet conditions of its holder's next act; `status` for a lead and `lead` print them under the item in `condition`'s wording, leaving out questions, which have their own list. `show <mission>` does not print them. For an approved item, the lead's `status` reads what changed since the approval from the branch's last commit (`Repo::since_approval`, split out of `Repo::checkout`), so it lists a stale approval as the holder does; what the holder has not committed is not seen.
- `review <id>` off the item's branch names the checkout that has it, through `git::checkout_of`, as `decide` does.

Hand transcripts. A dropped item whose branch then commits `status: broken`, before:

```text
$ [imp] atelier status            (on the item's branch)
atelier: .../item.md: front matter: status: unknown variant `broken`, ...
(exit 2)
$ [planner] atelier status        (on main; list, list --done, start, show the same)
atelier: 16d51548...:.atelier/items/demo-8i7r/item.md: front matter: status: unknown variant `broken`, ...
(exit 2)
```

After:

```text
$ [imp] atelier status
You are thread imp, on the branch of demo-vtmu.
Title: Say hello
Status: dropped
It was dropped on the base: Cancelled by planner
Nothing is left to do on it.
(exit 0)
$ [imp] atelier submit
demo-vtmu is dropped, and `atelier submit` runs on an item that is building, changes requested, approved. Nothing changed.
It was dropped on the base: Cancelled by planner
Nothing is left to do on it.
If the rule is wrong for this case, go past it on purpose: ...
(exit 1)
$ [planner] atelier status
You are thread planner, on the base branch main.
Dropped on the base while started; nothing is left to do on them:
  demo-vtmu: Cancelled by planner
Nothing is ready to start.
(exit 0)
$ [planner] atelier list --done
In no mission:
  No label:
    demo-vtmu (dropped, medium): Say hello
(exit 0)
$ [imp] atelier check             (on the item's branch)
`atelier check` found 1 problem in 0 items, 0 missions and config.toml.
These do not read as records, so no rule can be checked on them:
  .atelier/items/demo-f6ry/item.md: front matter: status: unknown variant `broken`, ...
(exit 1)
```

A plan committed with `COMMITTED_OUTCOME`, then edited to `UNCOMMITTED_OUTCOME` with a new `design.md`, before:

```text
Review the plan as commit 739b45c holds it, as someone who did not write it: ...
(its Outcome section:) UNCOMMITTED_OUTCOME

Its other documents:
  .atelier/missions/demo-dtdm/design.md
```

After:

```text
Review the plan as commit b14d768 holds it, as someone who did not write it: ...
(its Outcome section:) COMMITTED_OUTCOME

Its other documents:
  (none)
Its items:
  .atelier/items/demo-sy61/item.md (draft): First item
This checkout changes the plan from commit b14d768, and this review does not cover those changes: .atelier/missions/demo-1owk/design.md, .atelier/missions/demo-1owk/mission.md
```

Cases tried by hand around the record choice, each in a scratch repository: done on the base with a healthy leftover branch, dropped with a healthy branch, and dropped with the branch's record deleted all print the terminal status and its reason, with `submit` and `start` refused (exit 1); a healthy started item is unchanged (`status`, `show`, `list`, `submit`'s refusal and a take-over by another thread). A drop made in the base's checkout and not committed does not win, as the specification says (a planning edit counts once committed): with a healthy branch the item reads as building, and with a broken branch every command still exits 2 on the parse error.

The product specification's diff touches lines 1-6 (front matter), 172-178 (`show` row, under Commands), 232-249 ("What `status` prints", under Commands), 371-391 and 393-406 (both under Merge); Words is lines 73-95, Lifecycle 97-164 and Review 317-349, none changed.

### Review rounds

Round 1 (Sol): R1 `atelier-r9f5` (lead missed a stale approval), R2 `atelier-xn9k` (`show` on a done item's merged own branch read its broken record), R4 `atelier-o94f` (staged-only plan changes unnamed) fixed, each in its own commit with its regression added to the tests above; R3 `atelier-pb9t` fixed for an item of the plan whose checkout record does not parse, and answered by rebuttal for a `mission.md` that does not parse; R5 `atelier-w495` deferred as recommended, filed as `atelier-m2v1`.

### Deviations

- The architecture specification's account says the binary does not show the three-round escalation rule at all: nothing counts rounds. The Done when line lists it among the rules "shown and not enforced"; the text says what the binary does. It shows the rebuttal's one try and the lead as decider; a former reviewer is offered `review` again in a later round, so the fresh-reviewer rule is not shown either.
- For an item done on the base with a broken branch, `status` on the base succeeds and does not list it, as for any done item; only a dropped one with a branch is listed, with its reason, which the existing test `a_done_or_dropped_record_on_the_base_wins_over_the_branch` already fixes.

### Surprises

- `drop` on a started item whose branch record already does not parse exits 2: it reads the branch's record before it writes the base's. The architecture specification says `drop` writes the base's record "whatever the item's branch holds". Not fixed here; the drop works once the branch's record is repaired, and after it is committed every command reads it first.
