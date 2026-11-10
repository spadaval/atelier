---
schema: atelier.review
schema_version: 2
id: atelier-y3ex
title: finding on atelier-1sbw @141b4247
target: atelier-1sbw
identity: gpt-6.1-sol-r2
commit: 141b42473c322df6e21df221a7d43d9a8500a1c8
created_at: 2026-11-10T23:17:03.081598365Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:516 R1 Splitting a strategy separates reference-link uses from their definitions. In the invented fixture, append `See [the mock][m].` to the Why paragraph, leaving the existing `[m]: mocks/a.md#a-mock` definition under Shape. The original strategy resolves that reference. Conversion writes the use into mission.md but the definition only into design.md, so mission.md renders an undefined reference. Conversion and --verify both exit 0. Preserve the reference definition in every output document that uses it, or expand its uses into inline links before splitting, with destinations resolved from the original file. Add a regression that checks reference uses have definitions in the same output document; resolving only definition destinations, as the existing test does, misses this contract failure.
