---
schema: atelier.review
schema_version: 1
id: atelier-d7fj
title: 'crates/atelier-app/src/issue_read.rs:57 — the 5927bc00 rename turns the rollup Health from ''terminal'' into ''done'', but health() returns it whenever any scoped work is done and none is blocked, active, ready or waiting on the human, even with backlog work still open. Live: issue show atelier-1w8c --full prints ''Health : done'' over ''Scope : 7 issues: 5 done, 2 backlog'' for a todo mission. The old word was opaque; the new one states something false. It also departs from the widened Counts rule, which says closed work is ''closed''. Fix: check backlog before done, so done+backlog reads ''steady'', and reserve ''done'' (or ''closed'') for a scope with nothing open. Add a test for the done+backlog case.'
target: atelier-krf4
identity: Independent reviewer (Claude subagent)
commit: 6b4ae1c4e2f97d5c341a19336a6496db5a49c521
created_at: 2026-10-17T07:50:22.171643216Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/issue_read.rs:57 — the 5927bc00 rename turns the rollup Health from 'terminal' into 'done', but health() returns it whenever any scoped work is done and none is blocked, active, ready or waiting on the human, even with backlog work still open. Live: issue show atelier-1w8c --full prints 'Health : done' over 'Scope : 7 issues: 5 done, 2 backlog' for a todo mission. The old word was opaque; the new one states something false. It also departs from the widened Counts rule, which says closed work is 'closed'. Fix: check backlog before done, so done+backlog reads 'steady', and reserve 'done' (or 'closed') for a scope with nothing open. Add a test for the done+backlog case.
