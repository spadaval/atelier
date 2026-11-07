---
schema: atelier.review
schema_version: 2
id: atelier-q71m
title: 'crates/atelier-records/src/lib.rs:2213 (with crates/atelier-app/src/record_index.rs:88) — the new priority error embeds the record path, and the record-index wrapper already prefixes ''Invalid record file <path>:'', so check and every loading command print the path twice: ''Invalid record file .atelier/issues/x.md: priority ''low'' in .atelier/issues/x.md: records store ...''. Readable but redundant; the Done-when mandated the in-message path, so the duplication belongs with atelier-qrwr''s general error forms rather than this item.'
target: atelier-jpok
identity: Independent reviewer (Claude subagent)
commit: 45f00dc34528f43705a5daac1666562329ab1177
created_at: 2026-10-17T07:16:50.988287801Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 45f00dc34528f43705a5daac1666562329ab1177
    recorded_at: 2026-10-17T07:19:09.093590606Z
    follow_up: atelier-x68o
    reason: The doubled path comes from the general record-load wrapper, not the priority message; filed atelier-x68o.
    migrated_from: atelier-x6pz
---

crates/atelier-records/src/lib.rs:2213 (with crates/atelier-app/src/record_index.rs:88) — the new priority error embeds the record path, and the record-index wrapper already prefixes 'Invalid record file <path>:', so check and every loading command print the path twice: 'Invalid record file .atelier/issues/x.md: priority 'low' in .atelier/issues/x.md: records store ...'. Readable but redundant; the Done-when mandated the in-message path, so the duplication belongs with atelier-qrwr's general error forms rather than this item.
