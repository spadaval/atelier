---
schema: atelier.review
schema_version: 1
id: atelier-hxua
title: 'crates/atelier-app/src/work_classification.rs:169-175 and crates/atelier-cli/src/commands/workflow.rs:245-253 apply parking to children in any status, not just children that have not started. The work workflow allows ''defer'' (todo/blocked to backlog) on a parent whose child is in_progress (parent stays todo when a child starts). Repro in /tmp/r28b: after deferring the parent, the in_progress child vanishes from Active Work and shows in Backlog Work; a blocked child shows as backlog; the issue show rollup counts ''active 0 ... backlog 3''; and ''issue transition <child>'' blocks request_review (in_progress to review, an active-category status) with the parked message, though close is still allowed. The rule the human chose is ''not ready, and start refuses''; hiding in-flight work and blocking review goes further than that. Fix: classify only todo/blocked-category children as parked, keeping Active as Active, and apply the transition blocker only when the source status is not active-category (start/start-from-blocked). Or refuse ''defer'' on a parent with an active child. Add a test for either choice.'
target: atelier-28mz
identity: Independent reviewer (Claude subagent)
commit: 82f02bd8c6622240e624d952257e5f89a32c42e9
created_at: 2026-10-16T01:27:48.620384995Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/work_classification.rs:169-175 and crates/atelier-cli/src/commands/workflow.rs:245-253 apply parking to children in any status, not just children that have not started. The work workflow allows 'defer' (todo/blocked to backlog) on a parent whose child is in_progress (parent stays todo when a child starts). Repro in /tmp/r28b: after deferring the parent, the in_progress child vanishes from Active Work and shows in Backlog Work; a blocked child shows as backlog; the issue show rollup counts 'active 0 ... backlog 3'; and 'issue transition <child>' blocks request_review (in_progress to review, an active-category status) with the parked message, though close is still allowed. The rule the human chose is 'not ready, and start refuses'; hiding in-flight work and blocking review goes further than that. Fix: classify only todo/blocked-category children as parked, keeping Active as Active, and apply the transition blocker only when the source status is not active-category (start/start-from-blocked). Or refuse 'defer' on a parent with an active child. Add a test for either choice.
