---
created_at: "2026-11-09T05:55:07.731261036+00:00"
id: "atelier-pxme"
issue_type: "task"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-lfkn"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T06:31:56.101724212+00:00"
status: "done"
title: "Make SKILL.md's ask-a-human rules agree with complete-or-abort inside a mission"
updated_at: "2026-11-09T06:31:56.101724212+00:00"
---

## Goal

atelier-n7c4 made the orchestrator complete a mission or abort it, with no human work inside it. Two lines on every role's read path still send a mid-mission question to the human: SKILL.md's Respect authority ("Ask a human about product direction, risk tolerance and destructive or irreversible actions policy does not authorize") and references/atelier.md's Records ("agents leave it, and open decisions, to the human"). Both reviewers of atelier-n7c4 named them. Inside a mission, such a question is either settled within the strategy and Authority or aborts the mission; outside a mission the rules stand.

## Done when

- [x] SKILL.md's Respect authority and references/atelier.md's Records say, in as few words as possible, that inside an approved mission a question beyond the strategy and its Authority aborts the mission (as orchestrate.md says) rather than waiting on the human; outside a mission the existing rule is unchanged.
- [x] No other sentence on the orchestrator's or implementer's read path tells them to ask, consult or wait for the human mid-mission; the Report quotes any that remain and says why each is about aborting, closing or work outside a mission.

## Scope

SKILL.md and references/atelier.md only, on atelier-ss4y's branch.

## Notes

- 2026-11-09 [note] Implementer handoff: see ## Report. Independent review outstanding.

## Report

Outcome: both Done when lines hold at `80ec4fc4`. SKILL.md's Respect authority now reads "Ask a human about ... except inside an approved mission, where a question its strategy and Authority leave open aborts it", linking orchestrate.md's Before Starting; references/atelier.md's Records adds the same exception to "agents leave it, and open decisions, to the human". Outside a mission both rules are unchanged. A grep of the orchestrator's and implementer's read paths (SKILL.md, orchestrate.md, implement.md, references/atelier.md, review.md, decide.md, diagnose.md, validate.md and plan.md's Later Increments) for human, user, ask, consult and wait found these remaining, none a mid-mission wait:

- orchestrate.md: "report to the human, who replans with a planner": aborting.
- orchestrate.md: "walk the human through `atelier debrief`": closing.
- orchestrate.md: "Post progress as dated mission notes, not messages to the human": tells agents not to message the human.
- plan.md, Later Increments: "A change to the strategy itself goes back to a planner and the human": aborting to replan.
- implement.md: "Working alone, escalate to the user": a lone agent, outside a mission.
- SKILL.md, No reviewer: "ask the user to review or arrange it": a lone agent with no reviewer, outside an orchestrated mission.
- SKILL.md: "`install` only if the user asked": setup, outside a mission.
- references/atelier.md: a decision record when a choice "needs the human": a classification, not an ask.

### Deviations

- review.md's high-severity WON'T FIX line also told agents to label the item `needs-human` and leave the finding undisposed. It is outside this item's Scope but inside atelier-ss4y's, so atelier-ss4y changed it: inside an approved mission the orchestrator fixes or defers it instead.

### Changes and Proof

- Changes: SKILL.md's Respect authority, references/atelier.md's Records, at `80ec4fc4` on branch `atelier-ss4y`.
- Proof: the grep above, read by hand.

### Review

Independent review outstanding, with atelier-ss4y.
