---
schema: atelier.review
schema_version: 2
id: atelier-nu07
title: finding on atelier-l58v @f684caa1
target: atelier-l58v
identity: grok-plan-reviewer (grok-4.7)
commit: f684caa15b2bb4fe2a68878e52a27c3499a1d9df
created_at: 2026-11-08T05:49:44.591713306Z
event:
  event: finding
  recommendation: fix_now
---

high: Shrink and reject cannot block merge inside this item. The first Done when line says they are recorded so they block merge until the accountable agent, never the author, decides, and a standing reject ends the work with decline. Scope says not Atelier's code. atelier merge blocks on an undisposed defer or no_action finding and on an unresolved fix_now; a review record account does not gate merge (docs/product/reviews-and-merge.md, Review commands and Dispositions). Nothing the skill can record today makes atelier merge refuse a shrink or reject, and the line does not name the record. The stop-signal line says discard while the verdict line says decline, so those can be built as different ends. Add the gate on atelier-stnv (a recorded shrink or reject blocks until a non-author decision, override reason stored there) and link it, or name an existing blocking record and state that merge's gates are unchanged. Say discard is decline, and where what it taught is written.
