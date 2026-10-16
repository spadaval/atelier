---
schema: atelier.review
schema_version: 1
id: atelier-4d9x
title: 'crates/atelier-cli/src/commands/workflow.rs:249-252: the blocker message always says ''plan the parent first'', but parked_parent matches any todo-category status without a start, including custom draft statuses whose exit is ''ready'' and decisions in ''todo'' that have no plan. Users get told to run a transition that does not exist. Fix: name the parent''s available transitions out of its status, or word it generically (''move the parent out of <status> first''). Low impact under the default policy once the decision case is fixed.'
target: atelier-28mz
identity: Independent reviewer (Claude subagent)
commit: 82f02bd8c6622240e624d952257e5f89a32c42e9
created_at: 2026-10-16T01:27:48.726061416Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/workflow.rs:249-252: the blocker message always says 'plan the parent first', but parked_parent matches any todo-category status without a start, including custom draft statuses whose exit is 'ready' and decisions in 'todo' that have no plan. Users get told to run a transition that does not exist. Fix: name the parent's available transitions out of its status, or word it generically ('move the parent out of <status> first'). Low impact under the default policy once the decision case is fixed.
