---
schema: atelier.review
schema_version: 1
id: atelier-2tse
title: atelier-h867 done-when asked to measure removed overhead 'including separate source/base checkouts'; the close note says a separate base worktree was not timed. The removed scan was linear per checkout and is now gone from both checkouts (merge_gates.rs:1494 covers both), so the base-checkout saving should be the same order (about 70 ms per 20k files). The missing number does not change the decision.
target: atelier-1ioo
identity: independent-reviewer-2026-10-15
commit: d1cb7d908869c6ddadafcc69416ddf7aea42df75
created_at: 2026-10-15T19:13:37.256211954Z
event:
  event: finding
  recommendation: no_action
---

atelier-h867 done-when asked to measure removed overhead 'including separate source/base checkouts'; the close note says a separate base worktree was not timed. The removed scan was linear per checkout and is now gone from both checkouts (merge_gates.rs:1494 covers both), so the base-checkout saving should be the same order (about 70 ms per 20k files). The missing number does not change the decision.
