---
schema: atelier.review
schema_version: 2
id: atelier-69ua
title: finding on atelier-e05n @7d7551a8
target: atelier-e05n
identity: independent-planning-review-gpt-6-astra
commit: 7d7551a890e9dcf0043882aac7e9ab1098f86309
created_at: 2026-11-11T06:24:45.816113080Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-e05n.md:33 does not specify that a mission response aggregates identities from its items and their reviews and decisions, as well as its own lead and plan reviewers. Taken literally, a mission with no lead but a running child implementer returns no threads, while the item page shows that agent. That misses the strategy Outcome (agents working on the mission) and the Goal (mission and its items). FIX NOW: make the mission aggregation explicit and add a fixture with no mission lead, a child implementer and a child reviewer; assert their roles and whole identities on both the mission and item responses. Keep holds tied to the actual owning record.
