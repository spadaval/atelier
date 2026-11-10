---
schema: atelier.review
schema_version: 2
id: atelier-s9po
title: finding on atelier-fcb7 @6f837e9f
target: atelier-fcb7
identity: grok-4.6-plan-r1
commit: 6f837e9f52a011db821640751ec20b664759a850
created_at: 2026-11-10T01:05:58.971576147Z
event:
  event: finding
  recommendation: fix_now
---

high redesign-architecture.md:134-138 and :224-226: a configured forge always waits for CI on the head, but the work host is GitHub without Actions, so every merge there hangs or that host loses pull requests. Smallest fix: wait for CI only when ci_workflow is set; forge without it opens and pushes the PR and does not wait.
