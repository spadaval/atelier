---
created_at: "2026-11-09T03:58:51.764237065+00:00"
id: "atelier-fl12"
issue_type: "mission"
labels:
- "agent-factory"
strategy:
  path: docs/strategies/mission-review-boundary.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make each mission a human review boundary: a planner gets it approved, an orchestrator completes it or aborts"
updated_at: "2026-11-09T05:34:33.430472253+00:00"
---

## Outcome

Agent Factory treats a mission as a human review boundary: the human reviews its plan once before it starts and its result once after it merges. A planner role takes a mission to approval, making sure it solves a real need, has a reasonably well-defined technical design and is likely to succeed, checked by two independent planning reviews from different model families; the brief shows its sizing and derisking at the top. An orchestrator completes an approved mission or aborts it back to planning, with no human work inside it. An implementer owns its code and review loop and escalates to the orchestrator. See `docs/strategies/mission-review-boundary.md`.

## Success criteria

- planner-end-to-end: a fresh planner agent given only the new skill replans atelier-yfqu from Sumeet's recorded direction, and its plan has a technical design, sizing with a worth-it sentence, a derisking plan with the riskiest item first, no human work inside the mission, and two recorded planning review passes from two model families other than the planner's; `atelier brief atelier-yfqu` shows the sizing and derisking at the top. Shown by a validation item read by hand.

## Background

Planned 2026-11-08 at Sumeet's direction, in the discussion that reframed the site mission (atelier-yfqu). atelier-ss4y, already planned and reviewed, joins this mission because it settles the implementer's side of the same question: who decides what.

## Notes

- 2026-11-09 [note] 2026-11-09: Orchestration started (Opus 5.5). Started atelier-xb18 and atelier-cfq9 in parallel on their own branches. Estimates: xb18 about 150-250 changed lines across plan.md, SKILL.md, references/atelier.md, skill-evaluation.md; cfq9 about 150-250 lines in mission_review.rs, its tests and docs/product/mission-reviews.md.
