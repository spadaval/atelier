---
schema: atelier.review
schema_version: 2
id: atelier-11on
title: finding on atelier-1sbw @141b4247
target: atelier-1sbw
identity: gpt-6.1-sol-r2
commit: 141b42473c322df6e21df221a7d43d9a8500a1c8
created_at: 2026-11-10T23:17:03.433948518Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:33 R2 Angle-bracket Markdown destinations are not rewritten. In the invented fixture replace `[mocks](mocks/a.md "a mock")` with `[mocks](<mocks/a.md> "a mock")`, commit the old tree, and convert. The old target docs/strategies/mocks/a.md exists, but design.md retains `<mocks/a.md>`, which resolves to a missing file under .atelier/missions/demo-m1ss/. Conversion and --verify both exit 0. LINK passes the angle brackets into path resolution, so the valid tracked target is not recognized; the same parser also misses relative angle-bracket references in files_naming. Parse and preserve Markdown destination delimiters while resolving the inner path, and test both inline and reference-definition angle destinations plus listing of an unchanged tracked file that links this way.
