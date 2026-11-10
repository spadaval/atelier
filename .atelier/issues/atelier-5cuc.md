---
created_at: "2026-11-10T00:56:07.002116482+00:00"
id: "atelier-5cuc"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-5cuc
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-engh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add `review` and `verdict`, with the brief and one file per review"
updated_at: "2026-11-10T16:39:49.321367337+00:00"
---

## Goal

Build the reviewer's side of the loop: one command that gives a reviewer everything it needs and a file to write in, and one that records its verdict. Each submission opens a new round, and a dead reviewer can be replaced.

## Done when

- [ ] Scenario test: on an item in review, thread `t2` runs `atelier review <id>` with the new binary. It creates `reviews/1-<suffix>.md` with the head commit, `t2` as reviewer and the template, and prints a brief holding the item's Goal, Done when, Scope and Report, the command that shows the diff, the path of the review file and the two `verdict` commands. Run again by `t2`, it creates nothing and prints the same brief.
- [ ] A thread anywhere in the item's `implementers`, current or past, that runs `review <id>` gets the brief and a line saying an author does not review its own work. No file is created and no reviewer is changed. A test covers a thread that implemented the item and was then taken over.
- [ ] `verdict changes` on a file with an open fix-now finding sets `changes_requested` and commits the review file in one commit; on a file with none it is refused. `verdict approve` on a file with an open fix-now finding is refused; otherwise it sets `approved`.
- [ ] After `changes_requested`, the implementer's `status` lists each open fix-now finding by number and title. `submit` changes nothing until the Report's Responses has a line for each one, as fixed in a commit or as rebutted.
- [ ] Scenario test, second round: after that `submit`, `review <id>` by a thread creates `reviews/2-<suffix>.md`, which carries each open finding of round 1 by its name, such as `1-x7qd.1`, with the author's response under `Was:`. The reviewer sets `Now:` to fixed, withdrawn or kept, and a kept fix-now finding counts as open for the verdict. The same holds after an approval went stale and the item was submitted again.
- [ ] Scenario test, reviewer taken over: with round 1's file created by `t2` and no verdict, `review <id>` by `t3` appends `t3` to the file's `reviewers` so that it holds the review, keeps what the file holds, and says `t2` held it.
- [ ] A thread bound as reviewer that runs `submit` gets one line naming its role and its two commands, a non-zero exit, and no change.
- [ ] The Records section of `docs/strategies/redesign-architecture.md` matches the review file as built.

## Scope

Reviews of items. Plan reviews of missions belong to step B, and findings settled by a decision to step C. The rule that a rebuttal goes to a fresh reviewer is the skill's to state; this item does not enforce it.

## Context

Behaviour: `docs/strategies/redesign-product.md`, Commands (Reviewer), Review, and "When a thread dies". The reviewer works in the implementer's checkout, where T3 starts a delegated child by default.

## Notes

- 2026-11-10 [decision] The orchestrator narrowed the Done when line on a reviewer's wrong command from `submit` or `merge` to `submit` only
  Reason: `merge` does not exist until atelier-syyj; the orchestrator will tell that item's implementer to cover a reviewer who runs `merge`.
  Rejected: Build a stub merge here to test the reviewer's refusal
- 2026-11-10 [decision] Expected size: about 900 non-test lines of Rust, scenario and unit tests about as large again, and about 40 changed lines of the architecture specification
  Reason: The orchestrator's brief gives this size; past about twice it, the implementer escalates.
