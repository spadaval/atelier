---
schema: atelier.review
schema_version: 1
id: atelier-7i72
title: 'Re-reviewed master...task/atelier-28mz at 74e20495 (fix commits 36db2557 and 76e40ae7) against the rule Sumeet decided on 2026-10-15. Findings rzp3, hxua, 4d9x and esix are fixed; I agree with the no_action disposition of mqf2. parked_parent now requires a todo-category parent status that cannot start but has a transition into a startable todo status, so an open decision parks nothing. Classification parks only todo-category children; in-flight and blocked children keep their buckets. The transition check skips only children already in active work, so start from todo or blocked is refused and in-flight request_review/revise still work. The refusal names the parent''s way out (''plan''). Scratch repros: /tmp/r28c and /tmp/r28d, blocked child under a backlog parent is refused with no branch created, and starts after ''plan''. The integration test covers the backlog parent, the blocked child, the deferred parent with an in-flight child and the decision parent. The changed test expectations are not weaker. The work-model.md text matches the behaviour. Evidence atelier-kt1w: cargo nextest run --workspace passed at the fix commit 76e40ae7.'
target: atelier-28mz
identity: Independent reviewer (Claude subagent)
commit: 74e2049549d9e026114808e3a40f3d16c7842887
created_at: 2026-10-16T01:31:41.814068626Z
event:
  event: approval
---

Re-reviewed master...task/atelier-28mz at 74e20495 (fix commits 36db2557 and 76e40ae7) against the rule Sumeet decided on 2026-10-15. Findings rzp3, hxua, 4d9x and esix are fixed; I agree with the no_action disposition of mqf2. parked_parent now requires a todo-category parent status that cannot start but has a transition into a startable todo status, so an open decision parks nothing. Classification parks only todo-category children; in-flight and blocked children keep their buckets. The transition check skips only children already in active work, so start from todo or blocked is refused and in-flight request_review/revise still work. The refusal names the parent's way out ('plan'). Scratch repros: /tmp/r28c and /tmp/r28d, blocked child under a backlog parent is refused with no branch created, and starts after 'plan'. The integration test covers the backlog parent, the blocked child, the deferred parent with an in-flight child and the decision parent. The changed test expectations are not weaker. The work-model.md text matches the behaviour. Evidence atelier-kt1w: cargo nextest run --workspace passed at the fix commit 76e40ae7.
