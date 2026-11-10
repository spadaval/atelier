---
schema: atelier.review
schema_version: 2
id: atelier-zxcv
title: finding on atelier-fcb7 @d0d69b90
target: atelier-fcb7
identity: gpt-6-astra-plan-r1
commit: d0d69b90bfe0995d2a2ac9e34abcf336897402fb
created_at: 2026-11-10T01:00:00.604753219Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:154-199 and atelier-engh: the act table permits start only from ready while the ready item requires takeover from building; neither defines takeover from in_review, changes_requested or approved, or reviewer replacement after death. A dead implementer after approval leaves nobody authorized to merge. Define idempotent resume/rebind for every active state without resetting status, preserving role separation, and add recovery scenarios before the first eval.
