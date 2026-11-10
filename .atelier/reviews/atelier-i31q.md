---
schema: atelier.review
schema_version: 2
id: atelier-i31q
title: finding on atelier-fcb7 @92440ff2
target: atelier-fcb7
identity: gpt-6-astra-plan-r1
commit: 92440ff2e8f3329a98a00693863a77326e86d044
created_at: 2026-11-10T01:00:00.292554320Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:171-173: exempting all .atelier paths from review freshness now exempts config.toml and the entire mission design, so a reviewed branch can install weakened future checks or changed requirements without re-review. Reading config from base protects only this merge. Classify substantive config/design/requirement changes separately from routine status/report records and invalidate the relevant approval for those changes.
