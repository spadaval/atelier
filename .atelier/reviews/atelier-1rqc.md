---
schema: atelier.review
schema_version: 1
id: atelier-1rqc
title: Resolved. merge_resolution requires the merge tree to equal git merge-tree outside conflicted record paths, so an unlink in a non-conflicted record during a base merge goes stale, and a conflicted out-of-scope record must be a YAML three-way merge, so the base's blocks entry cannot be dropped. Covered by drop_blocker (stale) and keep_blocker (review.current passes, blockers.none_open fails).
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: decb65a8e086ca314995b9534104ce64d67261eb
created_at: 2026-10-16T20:21:12.434562700Z
event:
  event: disposition
  finding: atelier-e2xv
  decision: fix_now
  supersedes: null
  resolution_commit: 9c48a595e3a3a3799103820a7a2609c30488fd24
  follow_up: null
---

Resolved. merge_resolution requires the merge tree to equal git merge-tree outside conflicted record paths, so an unlink in a non-conflicted record during a base merge goes stale, and a conflicted out-of-scope record must be a YAML three-way merge, so the base's blocks entry cannot be dropped. Covered by drop_blocker (stale) and keep_blocker (review.current passes, blockers.none_open fails).
