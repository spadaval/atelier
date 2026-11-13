---
schema: atelier.item
schema_version: 3
id: atelier-ogpg
title: Record the branch model in docs/product.md and docs/design.md before any code changes
status: draft
mission: atelier-to83
labels:
- redesign
priority: high
blocked_by: []
implementers: []
ready: null
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

- [ ] `docs/product.md` describes the model in its own terms: only a merge writes the base; the three kinds of branch and the optional prefix, which applies to all three; what each kind needs before it merges; a mission as proposed while its status is `open` on a branch that has not merged, agreed once merged, and dropped, not proposed, when dropped on its branch; no draft status and no go-ahead; items keeping `draft` and `ready`; `lead` as attribution.
- [ ] Its tables of statuses, commands and records match: no `draft` or `ready` row for a mission, no `ready` on a mission, and which three commands act on Git and that every other command that changes a record only writes the file.
- [ ] `docs/design.md` says, for every act in its act table, which kind of branch its record change belongs on and whether the command or the agent commits it, and replaces the base rule of the pre-push hook with the rule that the base receives only merges.
- [ ] Its Merge section gives the steps with a forge, ending in the forge's merge of the pull request, and without one; says what `merge` does while a required human review is missing; and settles how `done` is recorded when the forge makes the merge commit, with the alternative it turned down.
- [ ] It says how a record is found across branches: which refs are read, which copy wins when the base and a branch both hold a record, and what `list` and `show` say about where a record was read from and how old the last fetch is.
- [ ] It settles what a record-writing command does in the base's own checkout.
- [ ] It gives the mission record's new format and how the binary reads the old one.
- [ ] "Tried and removed" and "Decisions" carry every row of this mission's `design.md`, and the row "A plan as its own branch" is corrected to say what was learned.
- [ ] Nothing in either document still says planning happens on the base.

## Scope

In: `docs/product.md` and `docs/design.md`.

Out: any change under `crates/`, `desk/` or `skills/`; the Desk's two documents, which atelier-v958 changes; decisions this mission's Authority withholds.

## Context

`.atelier/missions/atelier-to83/design.md`. The sections to change in `docs/design.md`: Status (line 320), Where records are read and written (591), Merge (772), Hooks and the bypass (977), The forge (1114), Tried and removed (1267), Decisions (1319).
