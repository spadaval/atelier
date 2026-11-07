---
schema: atelier.review
schema_version: 2
id: atelier-2xlu
title: finding on atelier-qj9d @857c333d
target: atelier-qj9d
identity: independent reviewer subagent (claude-opus-5-5), atelier-qj9d
commit: 857c333da6d978bcb4835106c7288c91840ec61d
created_at: 2026-11-07T19:01:50.435993606Z
event:
  event: finding
  recommendation: fix_now
---

Skill e2671ec, procedures/plan.md:63-70: the human's plan approval is no longer a condition of a mission being ready. At 4d88ffb (plan.md:92-112) 'draft to todo' required the planning reviewed AND the human's approval, unless the strategy's Authority delegates it. Now the ready conditions end at 'the planning has been reviewed', and 'The human then approves the plan' follows as a later step. Atelier's own brief footer orders approval first, then the transition (docs/product/mission-reviews.md:127-135). No other file in the new skill says work waits for the plan approval: orchestrate's Before Starting asks only for an agreed outcome or strategy. Failure: once agent review passes, a planner runs ready, and an orchestrator takes the todo mission and starts work before the human has walked the brief. Atelier allows ready with no approval and has no transition back to draft. A fresh Sonnet planner given only the new skill ran ready before any review and hand-edited the record back. It skipped the check rather than misreading 'then', so this shows only that the skill text is the sole guard. Fix, at no line cost: make the approval one of the conditions, e.g. '...; and the planning has been reviewed and the human has approved it after a walk through atelier brief <mission> --html (recorded under their name with atelier review approve <mission> --identity <the human>), unless the strategy's Authority section delegates that.'
