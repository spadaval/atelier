---
schema: atelier.review
schema_version: 2
id: atelier-9by5
title: finding on atelier-fcb7 @0a4cec39
target: atelier-fcb7
identity: gpt-6-astra-plan-r3
commit: 0a4cec3914a26642baf4cff64262a250d0fcc60f
created_at: 2026-11-10T01:32:22.517587282Z
event:
  event: finding
  recommendation: fix_now
---

medium docs/strategies/redesign-architecture.md:319-321,361: the designated rollback release 0.4.1 is cut after the site step removes serve from the old binary. Reinstalling it cannot restore the existing atelier-serve service, whose ExecStart runs atelier serve (docs/architecture/site.md:317). Pin the existing complete old release, or cut the fallback before removing serve; verify that rollback can start the old dashboard on restored records.
