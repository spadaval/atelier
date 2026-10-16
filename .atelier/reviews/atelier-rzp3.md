---
schema: atelier.review
schema_version: 1
id: atelier-rzp3
title: 'crates/atelier-app/src/work_classification.rs:248-260 (parked_parent) treats any todo-category parent status without an active transition as parked, which includes an open decision in ''todo'' (the decision workflow has no start). Children of an open decision are now parked, and the decision cannot be decided until they finish (''decide'' requires work.terminal), so the parent is stuck forever. Repro in /tmp/r28: decision D with spike child C; ''atelier start C'' fails with ''parent D is parked in todo ... plan the parent first'', ''work all'' puts C in Backlog, and D has no plan transition (only block/decide/supersede). On master C was startable. Fix: limit parking to parents that cannot enter active and are not decisions (or to statuses with a todo-category exit such as plan/ready), and add a test for a decision parent with a child.'
target: atelier-28mz
identity: Independent reviewer (Claude subagent)
commit: 82f02bd8c6622240e624d952257e5f89a32c42e9
created_at: 2026-10-16T01:27:48.516374543Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/work_classification.rs:248-260 (parked_parent) treats any todo-category parent status without an active transition as parked, which includes an open decision in 'todo' (the decision workflow has no start). Children of an open decision are now parked, and the decision cannot be decided until they finish ('decide' requires work.terminal), so the parent is stuck forever. Repro in /tmp/r28: decision D with spike child C; 'atelier start C' fails with 'parent D is parked in todo ... plan the parent first', 'work all' puts C in Backlog, and D has no plan transition (only block/decide/supersede). On master C was startable. Fix: limit parking to parents that cannot enter active and are not decisions (or to statuses with a todo-category exit such as plan/ready), and add a test for a decision parent with a child.
