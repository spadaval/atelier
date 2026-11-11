---
schema: atelier.review
schema_version: 2
id: atelier-fbcg
title: finding on atelier-wd1i @d41d3bcd
target: atelier-wd1i
identity: GPT-6 Astra independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:50:12.324839558Z
event:
  event: finding
  recommendation: fix_now
---

high: .atelier/issues/atelier-wd1i.md:48-50 G checks master CI only AFTER the runbook has closed and pushed wd1i, yet on any failure demands rollback using trials that never reach that state. Reverting the conversion after close modifies an item created by conversion causes a modify/delete conflict, and published rollback cannot restore original base SHA. Make F's reviewed rollback cover the final close and push states and define restored content versus revert history; or move final acceptance before close with a reviewed failure route. G must name and exercise the exact rollback it will run on post-runbook CI failure.
