---
schema: atelier.review
schema_version: 2
id: atelier-cd3l
title: crates/atelier-cli/src/commands/work.rs:614 — 'Start the top ready item' takes the first Ready row of ScopedWork, which is in ID order, not the priority/work_order order status and work ready use, and ignores in-flight marks. On this repo 'work mission atelier-o3lm' offers 'atelier start atelier-91w1' (low priority, already done on task/atelier-91w1, not merged) over atelier-pr4k (medium); in a 37-ready fixture it offers a low item while status's top ready is a medium one. Consistent with the doc's 'first item of Ready Work', but 'top ready' disagrees between status and mission/parent views. Related to atelier-6wyv (readiness gates) but not covered by it; suggest extending 6wyv or a follow-up to order scoped Ready Work by work_order and skip in-flight items.
target: atelier-ksx6
identity: Independent reviewer (Claude subagent)
commit: b14cd5e157c4e2392e7ded21a46961d63da7e3a8
created_at: 2026-10-17T08:27:59.479425300Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 28a064cf3306e0cfa9d71990c3c86a84730f6e49
    recorded_at: 2026-10-17T08:39:27.195398965Z
    resolution_commit: c03d385b929163f84bc1d4c08d865a83f2a05d1e
    reason: 'Fixed now: scoped views list items in the shared work order and the footer picks the first ready item no other branch started.'
    migrated_from: atelier-56fh
---

crates/atelier-cli/src/commands/work.rs:614 — 'Start the top ready item' takes the first Ready row of ScopedWork, which is in ID order, not the priority/work_order order status and work ready use, and ignores in-flight marks. On this repo 'work mission atelier-o3lm' offers 'atelier start atelier-91w1' (low priority, already done on task/atelier-91w1, not merged) over atelier-pr4k (medium); in a 37-ready fixture it offers a low item while status's top ready is a medium one. Consistent with the doc's 'first item of Ready Work', but 'top ready' disagrees between status and mission/parent views. Related to atelier-6wyv (readiness gates) but not covered by it; suggest extending 6wyv or a follow-up to order scoped Ready Work by work_order and skip in-flight items.
