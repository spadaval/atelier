---
created_at: "2026-11-09T17:52:26.858363528+00:00"
id: "atelier-zzpp"
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
title: "Remove the strategy's power to waive the human's plan approval"
updated_at: "2026-11-09T17:52:26.858363528+00:00"
---

## Goal

plan.md's Brief and Handoff records the human's approval of the brief "unless the strategy's Authority delegates it", so a strategy can let an agent approve the plan in the human's place. That contradicts mission atelier-fl12's outcome: a mission is a boundary where the human reviews the plan and the result. Sumeet asked for it removed (2026-11-09).

## Done when

- [ ] plan.md no longer lets a strategy's Authority delegate or waive the human's approval of the brief; the human approves every mission plan.
- [ ] plan.md's Strategy File section says Authority covers what the orchestrator may decide inside the mission, never approving the plan or the result.
- [ ] No other file in skills/agent-factory/, CONTEXT.md or docs/product/ says a strategy can waive the human's plan or result review; the Report lists what was searched.

## Scope

skills/agent-factory/procedures/plan.md, and any other file the third line finds.
