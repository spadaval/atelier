---
schema: atelier.review
schema_version: 1
id: atelier-n3f5
title: 'crates/atelier-app/src/objective_graph.rs:10 (effective_blockers): a parent blocked_by its own child turns into a self-cycle for that child. Repro (/tmp scratch): P, K --parent P, K2 --parent P, issue link P K --role blocked_by (accepted by issue link, worked on master). Now work ready lists nothing; issue transition K shows start [blocked] ''task K has direct blocker path K -> K''; atelier start K exits 1; K2 is also blocked on K. Graph is permanently deadlocked (K can never start, so P can never close). Because DependencyClosure::blocking_ids always includes cycle-path ids regardless of terminal status, the merge/land gates that call dependency_closure for scoped ids (review_gate.rs:387 blockers.none_open, review_gate.rs:575 blockers.transitive_none_open on terminal children) would report K as an open blocker even after hand-editing K to done (derived from code, not run). This regresses a previously valid graph and touches the merge/land-gate constraint. Fix: when inheriting, drop the issue itself and the parent''s blockers that are the parent''s own children (a parent waiting on its children is internal ordering, not a prerequisite for the children); add a test for parent blocked_by own child.'
target: atelier-e4c5
identity: Independent reviewer (Claude subagent)
commit: 7c7cb69cb8d1c0df8bf8d41c6bc6217a6d671ef0
created_at: 2026-10-16T00:51:31.751744353Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/objective_graph.rs:10 (effective_blockers): a parent blocked_by its own child turns into a self-cycle for that child. Repro (/tmp scratch): P, K --parent P, K2 --parent P, issue link P K --role blocked_by (accepted by issue link, worked on master). Now work ready lists nothing; issue transition K shows start [blocked] 'task K has direct blocker path K -> K'; atelier start K exits 1; K2 is also blocked on K. Graph is permanently deadlocked (K can never start, so P can never close). Because DependencyClosure::blocking_ids always includes cycle-path ids regardless of terminal status, the merge/land gates that call dependency_closure for scoped ids (review_gate.rs:387 blockers.none_open, review_gate.rs:575 blockers.transitive_none_open on terminal children) would report K as an open blocker even after hand-editing K to done (derived from code, not run). This regresses a previously valid graph and touches the merge/land-gate constraint. Fix: when inheriting, drop the issue itself and the parent's blockers that are the parent's own children (a parent waiting on its children is internal ordering, not a prerequisite for the children); add a test for parent blocked_by own child.
