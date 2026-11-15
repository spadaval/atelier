---
schema: atelier.item
schema_version: 3
id: atelier-b344
title: Tell planners to spike first when the goal is not clear until something is built
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- b97f3d4a-21be-42dd-a184-7a3f8581e372
ready: null
trivial: null
caused_by: null
created_at: 2026-11-15T07:22:39.247359188Z
---

## Goal

A planner should build a rough version of the whole thing with Sumeet
before it plans, when nobody can say what is wanted until it is seen. Today
the skill goes from his direction straight to a plan and two plan reviews,
and `docs/product.md` lists Derisking only as a section name. A planner
spikes only if he asks for one in conversation, though the Desk was built
that way (`spike/mission-control`, commit 6af629e2a).

## Done when

- [x] `skills/agent-factory/roles/planner.md` says when to spike (the goal is not clear until something is built or seen), how (the whole thing, rough, with Sumeet, on a `spike/<slug>` branch that never merges), and how it ends (a plan, or the spike thrown away).
- [x] It says what the plan records of a spike: its branch, what it showed, and whether the items rework its code or write fresh with it as reference.
- [x] It says a mission that changes code already understood needs no spike.
- [x] `SKILL.md`'s "Who Works With Sumeet" names the spike as part of the planner's work with him, and adds no role, mode or procedure file.
- [x] `docs/product.md` says a spike is how a mission whose goal is not clear until seen is derisked, and that its code reaches the base only through items and their reviews.
- [x] No command, record field or check changes.

## Scope

In: `skills/agent-factory/roles/planner.md`, `skills/agent-factory/SKILL.md`
and the Rules under Records in `docs/product.md`.

Out: a `procedures/spike.md` or a spike role, which Sumeet dropped with
atelier-y0qe on 2026-11-11 to keep the skill short; any rule that spikes
are required; the implementer's and reviewer's role files, since the plan
already tells an item what to do with a spike's code; the `atelier` tool.

## Context

Sumeet's direction, 2026-11-15, in conversation: his own way of working is
to spike a change end to end and rework it into proper code, and the
process should propose that. He confirmed atelier-y0qe's spike procedure was
dropped to streamline the skill, not because the idea was wrong. That item's
Context holds the reasoning: a spike is where rightness is found before a
mission is planned, and the deck was redirected five times, each time by
something built.

## Report

The planner's role file now tells a planner to spike before planning when
the goal is not clear until something is built or seen, in one paragraph
under "A Mission". `SKILL.md` gains one clause and `docs/product.md` one
rule. The change is 18 added lines and 1 removed, all prose; no code, command
or record format changed, so no test covers it and none was run beyond
`atelier check` and `git diff --check`, which pass.

One choice the reviewer may question: whether items start from the spike's
code is left to each plan. The Desk's mission moved the spike's app into
`desk/` (commit 29405f1f8), while a spike that only settled the shape is
better kept as reference, so one rule would be wrong for one of them.

### Validation

- `atelier check`: passes.
- `git diff --check`: no output.
- Line 3 of `skills/agent-factory/procedures/readiness.md` (links resolve):
  the change adds no link or path.
