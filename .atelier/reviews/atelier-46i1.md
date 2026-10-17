---
schema: atelier.review
schema_version: 1
id: atelier-46i1
title: 'crates/atelier-app/src/record_ref.rs:112 — with_article picks the article from the first letter, so a custom type whose sound differs from its letter (e.g. user_story, unit, uat) would print ''is an user_story, not a mission''. Low impact: workflow issue types are lowercase ASCII identifiers (atelier-workflow is_identifier), no built-in type or record kind is affected, and docs/product/human-cli-output.md:271 now states the letter rule explicitly, so behaviour matches the documented contract; an exception list would be speculative.'
target: atelier-jx7g
identity: Independent reviewer (Claude subagent)
commit: 1e29e392118f836aaa8903a906cb806568288517
created_at: 2026-10-17T08:55:24.304303531Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/record_ref.rs:112 — with_article picks the article from the first letter, so a custom type whose sound differs from its letter (e.g. user_story, unit, uat) would print 'is an user_story, not a mission'. Low impact: workflow issue types are lowercase ASCII identifiers (atelier-workflow is_identifier), no built-in type or record kind is affected, and docs/product/human-cli-output.md:271 now states the letter rule explicitly, so behaviour matches the documented contract; an exception list would be speculative.
