---
schema: atelier.review
schema_version: 2
id: atelier-f35t
title: finding on atelier-fcb7 @378eb60a
target: atelier-fcb7
identity: gpt-6-astra-plan-r2
commit: 378eb60a7695bdea855a47332bda78b1218b3445
created_at: 2026-11-10T01:17:32.896781947Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:245,309-315 (atelier-vw77 still open): the converted cutover branch must merge using schema-v3 config read from the unconverted base. The real base config is schema_version=1 and holds no merge checks or forge; policy lives in workflow.yaml. No binary reads both layouts, so the first new merge cannot obtain its trusted policy. Specify a narrowly reviewed bootstrap policy/publication step and rehearse the actual old-base/new-head merge, not just an already-converted scratch repository.
