---
schema: atelier.review
schema_version: 1
id: atelier-dc4o
title: 'crates/atelier-records/src/lib.rs:1371-1372. A mission body with the retired ''## Epics'' section gets the error ''it belonged to the retired epic type ... Run atelier migrate epics''. But migrate epics only rewrites epic records, so it prints ''No epic records or epic policy entries to migrate.'' and the user is left with no next step. Repro: append ''## Epics'' to a mission record, then run check and then migrate epics. Fix: give Epics on a mission its own message (move the content to Notes or delete it, and link work with advances), or have the migration also move a mission''s Epics section into Notes.'
target: atelier-o6h9
identity: Independent reviewer (Claude subagent)
commit: 4f66aab9fee9062a0f8cf6b3f586acc04a0cc218
created_at: 2026-10-16T00:37:31.035061189Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-records/src/lib.rs:1371-1372. A mission body with the retired '## Epics' section gets the error 'it belonged to the retired epic type ... Run atelier migrate epics'. But migrate epics only rewrites epic records, so it prints 'No epic records or epic policy entries to migrate.' and the user is left with no next step. Repro: append '## Epics' to a mission record, then run check and then migrate epics. Fix: give Epics on a mission its own message (move the content to Notes or delete it, and link work with advances), or have the migration also move a mission's Epics section into Notes.
