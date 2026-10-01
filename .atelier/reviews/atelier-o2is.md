---
schema: atelier.review
schema_version: 1
id: atelier-o2is
title: Prune apply does not hold an exclusive transaction and can leave partial record removals after a failure.
target: atelier-1ioo
identity: records-reviewer
commit: 8a4b850d103109db66e6672d1cd3391fb604e6ae
created_at: 2026-10-01T22:21:38.038230005Z
event:
  event: finding
  recommendation: fix_now
---

Prune apply does not hold an exclusive transaction and can leave partial record removals after a failure.
