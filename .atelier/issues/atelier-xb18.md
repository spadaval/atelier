---
created_at: "2026-11-09T03:59:06.511545335+00:00"
id: "atelier-xb18"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-xb18
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
closed_at: "2026-11-09T05:42:03.496185448+00:00"
status: "done"
title: "Give planning an owner: plan.md becomes the planner's procedure, with design, sizing, derisking and two review passes"
updated_at: "2026-11-09T05:42:03.496185448+00:00"
---

## Goal

Nobody owns planning today, so missions reach the human without a technical design, sizing or a plan for their riskiest assumption, after one review pass. Rewrite `skills/agent-factory/procedures/plan.md` as the procedure of a planner role that takes a mission from idea to the human's approval and hands it to an orchestrator, as `docs/strategies/mission-review-boundary.md` (Shape, Design) describes.

## Done when

- [x] `plan.md` opens with the planner's directive, word for word: "Make sure a mission solves a real need, has a reasonably well-defined technical design, and is likely to succeed." Each later section serves one of its three clauses, and says which.
- [x] Real need: the planner grooms the mission's area first (as today), sizes the mission on `references/sizing.md`'s three scales with a one-line reason each, and writes one sentence on why it is worth its cost against the cheaper alternative, including not doing it, in the strategy's new **Sizing** section.
- [x] Design: every mission's strategy has a **Design** section giving the technical design (components and where code lives, data flow, interfaces, dependencies chosen and why), proportionate to the mission; it goes on the base with the strategy as planning and is reviewed with it, and the items that build a component bring `docs/architecture/` up to date. An implementation choice that is costly to reverse, such as a framework or a crate layout, is settled in the design, never left as an item's Unknown. plan.md's Strategy File section lists Sizing, Design and Derisking.
- [x] Likely to succeed: the strategy's new **Derisking** section names the assumption that would hurt most if wrong and how the plan tests it early (a spike, a docs-first interface, or a first increment proving the outcome end to end), and items are ordered riskiest first. The existing "Confident enough to run unattended" test stays, folded into this.
- [x] Readiness: a mission is ready only when nothing in its scope needs the human, apart from the boundaries its Authority reserves; a `needs-human` item inside a mission's scope fails readiness, and human work after the mission is a loose end.
- [x] Review: every mission plan gets two independent planning review passes before the human's brief, from model families other than the planner's and different from each other when available, run in parallel without seeing each other's findings; a fix is re-reviewed by either. With one family, both passes still run as fresh agents. A later increment of an approved mission, planned by the orchestrator within its strategy and Authority, gets one independent planning review and no brief; a change to the strategy goes back to a planner and the human. `references/atelier.md`'s Planning on the Base says the same in one line and links here.
- [x] Handoff: after the human approves the brief, the planner hands the mission to an orchestrator; `SKILL.md`'s Subskills lists `plan` as the planner's role (take a mission to the human's approval), and SKILL.md stays within 100 lines.
- [x] `maintainers/skill-evaluation.md` agrees with the new contract: its "Strategy as design doc" row (and any other row that sends design out of the strategy) expects the design in the strategy's Design section, and plan.md's strategy length guidance ("under about 150 lines") allows for it.
- [x] The Report gives the planner's read path (SKILL.md, plan.md, every file it links, references/atelier.md) and every role's path whose count changed, before and after, without a target (Sumeet dropped the limits for this mission).

## Scope

`skills/agent-factory/procedures/plan.md`, `SKILL.md`, `references/atelier.md` and the planning rows of `maintainers/skill-evaluation.md`. Not orchestrate.md (the next item) or `atelier brief` (its own item).

## Context

The strategy's own Sizing, Shape and Derisking sections were written by hand to this standard and can serve as its example. atelier-fyay (done) added the "Confident enough to run unattended" test this item folds in.

## Notes

- 2026-11-09 [decision] Estimate: about 150-250 changed lines across plan.md, SKILL.md, references/atelier.md and skill-evaluation.md.
  Reason: The assignment gave this estimate; the stop signal is about twice it.
- 2026-11-09 [decision] Strategy File is tagged as serving the technical design clause: the strategy is the mission's design document.
  Reason: Done when asks each section to serve one clause; the Design section now lives in the strategy, so the strategy file is the design document, and tagging it 'all three' would break the one-clause pattern.
  Rejected: Tag Strategy File as serving all three clauses
- 2026-11-09 [decision] Kept the 'Shaping a Mission' and 'Ready Work' headings; replaced Layers, Strategic Revision and Handoff with Derisking, Planning Review, Brief and Handoff, and Later Increments and Strategy Changes.
  Reason: ADR 0024, docs/product/mission-authoring.md and docs/product/work-model.md link #shaping-a-mission and #ready-work; nothing outside records links the removed anchors.
- 2026-11-09 [decision] Kept the existing exception that the strategy's Authority may delegate the human's approval of the brief.
  Reason: Settled by atelier-qj9d's review (finding atelier-2xlu); removing it is a behavior change the settled decisions do not ask for. Flagged for review.
- 2026-11-09 [decision] Added no new pressure-scenario rows to skill-evaluation.md; only the Strategy as design doc row changed.
  Reason: No other planning row sends design out of the strategy, and atelier-v3nm validates the new planner end to end; unrun rows would be untested guidance.

## Report

### Outcome

`skills/agent-factory/procedures/plan.md` is now the planner's procedure, and every Done when line holds. It opens with the directive word for word (plan.md:3-4). Each of its eight sections begins by naming the clause it serves: Shaping a Mission and Brief and Handoff serve the real need; Design and Strategy File the technical design; Derisking, Ready Work, Planning Review and Later Increments and Strategy Changes the likelihood of success. Grooming first, sizing on `references/sizing.md`'s three scales with a reason each and the worth-it sentence are in Shaping a Mission (plan.md:16-28). The strategy's Design section, costly-to-reverse choices settled there and never an item's Unknown, and items updating `docs/architecture/` are in Design (plan.md:36-50). Derisking names the riskiest assumption, its early test and riskiest-first order, and folds in "Confident enough to run unattended" and the evidence-boundary rule from the old Layers section (plan.md:52-74). Strategy File lists Sizing, Design and Derisking and keeps "under about 150 lines" apart from the Design (plan.md:76-96). Ready Work fails a mission with human work in scope beyond Authority's reserved boundaries, and sends human work after it to a loose end (plan.md:98-112). Planning Review gives the two parallel, blind passes from other model families, both still run with one family, and re-review of a fix by either (plan.md:123-135). Later increments get one review and no brief; a strategy change goes back to a planner and the human (plan.md:148-158). `references/atelier.md`'s Planning on the Base adds one sentence linking `plan.md#planning-review`. SKILL.md's Subskills lists `plan` first as "the planner's role: take a mission to the human's approval", at 99 lines. skill-evaluation.md's "Strategy as design doc" row expects the design in the strategy's Design section; no other row sends design out of the strategy.

### Changes

- `skills/agent-factory/procedures/plan.md`: rewritten (100 to 158 lines).
- `skills/agent-factory/SKILL.md`: Subskills line for `plan`, reflowed (98 to 99 lines).
- `skills/agent-factory/references/atelier.md`: one sentence in Planning on the Base (105 to 107 lines).
- `skills/agent-factory/maintainers/skill-evaluation.md`: the "Strategy as design doc" row.
- About 231 changed lines in the skill (146 added, 85 removed), within the 150-250 estimate.

### Proof

- `git diff --check`: clean.
- `atelier check atelier-xb18`: "Check passed."
- A link check over plan.md, references/atelier.md and SKILL.md (every relative link's file exists and its anchor matches a heading): 0 broken. The anchors docs link to (`#shaping-a-mission`, `#ready-work`) are kept; `grep` finds no non-record link to the removed `#layers`, `#strategic-revision` or `#handoff`.
- Read-path line counts, master (c901e445, skills identical to 4d6b74e4) before and this branch after. A role's path is SKILL.md, its procedure, references/atelier.md and the files its procedure links directly. No target applies.
  - plan: 303 to 378 (SKILL.md 98 to 99, plan.md 100 to 158, references/atelier.md 105 to 107, and references/sizing.md 14, newly linked).
  - Every other role changes by +3 (SKILL.md +1, references/atelier.md +2): audit 251 to 254, decide 236 to 239, diagnose 238 to 241, garden 300 to 303, groom 298 to 301, implement 276 to 279, install 262 to 265, migrate 248 to 251, orchestrate 455 to 458, review 324 to 327, validate 242 to 245.
  - All agent-facing skill Markdown (excluding maintainers/): 1262 to 1323.
- Readability, the riskiest assumption, is judged by review, not by me; no eval was run. atelier-v3nm runs a fresh planner on a real replan.

### Decisions

See the decision notes: Strategy File is tagged as serving the design clause; the two linked headings are kept; Authority's delegation of the human's approval is kept; no new evaluation rows.

### Review

Independent review outstanding.

### Open items

- The "unless the strategy's Authority delegates it" exception to the human's approval stays from the old text. It sits uneasily with a mission as a human review boundary; the reviewer or Sumeet may want it gone.
- `orchestrate.md` still says to pause and route a strategy problem through `plan`, and does not mention the one-review later increment; atelier-n7c4 owns that.
- This mission's own strategy (`docs/strategies/mission-review-boundary.md`) has its design under Shape rather than a Design section. It was written before this standard and is out of scope here.
