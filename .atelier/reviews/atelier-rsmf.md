---
schema: atelier.review
schema_version: 1
id: atelier-rsmf
title: 'crates/atelier-app/src/issue_read.rs:32-43 (health), 85-92 (recently_updated), 125-131 (sort loop) and crates/atelier-cli/src/commands/issue.rs:156-195 (Objective Rollup Buckets line and work panels): fix 711b0fb8 added WorkBuckets.human, but the issue show objective rollup never reads it, so needs-human work now disappears from issue show <mission|epic>. Reproduced in a /tmp scratch repo: a mission whose only advances link is a needs-human todo item shows ''Scope: 1 scoped issue'', ''Buckets: active 0, ready 0, blocked 0, done 0, backlog 0'', ''Health: steady'', and no panel lists the item. An epic whose only child needs the human shows the same rollup. Before the fix the item was counted and listed as ready, so this is a regression: someone reading the mission record sees nothing pending although human work is waiting. work mission, work epic, work missions and status handle it correctly. Fix: add a human count to the Buckets line, a ''Needs a Human'' panel in the issue.rs panel list, human in the recently_updated chain and the sort loop, and a health value for human-only scope (e.g. ''needs human'' ahead of ''terminal''). Add a test assertion on issue show <mission>.'
target: atelier-q8ok
identity: Independent reviewer (Claude subagent)
commit: b1252e22150c0aca6abeb7a814b333511a51a99d
created_at: 2026-10-16T00:14:07.433598108Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/issue_read.rs:32-43 (health), 85-92 (recently_updated), 125-131 (sort loop) and crates/atelier-cli/src/commands/issue.rs:156-195 (Objective Rollup Buckets line and work panels): fix 711b0fb8 added WorkBuckets.human, but the issue show objective rollup never reads it, so needs-human work now disappears from issue show <mission|epic>. Reproduced in a /tmp scratch repo: a mission whose only advances link is a needs-human todo item shows 'Scope: 1 scoped issue', 'Buckets: active 0, ready 0, blocked 0, done 0, backlog 0', 'Health: steady', and no panel lists the item. An epic whose only child needs the human shows the same rollup. Before the fix the item was counted and listed as ready, so this is a regression: someone reading the mission record sees nothing pending although human work is waiting. work mission, work epic, work missions and status handle it correctly. Fix: add a human count to the Buckets line, a 'Needs a Human' panel in the issue.rs panel list, human in the recently_updated chain and the sort loop, and a health value for human-only scope (e.g. 'needs human' ahead of 'terminal'). Add a test assertion on issue show <mission>.
