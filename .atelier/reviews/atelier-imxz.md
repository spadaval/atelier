---
schema: atelier.review
schema_version: 1
id: atelier-imxz
title: 'crates/atelier-app/src/work_classification.rs:155-156: a todo-category record with open blockers is classified Blocked before the can_enter_active check. So a backlog item with a blocked_by link shows up in ''work blocked'' (repro: create --status backlog, then link --role blocked_by to an open todo; ''work all'' lists it under Blocked Work as ''[backlog] ... | 1 blocker''). ''defer'' from blocked also leaves blocked work in the Blocked bucket. This rule already existed for draft and the requirement (keep it out of work ready) is met, but someday work still crowds the blocked view. Suggested follow-up: classify non-startable todo-category statuses as Backlog whether or not they have blockers (still reporting the blocker count), or document the behavior in docs/product/workflow-configuration.md.'
target: atelier-ha6k
identity: Independent reviewer (Claude subagent)
commit: 40e0397b21a8f445ae37e346e69e81ef0d7bf292
created_at: 2026-10-15T23:23:47.347111739Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/work_classification.rs:155-156: a todo-category record with open blockers is classified Blocked before the can_enter_active check. So a backlog item with a blocked_by link shows up in 'work blocked' (repro: create --status backlog, then link --role blocked_by to an open todo; 'work all' lists it under Blocked Work as '[backlog] ... | 1 blocker'). 'defer' from blocked also leaves blocked work in the Blocked bucket. This rule already existed for draft and the requirement (keep it out of work ready) is met, but someday work still crowds the blocked view. Suggested follow-up: classify non-startable todo-category statuses as Backlog whether or not they have blockers (still reporting the blocker count), or document the behavior in docs/product/workflow-configuration.md.
