---
schema: atelier.review
schema_version: 1
id: atelier-ddlt
title: finding on atelier-bywq @59ae63e0
target: atelier-bywq
identity: independent planning reviewer subagent (claude-opus-5-5)
commit: 59ae63e0fc0faa110587261b677874c04e938ec0
created_at: 2026-10-30T06:47:20.360403891Z
event:
  event: finding
  recommendation: defer
---

R5: Done when requires 'a transition' on the base to still apply, exit 0 and warn, but done transitions on the base are atelier-9pa6's, whose open Unknown is refuse versus warn; a worker picking close would write a test 9pa6 may have to invert. Fix when planning into todo: name a non-done transition (e.g. block) for this test and leave done transitions to 9pa6. Reproduced the gap itself on master's debug build in a scratch repo: on main, issue update --priority high and issue note on an item started on task/<id> both exited 0 with no warning.
