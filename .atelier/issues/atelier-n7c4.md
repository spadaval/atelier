---
created_at: "2026-11-09T03:59:21.097704169+00:00"
id: "atelier-n7c4"
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
  - kind: "issue"
    id: "atelier-xb18"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make the orchestrator complete the mission or abort it, and say what a mission is for"
updated_at: "2026-11-09T03:59:21.097704169+00:00"
---

## Goal

`orchestrate.md` tells the orchestrator to keep going "without asking between steps" and lists four exceptions for stopping to ask, so agents bring the human mid-mission questions that planning should have settled. Replace that with one instruction, and say in `CONTEXT.md` that a mission is a batch the human reviews twice, so nothing inside it waits on the human (strategy `docs/strategies/mission-review-boundary.md`, Shape).

## Done when

- [ ] `procedures/orchestrate.md`'s Before Starting says: "Complete the mission. Abort only when its outcome can't be reached within the strategy, or carrying on would do harm." Aborting means stopping the mission's work, posting what was learned as a dated mission note, and reporting to the human, who replans with a planner. The four stop conditions are gone.
- [ ] Keeping the Plan Current: a strategy problem aborts instead of pausing for the human. The orchestrator still plans a later backlog increment at its evidence boundary within the approved strategy and Authority, with one independent planning review and no brief (as atelier-xb18's plan.md says); progress goes in dated mission notes, not messages to the human. Apart from aborting, orchestrate.md has no instruction to ask, consult or wait for the human before the mission closes; the Report quotes every remaining sentence that mentions the human or the user, and each is about aborting or closing.
- [ ] `CONTEXT.md`: Mission says it exists so the human reviews a batch of related work twice, its plan before it starts (plan review, `brief`) and its result after it merges (result review, `debrief`), with no human work inside it and human work after it as a loose end; Review names those two human reviews.
- [ ] The Report gives the orchestrator's read path before and after, without a target (Sumeet dropped the limits for this mission).

## Scope

`skills/agent-factory/procedures/orchestrate.md` and `CONTEXT.md`. Not the implementer's escalation, which atelier-ss4y adds to orchestrate.md after this merges.

## Context

The planner item (atelier-xb18) rewrites plan.md first; this item points at it.
