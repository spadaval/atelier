---
schema: atelier.review
schema_version: 2
id: atelier-muf9
title: finding on atelier-1sbw @82f0b9b6
target: atelier-1sbw
identity: gpt-6.1-sol-r1
commit: 82f0b9b6acae40566f4062ec4abab31875eb1c30
created_at: 2026-11-10T23:05:28.824062948Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:364 R2 Relative Markdown links with a title or a reference definition are not rewritten when their strategy moves. In the invented fixture, replace the Shape link with [mocks](mocks/a.md "mock title") and [reference][mock] plus [mock]: mocks/a.md, commit that old tree, then convert. Both destinations remain mocks/a.md in .atelier/missions/demo-m1ss/design.md, where they do not exist; docs/strategies/mocks/a.md still exists. --verify exits 0 because it repeats the same broken conversion. The regex only recognizes a no-whitespace inline destination, and the existing link test uses the same limitation, so it misses these standard Markdown links. Rewrite titled inline links and reference-definition destinations, and extend the test to resolve these destinations and their fragments after the move. The five surveyed baseline snapshots have no resolving titled or reference links in their moved strategies, but the stated link-preservation contract has no syntax restriction.
