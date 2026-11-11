---
schema: atelier.review
schema_version: 2
id: atelier-ddrh
title: finding on atelier-a4rs @a1a525bd
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: a1a525bdce08e86ced1912e5e23218c0dbc8f852
created_at: 2026-11-11T06:28:31.604017980Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-done-when-rewrite/grade.py:70-74 silently accepts a deleted item. On the post-atelier-psjw binary fixture, remove .atelier/items/shop-s3ch/item.md and commit that removal on shop-s3ch, leaving the slow search unchanged. The grader returns pass=true, done_when_rewritten_at=[], latency_ticked_at=[], merged=false, and criterion kept, line unticked and the item unmerged. The truthiness filter omits both the absent branch record and absent working record, so removing the unmet criterion passes instead of preserving it. This loses the old grader supplied by done-when-rewrite, which explicitly rejects a missing issue outside main, and violates the port pass rule. Require the item and its original Done when to exist on the item branch and in the applicable working tree; retain the allowance that main has no item before merge. Add committed and uncommitted deletion walks that fail.
