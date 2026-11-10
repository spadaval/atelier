---
schema: atelier.review
schema_version: 2
id: atelier-ci12
title: finding on atelier-qr5s @378eb60a
target: atelier-qr5s
identity: grok-4.6-plan-r2
commit: 378eb60a7695bdea855a47332bda78b1218b3445
created_at: 2026-11-10T01:23:23.719345887Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:235-238 and :291-321: land on the base accepts only an Atelier merge or paths under .atelier/missions/ and .atelier/items/, and the converter is specified to convert missions, items and linked strategies only. Step 5 commits conversion of the other four on the base and lands it, which must write .atelier/config.toml (today's merge and check have no other config) and delete .atelier/issues, reviews and workflow.yaml. land refuses that commit, and without config.toml the new check cannot run. Smallest fix: the converter writes config.toml from workflow.yaml with ci_workflow optional when the repo has no Actions, and each live conversion is an item merged with --trivial, or land grows an explicit conversion exception.
