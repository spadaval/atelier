---
schema: atelier.review
schema_version: 2
id: atelier-qy7l
title: finding on atelier-qr5s @db78b9cf
target: atelier-qr5s
identity: gpt-6-astra-plan-r2
commit: db78b9cf43aafd7a5b2ead857d695fc10007be90
created_at: 2026-11-10T01:17:35.751995079Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-wd1i and docs/strategies/redesign-architecture.md:323-326: rollback before step 4 says abandon the branch because nothing outside it changed, but step 3 already merges and pushes master. A failure after that push leaves published new records with the old installed binary, which abandoning a branch cannot undo. Move the rollback boundary to the merge/publication and rehearse failure immediately after it, including restoration of hooks and per-repository partial conversion progress.
