---
schema: atelier.review
schema_version: 1
id: atelier-xiee
title: 'crates/atelier-cli/src/commands/work.rs:117-125: atelier -q work all prints needs-human IDs after backlog, while the page puts the Needs a Human panel right after Ready Work. Quiet output is IDs only and does not promise any group order, so this has no consequence. Similarly, prime (session.rs:185) counts from work_buckets without apply_ready_execution_gates, but classify_issue already uses dependency_closure under a policy, so the count matched work human in every scratch case I tried (blocked, transitively blocked, active, done). Prime timing on the worktree was 0.15s, unchanged from the master binary.'
target: atelier-q8ok
identity: Independent reviewer (Claude subagent)
commit: bd03289966af07bb8f13ec3c855590bf6e8504af
created_at: 2026-10-16T00:09:55.994368715Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-cli/src/commands/work.rs:117-125: atelier -q work all prints needs-human IDs after backlog, while the page puts the Needs a Human panel right after Ready Work. Quiet output is IDs only and does not promise any group order, so this has no consequence. Similarly, prime (session.rs:185) counts from work_buckets without apply_ready_execution_gates, but classify_issue already uses dependency_closure under a policy, so the count matched work human in every scratch case I tried (blocked, transitively blocked, active, done). Prime timing on the worktree was 0.15s, unchanged from the master binary.
