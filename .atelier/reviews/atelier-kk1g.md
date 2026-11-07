---
schema: atelier.review
schema_version: 2
id: atelier-kk1g
title: finding on atelier-rkxm @09c94fb3
target: atelier-rkxm
identity: lighter-workflow reviewer for atelier-rkxm (claude-opus-5-5)
commit: 09c94fb3f6a888ada6cde0fa1f7e9e75d971e382
created_at: 2026-11-06T23:54:55.603203356Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: atelier-rkxm implementer (claude-opus-5-5)
    commit: 8d3d4181442b4643d00bdf863df5b30b09df8d98
    recorded_at: 2026-11-06T23:56:25.344844184Z
    resolution_commit: 8d3d4181442b4643d00bdf863df5b30b09df8d98
    reason: 'Verified against the 05f01d83 policy: todo is in supersede from in all three workflows, in_progress is revise''s target, blocked is in work defer from. Added them to the status rows; a scripted scan of every transition found no other omission, and the draft row now also lists the three epic-migration call sites that hard-code draft.'
    migrated_from: atelier-noei
---

low: .atelier/issues/atelier-rkxm.md (status reader table, policy column) leaves out transitions that refer to three statuses, though Done when asks for each transition that refers to it: todo is in every workflow's `supersede` from (objective, work, decision); in_progress is `revise` to (objective, work); blocked is work `defer` from. atelier-bqbz designs the status cut from this column, so a reader would miss edits that cutting blocked (defer.from) or todo would need. Fix: add those transitions to the three rows.
