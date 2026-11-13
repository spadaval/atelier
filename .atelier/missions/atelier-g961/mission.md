---
schema: atelier.mission
schema_version: 3
id: atelier-g961
title: Split the human-facing work into a planner who owns what to build and a gardener who reads how it went
status: ready
labels:
- agent-factory
lead: mcp:5dded1ba-9616-4c3e-8ca1-9e7b35bdaca5
ready:
  by: sumeet, in conversation on 2026-11-13
  at: 2026-11-13T08:27:11.885174987Z
created_at: 2026-11-12T01:02:51.245176518Z
---

## Why

Two roles work with Sumeet, and neither does the job he wants from it.

The gardener is meant to keep a repository healthy and bring him the redesigns
that need his taste. As written it is a measuring routine. `garden.md` never
says what gardening is for. It reads agents' threads last: at most three, each
only up to its first edit, plus any that explain a number that moved. Its look
at the code is a nine-line checklist and one sentence. Sumeet joins only after
the page is written. The first five runs (atelier-613a) show the result: seven
of nine proposals restate a checklist line, four as "run the documented checks
at merge" and three as "give bug items a cause". None came from watching an
agent struggle, which is where the richest evidence is.

Groom is planning done across the whole backlog, but it is written as a third
thing with its own digest. The agent may fix only a label or a format error and
must ask Sumeet about every other entry. The one digest it produced,
atelier-8i86, held 21 entries, waited weeks, and the cutover overtook 19 of
them. The sweep of 2026-11-11 worked the other way: the agent dropped 104
records it could prove stale, each with its evidence in the drop reason, and
brought Sumeet six calls, which he answered in one message.

## Outcome

The skill has two roles that work with Sumeet, with one question each. The
planner owns what to build, at two scales: one mission, and the whole backlog.
The gardener owns the state of what was built and how well we build it.

A gardening thread learns from `garden.md` what gardening is for. It starts
from a focus Sumeet names, or picks the area longest unlooked at. It uses the
numbers from `atelier log` to choose which threads to read, reads those in
full, and reads the code's structure in the focus area. It still fixes only
upkeep and signs, still brings at most three proposals as causes with counted
evidence, and still ends in a session with him.

A planner going over the backlog follows `backlog.md`. It drops what it can
prove stale or duplicated, and brings Sumeet a short list of calls with a
recommendation on each. The word "groom" is gone from the skill.

## Success criteria

- purpose-and-focus: `garden.md` opens by saying what gardening is for and
  what it is not, lists the areas a run may focus on, and says how a run gets
  its focus and records it. A thread told only "I want to do some gardening"
  can say back what it will look at and why before it measures anything.
- transcripts-first: the procedure has a run choose threads by the numbers, in
  a stated order up to a stated number of items, and read each one's
  implementer and one reviewer thread in full, with no cap
  that stops at a thread's first edit. It says how cheaper agents may do the
  bulk reading with each claim checked, and that no transcript text reaches
  the page.
- trial: one run in atelier under the new procedure, focused on structure, is
  approved and waits for its session. At least one of its proposals rests on
  a trip the gardener checked in a thread, given as the thread's ID and what
  happened; at least one is about the code's structure; and none only
  restates a line of `readiness.md`.
- one-planner: `SKILL.md` sends a request to plan, to shape, or to go over the
  backlog to `planner.md`, which names both scales. `grep -ri groom
  skills/agent-factory` finds nothing.
- drops-without-asking: `backlog.md` lets the planner drop an item once it has
  proved, against the item's Goal and unticked Done when lines, that the need
  is gone, already met, or covered by a named replacement. A name the code no
  longer has is not proof. It gives a test, readable from the records, for the
  items it may never drop without Sumeet. The calls it brings him are a
  numbered list of at most seven, each with a recommendation.
- no-growth: `garden.md` and `readiness.md` together are no longer than
  today's 298 lines, and `planner.md` and `backlog.md` together are no longer
  than today's 96 (`planner.md` 11, `groom.md` 85).

## Scope

- In: `skills/agent-factory/SKILL.md`, `roles/planner.md`,
  `procedures/garden.md`, `procedures/readiness.md`, `procedures/groom.md`
  (renamed `backlog.md`); the lines of `docs/product.md`,
  `docs/product/desk.md` and `docs/architecture/desk.md` that describe the
  gardener or name grooming; one gardener run in atelier.
- Out: any change under `crates/` or `desk/`; a command that starts a run or
  takes the numbers; a schedule for the gardener (atelier-jy8k, which should
  become weekly or on request and is replanned after this mission); the
  sessions on the five first pages, which settle under the procedure that
  produced them; runs in the other four repositories; evals for either
  procedure.
- Target environment: this repository and this host.

## Authority

The lead may choose the wording, the list of focus areas, the page's layout,
and the order and number of threads a run reads. It may not give a gardener
run the right to add or remove a rule, check, gate, interface or default; let
the planner drop an item Sumeet asked for or set aside himself (one whose
`ready.by` names him, or whose body says he asked for it or gives his
condition for picking it up), a started item, or an item in a mission another
thread leads; let transcript text, a thread's title, a home path or a
personal detail onto a page, which reaches public Git; raise the limit of
three proposals; or remove the session with Sumeet.

## Result
