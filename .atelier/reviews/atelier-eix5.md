---
schema: atelier.review
schema_version: 1
id: atelier-eix5
title: 'crates/atelier-app/src/review_gate.rs:417: the recovery ''Reset the branch to the tip it had before ... then merge the base'' does not warn that commits made after the rebase are discarded by the reset. Scenario: user rebases a reviewed branch, commits a fix and new proof, then atelier merge refuses; following the advice with git reset --hard to the pre-rebase tip silently drops that later work. Suggest adding ''cherry-pick any commits made since'' (and the same in docs/product/reviews-and-merge.md:96). Matches the issue''s stated acceptance, so not blocking.'
target: atelier-j48s
identity: Independent reviewer (Claude subagent)
commit: 06f62abff594387257edc29e1fd681eb9afacd15
created_at: 2026-10-16T18:47:15.982446595Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/review_gate.rs:417: the recovery 'Reset the branch to the tip it had before ... then merge the base' does not warn that commits made after the rebase are discarded by the reset. Scenario: user rebases a reviewed branch, commits a fix and new proof, then atelier merge refuses; following the advice with git reset --hard to the pre-rebase tip silently drops that later work. Suggest adding 'cherry-pick any commits made since' (and the same in docs/product/reviews-and-merge.md:96). Matches the issue's stated acceptance, so not blocking.
