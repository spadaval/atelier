---
schema: atelier.review
schema_version: 1
id: atelier-t7hl
title: 'crates/atelier-app/src/objective_graph.rs:53,158: inheritance is applied at every node of the transitive walk, not only to the issue being started. A done blocker that is a child of a parent with an open blocker now blocks downstream work. Repro (/tmp scratch): Q, X --parent Q, C blocked_by X; start and close X; then link Q blocked_by Z (open). C is blocked with ''transitive blocker path C -> X -> Z'' even though X, C''s only declared prerequisite, is done. Before this change C was ready. Narrow case (the parent gets a blocker after a child finished), but it stops completed work from unblocking dependents and is not covered by the record''s requirement. Fix: inherit the parent''s blockers only for the root issue (or only for non-terminal nodes), or document it as intended.'
target: atelier-e4c5
identity: Independent reviewer (Claude subagent)
commit: 7c7cb69cb8d1c0df8bf8d41c6bc6217a6d671ef0
created_at: 2026-10-16T00:51:31.862041759Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/objective_graph.rs:53,158: inheritance is applied at every node of the transitive walk, not only to the issue being started. A done blocker that is a child of a parent with an open blocker now blocks downstream work. Repro (/tmp scratch): Q, X --parent Q, C blocked_by X; start and close X; then link Q blocked_by Z (open). C is blocked with 'transitive blocker path C -> X -> Z' even though X, C's only declared prerequisite, is done. Before this change C was ready. Narrow case (the parent gets a blocker after a child finished), but it stops completed work from unblocking dependents and is not covered by the record's requirement. Fix: inherit the parent's blockers only for the root issue (or only for non-terminal nodes), or document it as intended.
