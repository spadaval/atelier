---
schema: atelier.review
schema_version: 2
id: atelier-5lh1
title: finding on atelier-fcb7 @36de8f1a
target: atelier-fcb7
identity: gpt-6-astra-amend2-r1
commit: 36de8f1abc4b2ec76b1eb30ca8b9f85588d82929
created_at: 2026-11-10T03:59:39.904481016Z
event:
  event: finding
  recommendation: fix_now
---

high, docs/strategies/redesign-architecture.md:288 and atelier-mal9: the promised history of acts cannot be reconstructed from the specified records. The selector misses escalate, decide, set and mission verdicts because none changes status, holder or settlement; decide stores its actor but the selector never reads it. Ready records the authorizer, not the executing thread; drop, close and finding settlement store no actor, so using the last implementer or lead misattributes them. Planning may batch several acts into one hand-made commit, losing intermediate states entirely. Mission 4 cannot recover information mission 2 never persisted. Smallest fix: define the exact logged events and persist the minimum actor and event information in the records written by those commands now, including planning and settlement, or explicitly narrow the promised log; align mal9 and the detection rule, and remove the claim that every act changes status.
