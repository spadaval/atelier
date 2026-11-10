---
schema: atelier.review
schema_version: 2
id: atelier-qtfj
title: finding on atelier-fcb7 @16cd3988
target: atelier-fcb7
identity: gpt-6-astra-plan-r2
commit: 16cd3988f7939ffc3cff2b08fd0facb6e42cc44a
created_at: 2026-11-10T01:17:34.071751297Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:181,221-238: drop is a base planning command allowed for started items, but readers prefer the unmerged work branch and that branch may change its own directory only there. Dropping a building item on base therefore leaves it visibly building and mergeable, preventing reliable cancellation and mission closure. Route drop of started work to its branch and define terminal precedence and merge refusal; test drop then mission close without merging abandoned code.
