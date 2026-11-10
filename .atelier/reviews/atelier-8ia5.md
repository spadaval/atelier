---
schema: atelier.review
schema_version: 2
id: atelier-8ia5
title: finding on atelier-fcb7 @6f837e9f
target: atelier-fcb7
identity: grok-4.6-plan-r1
commit: 6f837e9f52a011db821640751ec20b664759a850
created_at: 2026-11-10T01:05:48.578897465Z
event:
  event: finding
  recommendation: fix_now
---

high redesign-architecture.md:154-163 vs :197-199 and redesign-product.md:114: the act table allows start only from ready, Threads says start on a building item rebinds, and a mission leaves ready on first start. start writes the work branch, the mission record lives on the base, and nothing is copied between branches, so first start cannot store running; after a dead implementer in changes_requested or approved, no command rebinds and merge cannot run. Smallest fix: only lead on the base sets a mission running, and start requires that; start rebinds from every implementer-held status (ready, building, changes_requested, approved).
