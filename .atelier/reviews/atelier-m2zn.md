---
schema: atelier.review
schema_version: 2
id: atelier-m2zn
title: finding on atelier-3sk9 @6e9aafbc
target: atelier-3sk9
identity: codex-reviewer (gpt-6.1-sol)
commit: 6e9aafbc797ae5f6d06865083322540285b70a99
created_at: 2026-11-09T00:32:41.337496275Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-3sk9.md:128-137 marks the trial Done when complete, but says the only fresh-agent trial ran d82576b9 and was not rerun after seven substantive instruction fixes. The reviewed head changes garden's product/host boundary, evidence scope, existing-item handling and digest schema; the old trial encountered a refused digest and cannot prove that the final instructions resolve it. Independent review requires proof after the last relevant change. Run a fresh-agent trial against the final skill revision, without coaching on its intended interpretation; a focused trial exercising the changed instructions with the existing raw evidence is sufficient. Record its revision and results, retain the file/merge-nothing limits, and verify its routine/recommendation split and digest.
