---
created_at: "2026-11-09T03:59:06.511545335+00:00"
id: "atelier-xb18"
issue_type: "task"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Give planning an owner: plan.md becomes the planner's procedure, with design, sizing, derisking and two review passes"
updated_at: "2026-11-09T03:59:06.511545335+00:00"
---

## Goal

Nobody owns planning today, so missions reach the human without a technical design, sizing or a plan for their riskiest assumption, after one review pass. Rewrite `skills/agent-factory/procedures/plan.md` as the procedure of a planner role that takes a mission from idea to the human's approval and hands it to an orchestrator, as `docs/strategies/mission-review-boundary.md` (Shape, Design) describes.

## Done when

- [ ] `plan.md` opens with the planner's directive, word for word: "Make sure a mission solves a real need, has a reasonably well-defined technical design, and is likely to succeed." Each later section serves one of its three clauses, and says which.
- [ ] Real need: the planner grooms the mission's area first (as today), sizes the mission on `references/sizing.md`'s three scales with a one-line reason each, and writes one sentence on why it is worth its cost against the cheaper alternative, including not doing it, in the strategy's new **Sizing** section.
- [ ] Design: a mission that adds a component, a crate, a service or an interface between crates has a design doc under `docs/architecture/` (components and where code lives, data flow, interfaces, dependencies chosen and why), reviewed with the plan; any other mission puts its design in the strategy's Shape. An implementation choice that is costly to reverse, such as a framework or a crate layout, is settled in the design, never left as an item's Unknown.
- [ ] Likely to succeed: the strategy's new **Derisking** section names the assumption that would hurt most if wrong and how the plan tests it early (a spike, a docs-first interface, or a first increment proving the outcome end to end), and items are ordered riskiest first. The existing "Confident enough to run unattended" test stays, folded into this.
- [ ] Readiness: a mission is ready only when nothing in its scope needs the human, apart from the boundaries its Authority reserves; a `needs-human` item inside a mission's scope fails readiness, and human work after the mission is a loose end.
- [ ] Review: every mission plan gets two independent planning review passes before the human's brief, from model families other than the planner's and different from each other when available, run in parallel without seeing each other's findings; a fix is re-reviewed by either. With one family, both passes still run as fresh agents. `references/atelier.md`'s Planning on the Base says the same in one line and links here.
- [ ] Handoff: after the human approves the brief, the planner hands the mission to an orchestrator; `SKILL.md`'s Subskills lists `plan` as the planner's role (take a mission to the human's approval), and SKILL.md stays within 100 lines.
- [ ] Read paths, counted by atelier-qj9d's rule: the planner's (SKILL.md, plan.md, every file it links, references/atelier.md) is at most 400 lines, and no other role's path grows past its limit or, where it is already over, past its current count. The Report gives a before-and-after table.

## Scope

`skills/agent-factory/procedures/plan.md`, `SKILL.md` and `references/atelier.md`. Not orchestrate.md (the next item) or `atelier brief` (its own item).

## Context

The strategy's own Sizing, Shape and Derisking sections were written by hand to this standard and can serve as its example. atelier-fyay (done) added the "Confident enough to run unattended" test this item folds in.
