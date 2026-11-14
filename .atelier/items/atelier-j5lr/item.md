---
schema: atelier.item
schema_version: 3
id: atelier-j5lr
title: Bring the skill, AGENTS.md and the readiness checklist to the branch model
status: done
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-2vdq
- atelier-t5am
- atelier-xbog
implementers:
- thread:delegated-task:command%3Amcp%3A93b1b51f-2095-4d1f-af23-677aa6ba570d%3Adelegate-task%3A7c0b369c-e261-4886-ab41-7850db3f9d4c
ready:
  by: 'sumeet, on 2026-11-13: execute atelier-to83, relayed by the planner thread'
  at: 2026-11-14T01:30:23.044847409Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.744258640Z
---

## Goal

Bring what agents read into line with the tool. The skill and `AGENTS.md`
tell a planner to work on the base, cherry-pick reviews and wait for Sumeet's
go, and tell everyone that planning records may be committed to `master`
directly.

## Done when

- [x] `roles/planner.md` describes planning a mission on its branch: `submit`, two blind reviewers who each publish their own verdict, fixes, and `merge` once both approve. It says a mission starts from Sumeet's direction, that the Why names it, and that a planner never plans a mission nobody asked for. Nothing in it mentions `ready` on a mission or a cherry-pick.
- [x] The backlog procedure (`backlog.md`, or `groom.md` if atelier-g961 has not merged) says a pass's drops and fixes are committed on a `backlog/` branch and merged through the record-only path.
- [x] `roles/decider.md` is gone. `roles/implementer.md` says a question goes to the thread that started you, and that a finding you think wrong is rebutted once in Responses with a fact or a test and settled by a fresh reviewer. Where `garden.md` counts escalations it counts `atelier list --disputed` instead, by outcome, and reads those items' threads first.
- [x] `roles/lead.md` says a lead steps in on a dispute `show <mission>` lists, and how: it does not settle the finding, which is the fresh reviewer's, but it owns the plan, so where the argument is whether the work belongs in this item it changes the item's Scope or Done when and files the rest as an item.
- [x] `roles/lead.md` also says how a lead adds an item or a note mid-mission, through a record-only branch, and what `lead` now records.
- [x] `readiness.md` gains one line: a branch that changes only records passes CI in under two minutes, with its test and the measure that shows its failure, the time from `submit` to `merge` on such branches in `atelier log`.
- [x] `AGENTS.md`'s Work and Merge Policy sections describe the branch model in full (atelier-ob9u changed only the one sentence that allowed a commit on the base), and nothing in the skill or `AGENTS.md` says a record may be committed on the base.
- [x] The skill is no longer in total than before this item, and the Report gives the line counts.

## Scope

In: `skills/agent-factory/`, `AGENTS.md`, `CONTRIBUTING.md` where it repeats them.

Out: the gardener's procedure beyond the one checklist line; the other four repositories' own `AGENTS.md`, which atelier-ql64 changes.

## Context

Mission atelier-g961 rewrites `garden.md` and folds groom into the planner as `backlog.md`. This item is written to follow it; if it has not merged, this item edits the files as they are.

## Report

The skill, `AGENTS.md` and `CONTRIBUTING.md` now describe the branch model
the tool has since atelier-ob9u, atelier-2vdq, atelier-t5am and atelier-xbog:
every change on a branch `start` makes, a mission planned and agreed on its
own branch by two plan reviews with no go-ahead, record-only changes on
`backlog/<slug>` branches, and no `escalate` or `decide`.

- `roles/planner.md`: a mission starts from Sumeet's direction, its Why
  names him, and a planner never plans one nobody asked for. Planning is
  `start --mission`, `submit`, two blind reviewers each pushing their own
  verdict, fixes, and `merge` once both approve. No `ready` on a mission and
  no cherry-pick.
- `procedures/backlog.md`: a pass's drops and fixes go on one `backlog/`
  branch, merged as records alone.
- `roles/decider.md` is deleted, and `SKILL.md` no longer routes to it or
  names `escalate`. `roles/implementer.md`: a question goes to the thread
  that started you; a finding has two exits, fix or one rebuttal judged by a
  fresh reviewer.
- `procedures/garden.md`: escalations are replaced by disputes, counted from
  `atelier list --disputed` by outcome, and items with a dispute come first
  in the threads read. Four places that filed or committed records on the
  base (opening the run, bugs, labels after the merge, agreed proposals) now
  use a backlog branch. These go past the "one checklist line" in Scope,
  because the Done when line on `AGENTS.md` and the skill requires that
  nothing says a record may be committed on the base.
- `roles/lead.md`: what `lead` records (the lead in the mission's file,
  committed by the lead on a backlog branch), how an item or a note is added
  mid-mission through a record-only branch, and how a lead steps in on a
  dispute `show <mission>` lists: it does not settle the finding, but changes
  the item's Scope or Done when on the item's branch and files the rest as
  an item.
- `procedures/readiness.md` line 10: a record-only branch passes CI in under
  two minutes; test `gh run list --branch <branch>`; shown in the time from
  `submit` to `merge` on such branches in `atelier log`.
- `AGENTS.md` Work: the three branch kinds, which commands act on Git, and
  how a mission is planned. Merge Policy: what each kind of branch needs to
  merge, the forge merging the pull request, and the pre-push hook refusing
  any push that moves `master`. `CONTRIBUTING.md`: readying an item happens
  on a backlog branch.

Line counts of `skills/agent-factory`, before and after: `SKILL.md` 63 to
57, `roles/planner.md` 16 to 21, `roles/reviewer.md` 31 to 31,
`roles/decider.md` 10 to 0, `roles/implementer.md` 18 to 13, `roles/lead.md`
7 to 16, `procedures/garden.md` 197 to 198, `procedures/backlog.md` 70 to
72, `procedures/readiness.md` 104 to 107. Total 516 to 515.

### Validation

- `atelier check`: every record is valid.
- `git diff --check`: clean.
- Readiness line 3's script on `AGENTS.md` prints nothing.
- `grep -rn "on the base\|cherry\|decide\|escalat" skills/agent-factory
  AGENTS.md CONTRIBUTING.md` finds no record committed on the base and no
  reference to the removed commands.
- Each command and flag named was checked against the tree's help in
  `crates/atelier-cli/src/main.rs` and `docs/product.md`. The installed
  binary predates these items (it has no `start --backlog`), so nothing was
  run through it.

### Responses