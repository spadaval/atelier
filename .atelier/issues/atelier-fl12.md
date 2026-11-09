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
updated_at: "2026-11-09T07:05:44.712480976+00:00"
---

## Outcome

Agent Factory treats a mission as a human review boundary: the human reviews its plan once before it starts and its result once after it merges. A planner role takes a mission to approval, making sure it solves a real need, has a reasonably well-defined technical design and is likely to succeed, checked by two independent planning reviews from different model families; the brief shows its sizing and derisking at the top. An orchestrator completes an approved mission or aborts it back to planning, with no human work inside it. An implementer owns its code and review loop and escalates to the orchestrator. See `docs/strategies/mission-review-boundary.md`.

## Success criteria

- planner-end-to-end: a fresh planner agent given only the new skill replans atelier-yfqu from Sumeet's recorded direction, and its plan has a technical design, sizing with a worth-it sentence, a derisking plan with the riskiest item first, no human work inside the mission, and two recorded planning review passes from two model families other than the planner's; `atelier brief atelier-yfqu` shows the sizing and derisking at the top. Shown by a validation item read by hand.

## Background

Planned 2026-11-08 at Sumeet's direction, in the discussion that reframed the site mission (atelier-yfqu). atelier-ss4y, already planned and reviewed, joins this mission because it settles the implementer's side of the same question: who decides what.

## Notes

- 2026-11-09 [note] 2026-11-09: Orchestration started (Opus 5.5). Started atelier-xb18 and atelier-cfq9 in parallel on their own branches. Estimates: xb18 about 150-250 changed lines across plan.md, SKILL.md, references/atelier.md, skill-evaluation.md; cfq9 about 150-250 lines in mission_review.rs, its tests and docs/product/mission-reviews.md.
- 2026-11-09 [note] 2026-11-09: atelier-xb18 merged (d04c9b96, PR #68). Sol (xhigh) approved with no findings; CI passed. Decision: plan.md keeps the existing rule that a strategy's Authority may waive the human's approval of the brief. The human writes Authority, so a waiver is still the human's choice, and removing it is outside xb18's Done when. Flagged for the debrief. The Release workflow's dist install failed on the PR; it does not gate merges, and atelier-vqoy hit the same failure. Starting atelier-n7c4.
- 2026-11-09 [note] 2026-11-09: atelier-n7c4 merged (97f976ec, PR #70). Sol and Grok both found that aborting on any current hazard contradicted 'abort only when'; fixed so a hazard stops the harmful work first and aborts only if carrying on would still do harm; Sol re-approved. Grok's low finding on linking all of plan.md deferred as atelier-c1sr. Decision: both reviewers noted SKILL.md's Respect authority and references/atelier.md still send mid-mission questions to the human; filed as a child of atelier-ss4y (which edits both files) so one review covers them. atelier-cfq9 synced with master after n7c4 and is rerunning CI.
- 2026-11-09 [note] 2026-11-09: atelier-cfq9 merged (1a8d52c3, PR #69; Sol approved, no findings). atelier-ss4y merged with atelier-ixdi, atelier-pxme and atelier-ksme (91d3b2d0, PR #71). Sol and Grok found three wording defects (no-reviewer rule asked the user inside a mission; two next steps for a reviewer-recommended WON'T FIX; high-severity WON'T FIX parked on needs-human inside a mission); fixed in 2504f748 and re-approved by Sol. Decision: a reviewer-recommended WON'T FIX or NO ACTION is left undisposed in the implementer's summary for the orchestrator, not escalated, since it is routine. Grok's low R4 was fixed in passing as atelier-ksme. Installed atelier 0.3.1+91d3b2d0 and moved the live skill to master. Starting atelier-xc5y (orchestrator evals) and atelier-v3nm (planner validation) in parallel. The Release workflow keeps failing on PRs with a 504 downloading cargo-dist; it does not gate merges.
- 2026-11-09 [decision] 2026-11-09: atelier-v3nm's validator found planner-end-to-end passes (receipt atelier-ze4f), but marking it done on master made atelier land refuse for every thread (findings.disposed: planning approval atelier-4102 predates code merged since). I moved the done commit 26186c06 off master onto local branch atelier-v3nm so master lands again, pushed master (the yfqu replan's reviewed planning records), and asked Sol to review the validation at 26186c06; with an approval it goes back. Filed the land defect as atelier-ygul (backlog, outside this mission). The validator also found the two planning reviewers were blind to each other only by timing; plan.md says they must be blind but not how, so I added atelier-v5zk to this mission. Its code review also judges its two Done when lines; no separate planning review for a two-sentence item.
  Reason: Keep master landable for other threads, and keep the mission's two-pass rule true in practice, within the strategy's Authority to split items.
