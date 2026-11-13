---
schema: atelier.item
schema_version: 3
id: atelier-s47q
title: Fold groom into the planner as backlog.md, and let it drop provably stale items itself
status: in_review
mission: atelier-g961
labels:
- agent-factory
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3Aede59fb7-cf1e-46fc-bfdc-d4202d8ed2aa
ready:
  by: sumeet, in conversation on 2026-11-13
  at: 2026-11-13T08:27:11.885174987Z
trivial: null
caused_by: null
created_at: 2026-11-12T01:03:20.861719673Z
---

## Goal

Make the whole-backlog pass part of the planner's job, under a plain name, and
let the planner act on what it can prove. Today it is a separate procedure
called groom that sends every entry but a label fix to Sumeet in a digest. The
one digest, atelier-8i86, waited weeks with 21 entries. The sweep of
2026-11-11 dropped 104 records on evidence and needed six answers from him.

## Done when

- [x] **Two scales.** `roles/planner.md` says the planner works at two scales, one mission and the whole backlog, and sends the second to `procedures/backlog.md`. It keeps its Readiness and Review rules.
- [x] **Renamed.** `procedures/groom.md` is `procedures/backlog.md` (`git mv`, so history follows). `grep -ri groom skills/agent-factory` finds nothing. `SKILL.md`'s description and "Your Role" section route a request to go over, sweep or reorder the backlog to `planner.md`.
- [x] **Drops on proof.** `backlog.md` lets the planner run `atelier drop` itself on an item once it has proof against the item's Goal and unticked Done when lines that nothing is left to do: the need is gone (the feature or rule it serves was removed, by a named commit); the behaviour is already there (a test on the installed binary, or the commit that did it); or a named item covers all that remains. The proof goes in the drop's reason.
- [x] **Gone names are not proof.** It says a search that finds nothing the item names shows only that the wording is old. The planner then traces what replaced the code and tests the outcome there. If the work is still wanted, it rewrites the item against today's code; if part is done, it keeps the rest; if it cannot tell, the item is a call for Sumeet.
- [x] **Other fixes.** It may also make the record fixes groom allowed and clear a blocker on a dropped item.
- [x] **Never without Sumeet.** It names what the planner brings to him and does not decide, with a test readable from the records for the first: an item he asked for or set aside himself, meaning one whose `ready.by` names him, or whose body says he asked for it or gives his condition for picking it up, even when its proof of staleness is complete; real work we could do but may not want; a change of priority or order between missions; forming or dropping a mission that is not wholly stale. A started item, and an item in a mission another thread leads, are left to their owners.
- [x] **The calls.** What goes to Sumeet is one numbered list of at most seven calls, each a sentence of evidence and a recommendation, put to him in the conversation. Further calls wait for the next pass and the report says how many. An item labelled `needs-human` holds the list only when he is not in the conversation, and the per-entry Size block, the word "parked" and the `groom` label are gone.
- [x] **Report.** The pass ends with a report in the order the sweep used: the counts before and after, what was dropped by kind with how it was proved, what was changed on kept items, the calls, and a suggested order for what remains.
- [x] **Docs.** `docs/product/desk.md` (lines 48 and 91 today) and `docs/architecture/desk.md` (line 763) name the backlog pass in place of grooming.
- [x] **Size.** `planner.md` and `backlog.md` together are no longer than 96 lines, and the Report gives the counts before and after.

## Scope

In: `skills/agent-factory/roles/planner.md`, `procedures/groom.md` renamed to `backlog.md`, the description and "Your Role" section of `SKILL.md`, and the three lines in the Desk's two docs.

Out: the "Who Works With Sumeet" section of `SKILL.md` and anything in `garden.md`, which atelier-wb05 changes; `docs/design.md`, whose one use of "grooming" is plain English in a table of things tried; any command or label rule in `crates/`; the closed digest atelier-8i86.

## Context

The sweep this item takes its shape from is in the commits b4895f933..7807b8d51 on `master`: each `Drop atelier-xxxx` commit carries its proof as the reason. Sumeet agreed the split and the name on 2026-11-11: the planner owns what to build, the gardener the state of what was built.

## Report

The backlog pass is now the planner's job, in
`skills/agent-factory/procedures/backlog.md` (renamed from `groom.md` with
`git mv`). The planner drops an item itself when it can prove, against the
Goal and each unticked Done when line, that nothing is left: the need is gone
(a named commit removed it), the behaviour is there (a test on the installed
binary or a named commit), or a named item covers the rest. The proof goes in
`--reason`.

- **Gone names are not proof.** A search that finds nothing only shows old
  wording; the planner traces the replacement and tests there, then rewrites,
  keeps the rest, or makes it a call.
- **Never without Sumeet.** Items whose `ready.by` names him or whose body
  says he asked for it or sets his pick-up condition; work we may not want;
  priority or order between missions; forming a mission or dropping one not
  wholly stale. Started items and items in a mission another thread leads go
  to their owners.
- **The calls.** At most seven, numbered, each a sentence of evidence and a
  recommendation, put in conversation. A `needs-human` item holds them only
  when he is absent. The Size block, "parked" and the `groom` label are gone.
- **Report** order: counts, drops by kind with proof, changes to kept items,
  calls and how many wait, suggested order.
- `roles/planner.md` opens with the two scales and keeps Readiness and
  Review unchanged. `SKILL.md`'s description and Your Role route "go over,
  sweep or reorder the backlog" to the planner; the groom line is removed.
  `grep -ri groom skills/agent-factory` finds nothing.
- `docs/product/desk.md` (two lines) and `docs/architecture/desk.md` (one)
  say backlog pass instead of grooming; I rewrapped the two product lines.

Size: `planner.md` plus the procedure went from 96 lines (11 + 85) to 86
(16 + 70).

Checks: `git diff --check` is clean. No code changed, so no tests ran.
