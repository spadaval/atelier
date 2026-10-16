---
schema: atelier.review
schema_version: 1
id: atelier-g606
title: 'crates/atelier-core/src/lib.rs:362-366: a decision created without --body (or with whitespace) gets ''## Question\n\nGoal was not specified.''; the placeholder names the wrong section. Suggested fix: per-type placeholder (''Question was not specified.'').'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:09.279747183Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-core/src/lib.rs:362-366: a decision created without --body (or with whitespace) gets '## Question\n\nGoal was not specified.'; the placeholder names the wrong section. Suggested fix: per-type placeholder ('Question was not specified.').
