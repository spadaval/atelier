---
schema: atelier.item
schema_version: 3
id: atelier-ogpg
title: Record the branch model in docs/product.md and docs/design.md before any code changes
status: in_review
mission: atelier-to83
labels:
- redesign
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A23988f7d-f26b-412b-9940-872c731e6585%3Adelegate-task%3A6881c7fb-1ff3-42fb-8ac4-469d91af3d6c
ready:
  by: sumeet, in conversation on 2026-11-13
  at: 2026-11-13T08:27:12.090629970Z
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.596717477Z
---

## Goal

Put the branch model into the two documents the tool is built from, before any
code changes, so each later item is implemented against reviewed text. The
decisions are in this mission's `design.md`; an earlier version of one of them
was agreed in a thread on 2026-11-10 and lost because it was never written
down.

## Done when

- [x] `docs/product.md` describes the model in its own terms: only a merge writes the base; the three kinds of branch and the optional prefix, which applies to all three; what each kind needs before it merges; a mission as proposed while its status is `open` on a branch that has not merged, agreed once merged, and dropped, not proposed, when dropped on its branch; no draft status and no go-ahead; items keeping `draft` and `ready`; `lead` as attribution.
- [x] Its tables of statuses, commands and records match: no `draft` or `ready` row for a mission, no `ready` on a mission, and which three commands act on Git and that every other command that changes a record only writes the file.
- [x] `docs/design.md` says, for every act in its act table, which kind of branch its record change belongs on and whether the command or the agent commits it, and replaces the base rule of the pre-push hook with the rule that the base receives only merges.
- [x] Its Merge section gives the steps with a forge, ending in the forge's merge of the pull request, and without one; says what `merge` does while a required human review is missing; and settles how `done` is recorded when the forge makes the merge commit, with the alternative it turned down.
- [x] It says how a record is found across branches: which refs are read, which copy wins when the base and a branch both hold a record, and what `list` and `show` say about where a record was read from and how old the last fetch is.
- [x] It settles what a record-writing command does in the base's own checkout.
- [x] It gives the mission record's new format and how the binary reads the old one.
- [x] "Tried and removed" and "Decisions" carry every row of this mission's `design.md`, and the row "A plan as its own branch" is corrected to say what was learned.
- [x] Nothing in either document still says planning happens on the base.

## Scope

In: `docs/product.md` and `docs/design.md`.

Out: any change under `crates/`, `desk/` or `skills/`; the Desk's two documents, which atelier-v958 changes; decisions this mission's Authority withholds.

## Context

`.atelier/missions/atelier-to83/design.md`. The sections to change in `docs/design.md`: Status (line 320), Where records are read and written (591), Merge (772), Hooks and the bypass (977), The forge (1114), Tried and removed (1267), Decisions (1319).

## Report

`docs/product.md` and `docs/design.md` now describe the branch model of
atelier-to83, with a note at the top of each that the parts an item covers
are ahead of the binary until that item merges.

What I settled where the mission left it to this item:

- **How `done` is recorded with a forge.** Nothing writes `done` at a merge.
  A started item's record reaches the base only through its own merge (the
  pre-commit hook refuses record changes on the base, and `merge` refuses
  any other branch bringing an item in a started status), so the reader reads
  a base copy in a started status as done. Turned down: a last commit on the
  branch setting `done`, because under a protection that requires CI on the
  head it costs an extra CI run per merge, says done before the merge, and
  stays wrong if the forge refuses. Same rule without a forge, so there is one
  path. `log` finds the merge from the record, not the trailer, so a merge
  made by hand on GitHub reads the same. This is in Merge, "How `done` is
  recorded", and in Decisions.
- **A record-writing command in the base's checkout** refuses and writes
  nothing, naming the `start` for the right branch; `start` there makes and
  claims the branch without switching and prints the `git worktree add`.
  Turned down: the tool making the worktree.
- **Which copy wins across branches**: base when final (done, dropped, or a
  started item, which means merged); else the record's own unmerged branch,
  the newer of local and remote-tracking, with `dropped` on either winning if
  they diverged; else the base; a record only on some other unmerged branch
  (filed on a backlog branch, or in a proposed plan) is read from there as
  proposed. Copies on other branches are proposed changes. Fetch age comes
  from `FETCH_HEAD`'s modification time.
- **The base's tip** is the remote-tracking base when a forge is configured,
  since the forge makes every merge, else the local base.
- **Mission format**: `status: open|done|dropped`, `planners`, no `ready` or
  `created_at`, schema version unchanged; the old form is read (`ready` as
  `open`, `draft` as proposed, old fields ignored) and rewritten in the new
  form when next written.
- **`branch_prefix`** is the config key for the prefix.

Choices a reviewer should look at, since they go a little past the mission's
text:

- Two exceptions to "no command fetches": `start` fetches the one branch
  whose claim it lost, to name the holder, and `merge` fetches the base after
  the forge merged. Without these, neither can report what it did.
- A backlog branch that changes code is not a skippable rule, since the
  mission's Authority forbids code merging without review.
- `start` on an item of a proposed plan waits for the plan to merge (its
  record must be on the base), which the mission's items did not state.
- `escalate` and `decide` stay described, with rows in the act table marked
  as removed by atelier-xbog, which owns taking them out of both documents.
- The item format keeps `ready.at` and `created_at`: no item of this mission
  changes it, and the Decisions row says so.

### Validation

Documentation only. `git diff --check` is clean. I grepped both documents for
"planning", "on the base", "draft", "cherry" and "go-ahead" and reworded every
place that still put planning on the base. No code or tests were touched.

### Responses

- 1-ep2v.1: fixed in c328a9380: `merge` step 5 and both documents now apply every gate a branch's contents need, whatever its kind. Code is allowed only on an item's branch (refused elsewhere, not skippable); a plan change needs two plan approvals on a mission's or backlog branch and is refused on an item's branch.
- 1-ep2v.2: fixed in c328a9380: an amendment of an agreed plan goes on a backlog branch. `submit` there adds the submitter to the mission's `planners` and opens the next plan round; `review` and both plan verdicts run from `open` with the plan changed on that branch; `merge` (backlog) applies the plan gate for each mission it changes.
- 1-ep2v.3: fixed in c328a9380: `config.toml` counts as code in the definition of a record-only change, the Branches table, `merge` step 5 and the backlog merge row.
- 1-ep2v.4: fixed in c328a9380: the product's Merge section now pushes whenever the forge's copy is not the head, and explains the extra CI run under a protection that requires CI on the head.
- 1-ep2v.5: fixed in c328a9380: The walk's `--full-history` bullet now explains why the flag is needed when a merge writes nothing of its own.
- 2-gci2.6: fixed in 89aef9cb0: every command but `drop` refuses to write an old `draft` mission, naming the rollout, which moves it onto its own branch to go through plan reviews; `drop` writes it as `dropped`.
- 2-gci2.7: fixed in 89aef9cb0: a plan reviewer pushes to the branch it reviewed (the mission's branch, or the backlog branch for an amendment), in both documents and the Decisions row.

