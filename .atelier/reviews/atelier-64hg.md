---
schema: atelier.review
schema_version: 1
id: atelier-64hg
title: 'low: crates/atelier-app/src/git_freshness.rs:58-61 git_input now reuses Requests::pair, whose error context says ''cannot create git cat-file --batch input''. If socket creation fails while loading the record graph, the message names the wrong command. Cosmetic: the context could be passed in or made generic (''cannot create git input socket'').'
target: atelier-7b6e
identity: Independent reviewer (Claude subagent)
commit: 794f263aea20de359863f8112716c6ed6d840447
created_at: 2026-10-22T23:23:21.675657699Z
event:
  event: finding
  recommendation: no_action
---

low: crates/atelier-app/src/git_freshness.rs:58-61 git_input now reuses Requests::pair, whose error context says 'cannot create git cat-file --batch input'. If socket creation fails while loading the record graph, the message names the wrong command. Cosmetic: the context could be passed in or made generic ('cannot create git input socket').
