---
schema: atelier.review
schema_version: 1
id: atelier-8w4b
title: 'crates/atelier-app/src/criteria.rs:286: when the deciding receipt''s commit is not an ancestor of the end (the mission''s close), its age is reported as Some(0), so it reads as fresh, with no age cue and no aged-proof deviation. Scenario: receipt R pinned on a feature branch from B; code commit C lands on master; the mission closes at K (contains C, not R); the feature merges later, so R is in HEAD''s history and decides. C predates the close and R never saw it, yet the debrief presents the proof as current, against the strategy''s rule never to present a receipt as passing without checking freshness. A receipt after the close is already 0 by the range. Fix: drop the ancestor guard and always count code_commits_between(R, end); the R..end range counts what end has and R lacks in both cases.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:15.809493886Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/criteria.rs:286: when the deciding receipt's commit is not an ancestor of the end (the mission's close), its age is reported as Some(0), so it reads as fresh, with no age cue and no aged-proof deviation. Scenario: receipt R pinned on a feature branch from B; code commit C lands on master; the mission closes at K (contains C, not R); the feature merges later, so R is in HEAD's history and decides. C predates the close and R never saw it, yet the debrief presents the proof as current, against the strategy's rule never to present a receipt as passing without checking freshness. A receipt after the close is already 0 by the range. Fix: drop the ancestor guard and always count code_commits_between(R, end); the R..end range counts what end has and R lacks in both cases.
