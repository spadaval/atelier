---
created_at: "2026-11-09T17:52:26.858363528+00:00"
id: "atelier-zzpp"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-zzpp
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
status: "in_progress"
title: "Remove the strategy's power to waive the human's plan approval"
updated_at: "2026-11-09T17:53:37.529160821+00:00"
---

## Goal

plan.md's Brief and Handoff records the human's approval of the brief "unless the strategy's Authority delegates it", so a strategy can let an agent approve the plan in the human's place. That contradicts mission atelier-fl12's outcome: a mission is a boundary where the human reviews the plan and the result. Sumeet asked for it removed (2026-11-09).

## Done when

- [x] plan.md no longer lets a strategy's Authority delegate or waive the human's approval of the brief; the human approves every mission plan.
- [x] plan.md's Strategy File section says Authority covers what the orchestrator may decide inside the mission, never approving the plan or the result.
- [x] No other file in skills/agent-factory/, CONTEXT.md or docs/product/ says a strategy can waive the human's plan or result review; the Report lists what was searched.

## Scope

skills/agent-factory/procedures/plan.md, and any other file the third line finds.

## Notes

- 2026-11-09 [decision] Removed the clause in constitution.md and SKILL.md that lets a strategy grant authority to change itself; a strategy now changes only by human direction. The orchestrator's tactical freedom within Authority is unchanged.
  Reason: A self-authorized strategy change skips the human's plan approval that plan.md's Later Increments and Strategy Changes requires, so it is the waiver Done when line 3 asks to remove.

## Report

All three Done when lines hold; the proof is reading the diff and the searches below, since this is wording. plan.md's Brief and Handoff now records the human's approval with "no strategy can delegate it", and its Strategy File section says Authority covers what the orchestrator may decide inside the mission, never approving its plan or result. The search found one more waiver, of plan review for strategy changes: `constitution.md` (Strategy changes deliberately) and `SKILL.md` (Respect authority) let a strategy grant authority to change itself, while plan.md's Later Increments and Strategy Changes sends every strategy change back to the human for approval. Both now say a strategy changes only by human direction.

Searched, case-insensitive, in `skills/agent-factory/` (procedures, references, maintainers, `SKILL.md`, `constitution.md`), `CONTEXT.md` and `docs/product/`: `authority|delegat|waive|in the human's place|instead of the human`; `human('s)? (approv|review)|approv(al|es) (of|the) (brief|plan|mission|result|debrief)|without the human|unless`; `debrief|result review`; `grants|written in it`. Then the whole repository, Markdown, Rust, YAML and JSON, for the removed phrases (`Authority delegates`, `grants that authority`, `authority written in it`) and `evals/`, `docs/`, `skills/` for delegated approval. The only remaining hits are history in `.atelier/` records (atelier-2xlu's review, atelier-xb18's Report, this item's Goal). Result review (`orchestrate.md`'s close step, `atelier debrief`) had no waiver.

### Decisions

- Removing the strategy's self-granted authority in `constitution.md` and `SKILL.md` goes one step past plan.md; see the decision note.
