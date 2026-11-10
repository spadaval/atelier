---
schema: atelier.review
schema_version: 2
id: atelier-i5sn
title: finding on atelier-fcb7 @6f837e9f
target: atelier-fcb7
identity: grok-4.6-plan-r1
commit: 6f837e9f52a011db821640751ec20b664759a850
created_at: 2026-11-10T01:06:05.848524983Z
event:
  event: finding
  recommendation: fix_now
---

high redesign-architecture.md:235-238 and :278: the site keeps its routes, generated types stay as they are, and the board reads status directly. That is a compatibility facade over the old Issue, Evidence and Workflow JSON types, which the rule for this work forbids, and it contradicts a stored-status board. Smallest fix: drop generated types stay; step E updates the JSON and UI to the three record kinds, or cut the result-comparison from mission 2 and keep only a smoke that serve loads a new-layout fixture.
