---
schema: atelier.review
schema_version: 1
id: atelier-jdjv
title: 'Low: crates/atelier-app/src/mission_review.rs:290-294 Debrief::entry_holding is added but never called anywhere (dead pub code; rustc does not warn because it is pub in a library). Remove it.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:08.772100483Z
event:
  event: finding
  recommendation: fix_now
---

Low: crates/atelier-app/src/mission_review.rs:290-294 Debrief::entry_holding is added but never called anywhere (dead pub code; rustc does not warn because it is pub in a library). Remove it.
