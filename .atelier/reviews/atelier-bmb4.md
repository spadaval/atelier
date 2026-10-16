---
schema: atelier.review
schema_version: 1
id: atelier-bmb4
title: 'crates/atelier-cli/src/commands/work.rs:233, work.rs:371, work.rs:401, issue.rs:168: the new Context bucket is counted in ObjectiveBucketTotals.context and WorkBuckets.context, but none of the count lines print it, so counts no longer add up and parents/missions silently vanish from summaries (violates the ''counts stay complete and consistent across views'' constraint). Reproduced in a scratch repo with a todo mission advanced by a feature parent with one todo child: ''work missions'' prints ''Scope: 2 issues: 0 active, 1 ready, 0 blocked, 0 done, 0 backlog'' (sums to 1); ''work mission <id>'' Work line and ''-q work mission'' print ready=1 with nothing else; ''-q work parent <parent>'' prints ''active=0 ready=1 ... backlog=0 proof_gaps=2'' (2 rows scoped, 1 counted); ''issue show <mission>'' Buckets line says 1 of 2 scoped issues. Before this change the parent was counted as ready. Fix: add the context count to all four lines (e.g. ''{} context'' / ''context={}'', shown only when nonzero like human_count if preferred), and add a test asserting the scope total equals the bucket sum for a mission with a parent and child.'
target: atelier-ymmb
identity: Independent reviewer (Claude subagent)
commit: b04dabfa30b96e84fe1b5ea9745987123c3a026f
created_at: 2026-10-16T01:03:46.656526428Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/work.rs:233, work.rs:371, work.rs:401, issue.rs:168: the new Context bucket is counted in ObjectiveBucketTotals.context and WorkBuckets.context, but none of the count lines print it, so counts no longer add up and parents/missions silently vanish from summaries (violates the 'counts stay complete and consistent across views' constraint). Reproduced in a scratch repo with a todo mission advanced by a feature parent with one todo child: 'work missions' prints 'Scope: 2 issues: 0 active, 1 ready, 0 blocked, 0 done, 0 backlog' (sums to 1); 'work mission <id>' Work line and '-q work mission' print ready=1 with nothing else; '-q work parent <parent>' prints 'active=0 ready=1 ... backlog=0 proof_gaps=2' (2 rows scoped, 1 counted); 'issue show <mission>' Buckets line says 1 of 2 scoped issues. Before this change the parent was counted as ready. Fix: add the context count to all four lines (e.g. '{} context' / 'context={}', shown only when nonzero like human_count if preferred), and add a test asserting the scope total equals the bucket sum for a mission with a parent and child.
