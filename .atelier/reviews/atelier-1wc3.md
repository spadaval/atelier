---
schema: atelier.review
schema_version: 2
id: atelier-1wc3
title: finding on atelier-fcb7 @3e49443c
target: atelier-fcb7
identity: gpt-6-astra-plan-r1
commit: 3e49443cb4f2c0dacd6fda89db8e5b2832ae1ef2
created_at: 2026-11-10T01:00:02.198287871Z
event:
  event: finding
  recommendation: fix_now
---

medium docs/strategies/redesign-architecture.md:235-278: the swap promises unchanged site fixtures and generated types while replacing issue/review/evidence schemas, removing the old parsers and retaining the site's API routes. Current CI runs web typecheck, Vitest and Playwright for Rust changes; pinning only atelier check cannot keep old-schema site fixtures green. Specify the site view-model adapter and fixture conversion boundary before deleting the old reader, and require the existing web CI jobs to pass at the swap.
