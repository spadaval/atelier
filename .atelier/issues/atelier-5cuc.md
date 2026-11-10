---
created_at: "2026-11-10T00:56:07.002116482+00:00"
id: "atelier-5cuc"
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
    id: "atelier-engh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add `review` and `verdict`, with the brief and one file per review"
updated_at: "2026-11-10T00:56:07.002116482+00:00"
---

## Goal

Build the reviewer's side of the loop: one command that gives a reviewer everything it needs and a file to write in, and one that records its verdict.

## Done when

- [ ] Scenario test: on an item in review, thread `t2` runs `atelier-next review <id>`. It creates `reviews/1.md` with the head commit, `t2` as reviewer and the template, and prints a brief holding the item's Goal, Done when, Scope and Report, the command that shows the diff, the path of the review file and the two `verdict` commands. Running it again creates nothing and prints the same brief.
- [ ] The implementer's thread running `review <id>` gets the brief and a line saying an author does not review its own work. No file is created.
- [ ] `verdict changes` on a file with a fix-now finding sets `changes_requested` and commits the review file in one commit; on a file with none it is refused. `verdict approve` on a file with a fix-now finding is refused; otherwise it sets `approved`.
- [ ] After `changes_requested`, the implementer's `status` lists each open fix-now finding by title. `submit` changes nothing until the Report's Responses names each one as fixed in a commit or rebutted.
- [ ] Review 2's template carries each open finding of review 1 with the author's response under `Was:`. The reviewer sets `Now:` to fixed, withdrawn or kept, and a kept fix-now finding counts as one for the verdict.
- [ ] A thread bound as reviewer that runs `submit` or `merge` gets one line naming its role and its two commands, a non-zero exit, and no change.
- [ ] The Records section of `docs/strategies/redesign-architecture.md` matches the review file as built.

## Scope

Reviews of items. Plan reviews of missions belong to step B. The rule that a rebuttal goes to a fresh reviewer is the skill's to state; this item does not enforce it.

## Context

Behaviour: `docs/strategies/redesign-product.md`, Commands (Reviewer) and Review. The reviewer works in the implementer's checkout, where T3 starts a delegated child by default.
