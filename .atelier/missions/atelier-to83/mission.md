---
schema: atelier.mission
schema_version: 3
id: atelier-to83
title: 'Write master only by merges: plans and record changes on branches, the whole picture derived, and no upfront go'
status: draft
labels:
- redesign
lead: null
ready: null
created_at: 2026-11-13T06:14:32.538523830Z
---

## Why

Atelier keeps its records in the repository so that they travel with the code:
an agent's context is one checkout at one commit, and a pull request that
fixes a bug can carry that bug's record. But the tool also uses `master` as a
live board. `new`, `set`, `ready`, `drop`, `close`, `lead` and the plan-review
cherry-picks all commit straight to the base, and `merge` pushes the base
itself. Three things follow.

- **It cannot work on a protected branch.** Most repositories refuse a direct
  push to their base. Every one of those acts, and the merge, needs one.
- **`master` holds things that are not real.** A draft mission sits on the
  base flagged as not yet agreed. Planning atelier-g961 took three pushed
  revisions of a draft and eight cherry-picked review files.
- **Work in motion is half visible.** The tool reads a started item from the
  local branch named for it and nothing else. A plan on a branch, or an item
  started on another host, appears nowhere. The Desk reads a clone frozen on
  2026-11-11 and showed none of the two days of work after it.

The go-ahead adds a wait and no information. Sumeet's `ready --by` was built
when agents would dive into any ready work they found. Missions now start
from a conversation with him and get two blind plan reviews, which found nine
problems in atelier-g961 that neither he nor the planner had seen. Asking him
again after the reviews repeats a question he answered at the start, and he
is the slowest step: atelier-g961 has been approved and idle since
2026-11-12.

Sumeet set this direction in conversation on 2026-11-13. The reasoning, and
what was considered and turned down, is in [design.md](design.md).

## Outcome

Only a merge writes the base. A thread's changes, code or records, live on a
branch named by a convention the commands apply themselves: an item's ID, a
mission's ID, `backlog/<slug>` for record changes tied to neither, and
`review/<mission-id>/<round>-<suffix>` for one plan reviewer's verdict. What a
branch needs before it merges depends on what it holds: code needs an
independent review, a mission's plan needs two approving plan reviews, and
records alone need `atelier check`. A mission's plan is the Why, Outcome,
Success criteria, Scope and Authority sections of `mission.md`, the documents
beside it other than `notes.md` and its reviews, and, until the plan has
merged, its items. Its lead, its notes, its
Result section and items added to it once it is open are records.

A mission is proposed while its record is `open` and on a branch that has
not merged, and agreed once that merges. One dropped on its branch is
dropped, not proposed, and cannot merge. It has no draft status and no go-ahead; its Why says whose direction
it started from. Its stored status is `open`, `done` or `dropped`. Items still merge to the base one at a time, so the base
shows a mission in progress as it is: agreed, with some items done.

`atelier list`, `atelier show` and the Desk give the whole picture by reading
the base and the named branches, local and fetched. The Desk reads the live
repositories, shows what is proposed and in motion and which thread is doing
it, tells Sumeet when a plan is proposed and when it merges, and lets him
stop a mission at any time, by an act that is finished only when the stop is
published.

## Success criteria

- protected-base: against a remote that refuses every push to the base that
  is not the forge's merge of a pull request, a mission is planned, reviewed
  and merged, one of its items is implemented, reviewed and merged, a bug is
  filed and an item is dropped, with no `ATELIER_SKIP` and no direct push.
  The same script passes with no forge configured, where `atelier merge`
  moves the base locally.
- only-merges: over the proof mission in item atelier-ql64, every commit on
  the base's first-parent history is a merge of a named branch.
- stop-is-effective: after a stop of an open mission from the Desk, `start`
  on one of its ready items and `merge` of one of its started items are both
  refused, in a second clone that has fetched. After `drop` of a started
  item, that item's `merge` is refused, whichever of the two was run first
  on the other host.
- one-holder: two clones of one remote start the same item at the same
  moment; one succeeds and the other is refused and told which thread holds
  it.
- seen-before-merge: a mission planned in one clone and pushed appears, as
  proposed and with the branch it was read from, in `atelier list` in a
  second clone after `git fetch`, and on the Desk within its refresh period.
- reviews-are-the-gate: `atelier merge` refuses a branch that adds a mission
  or changes a mission's plan while fewer than two reviewers, neither a
  planner of it, approve the plan its head holds, and merges it with two. A
  branch that changes only a mission's lead, notes, Result or status, or adds
  an item to an open mission, merges on `atelier check`. No command records
  a go-ahead on a mission, and `start` asks of a mission only that it is not
  dropped.
- desk-is-live: for one open mission in each of the five repositories the
  Desk's page matches `atelier show <mission>` run in the live repository,
  and its page of work in motion names the thread on each started item.
- records-are-quick: in each of the five repositories a branch that changes
  only records goes from `submit` to merged in under two minutes, or the
  Result says what stops it and files the item.

## Scope

- In: the commands and hooks in `crates/`; the Desk under `desk/` and
  `crates/atelier-desk`; `docs/product.md`, `docs/design.md` and the Desk's
  two documents; the skill and `AGENTS.md`; the records, hooks and CI rule
  of the five repositories on this host.
- Out: an integration branch for a mission; a database or a lock service for
  records; any change to how an item is reviewed; the work host, which
  adopts the tool once through atelier-7w9s after this mission closes; a
  finder on the Desk (atelier-ic22) and a mission's history page
  (atelier-hj6b); the gardener and backlog procedures, which mission
  atelier-g961 rewrites and item atelier-j5lr then brings to this model.
- Target environment: this host and GitHub for the five repositories; a
  local remote with a refusing hook stands in for a protected branch in
  tests.

## Authority

The lead may choose the wording of the design, the config keys, the layout
of `list`, `show` and the Desk's pages, how `done` is recorded when the forge
makes the merge commit, and the order of items within their blockers. It may
not let a branch that changes any path outside `.atelier/` merge without an
independent review; turn on or change branch protection or any other setting of a GitHub
repository, which is Sumeet's to do; push to the base of any of the five
repositories outside an Atelier merge without a recorded `ATELIER_SKIP`
reason; add an integration branch, a database or a lock; restore a go-ahead
on a mission; or change what review an item's code needs. A change of record
format must keep the installed binary reading the old form, since one binary
serves all five repositories.

## Result
