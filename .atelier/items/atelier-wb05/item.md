---
schema: atelier.item
schema_version: 3
id: atelier-wb05
title: Rewrite garden.md so a run has a stated purpose, a focus and transcripts as its main evidence
status: draft
mission: atelier-g961
labels:
- agent-factory
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-12T01:03:20.840469739Z
---

## Goal

Rewrite the gardener's procedure so a run is a look at a repository's health
with Sumeet in the loop, and not a measuring routine. Today `garden.md` never
states its purpose, reads threads last and only up to their first edit, and
gives the code one sentence. Seven of the nine proposals from the first five
runs restate a checklist line.

## Done when

- [ ] **Purpose.** `garden.md` opens, before step 1, with at most 15 lines saying what gardening is for (a repository that stays easy to navigate, change and validate, and the redesigns that need Sumeet's taste), what a run produces, and what it is not: a linter, a firefighter, or the backlog pass.
- [ ] **Areas.** It lists the areas a run may focus on, each in a line saying what to read: structure and design drift; tests (what they prove, speed, flakiness); dependencies and toolchain; features and code nobody uses; how work flows (the `atelier log` numbers); the host and the agents' setup (worktrees, hooks, skills, memory files); Sumeet's time (how often and how long work waited on him); cost.
- [ ] **Focus.** A run starts with a focus: the one Sumeet names, or, when he names none, the area the earlier pages show longest unlooked at. The run item's Goal and the page's first line record it. The numbers and the checklist are taken every run whatever the focus.
- [ ] **Threads first.** Step 4 has the run choose threads by the numbers (the items longest in each status, those with the most rounds, every escalation and every skip) and read them in full, implementer and reviewer both. The cap of three threads read to their first edit is gone. It says cheaper agents may extract trips from threads in bulk, and that the gardener checks each claim against the thread before it counts as evidence.
- [ ] **Code.** In the focus area the run reads the code itself, not only docs against code: what each module is for, where two parts solve one problem two ways, what has outgrown its design. A proposal about structure names the files and what the change would make easier.
- [ ] **Kept.** These are unchanged in meaning: a run changes only upkeep and signs on its own branch; anything that adds or removes a rule, check, gate, interface or default is a proposal; at most three proposals, each one cause with counted evidence; the page is at most 60 lines; the run ends in a session with Sumeet.
- [ ] **Not from the checklist alone.** The procedure says a proposal that only restates a line of `readiness.md` goes in the page's Checklist line, and takes one of the three places only when a thread or a number shows what the gap cost.
- [ ] **Backlog.** The page gives the open counts as today, and says in one line when a backlog pass looks due and why. The run does not do that pass.
- [ ] **The first runs' guesses.** Each gap atelier-h201 listed is closed in `garden.md` or `readiness.md`, or the Report says which change made it moot: where a repository other than atelier finds the Scorecard's measures; whether open counts include the run's own item; the wording for a window with nothing merged and for a first run's Last run column; what a first run with nothing merged reads; what becomes of a bug the run files; where a page names items found stale; a checklist line with mixed results, and whether it is judged on the base; the four lines two runs read differently (2 and 4, 7, 8, 9); and line 3's script reporting home paths, URLs and routes as missing files.
- [ ] **Docs.** The Gardener row of the table in `docs/product.md` and the "Who Works With Sumeet" section of `SKILL.md` match the new procedure, including that Sumeet may set the focus at the start.
- [ ] **Size.** `garden.md` and `readiness.md` together are no longer than 298 lines, and the Report gives both counts before and after.

## Scope

In: `skills/agent-factory/procedures/garden.md` and `readiness.md`; the "Who Works With Sumeet" section of `SKILL.md`; the Gardener row and the paragraph on the gardener's procedure in `docs/product.md`.

Out: `SKILL.md`'s description and "Your Role" section, which atelier-s47q changes; any script or command; the definition of any Scorecard measure; a schedule for runs.

## Context

The first runs' full lists of guesses are in atelier-613a's Report; atelier-h201 summarised them and was dropped into this item. The design note for the current procedure is `.atelier/missions/atelier-295u/design.md`; its "Numbers first, transcripts second" is what this item reverses, for the reason in this mission's Why. `procedures/groom.md` already shows the pattern of bulk readers with each claim checked.
