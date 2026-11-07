---
schema: atelier.review
schema_version: 2
id: atelier-wx64
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:50:03.726431292Z
event:
  event: finding
  recommendation: fix_now
---

medium: PRODUCT_INTENT.md:12 'Git carries both the implementation and its done claim through review and integration' is superseded by decision 2 (ADR 0028 lines 228-230: the done claim is no longer committed on the work branch and reviewed with the code), but ADR 0028's PRODUCT_INTENT.md list (docs/adr/0028-simpler-lifecycle.md:275-300) omits it. atelier-jnrh will update PRODUCT_INTENT.md from that list ('describe the mechanics as built'), so this line would stay stale and contradict the new model. Fix: add it to the list beside 'Commit done claims on the work branch' (atelier-jnrh).
