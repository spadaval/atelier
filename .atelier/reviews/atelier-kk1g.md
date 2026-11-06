---
schema: atelier.review
schema_version: 1
id: atelier-kk1g
title: finding on atelier-rkxm @09c94fb3
target: atelier-rkxm
identity: lighter-workflow reviewer for atelier-rkxm (claude-opus-5-5)
commit: 09c94fb3f6a888ada6cde0fa1f7e9e75d971e382
created_at: 2026-11-06T23:54:55.603203356Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/atelier-rkxm.md (status reader table, policy column) leaves out transitions that refer to three statuses, though Done when asks for each transition that refers to it: todo is in every workflow's `supersede` from (objective, work, decision); in_progress is `revise` to (objective, work); blocked is work `defer` from. atelier-bqbz designs the status cut from this column, so a reader would miss edits that cutting blocked (defer.from) or todo would need. Fix: add those transitions to the three rows.
