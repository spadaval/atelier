---
schema: atelier.review
schema_version: 1
id: atelier-cakh
title: 'HIGH crates/atelier-cli/src/commands/merge.rs (index built from the branch-head snapshot; review_gate::evaluate at fork_point): under merge_commit the gates read only the branch''s records, so records the base gained after the fork point are invisible, yet the merge commit puts them on the base. Under fast_forward_only the branch had to contain the base, so they were always gated. Probe: approve the item on its branch, record a fix_now finding against it on main, run atelier merge: findings.disposed passes and the merge commit lands on main with an undisposed blocking finding (atelier land refuses afterwards, but the base already has it). The same hole covers base-side blockers or status changes in the item''s scope. Fix: evaluate the record gates against the merged commit''s records (or refuse when the base changed .atelier records in the owner''s scope since the fork point) and add a test.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: 7185c3d00f161519ba59396b48f26ff2595cd28b
created_at: 2026-10-17T06:10:47.602419933Z
event:
  event: finding
  recommendation: fix_now
---

HIGH crates/atelier-cli/src/commands/merge.rs (index built from the branch-head snapshot; review_gate::evaluate at fork_point): under merge_commit the gates read only the branch's records, so records the base gained after the fork point are invisible, yet the merge commit puts them on the base. Under fast_forward_only the branch had to contain the base, so they were always gated. Probe: approve the item on its branch, record a fix_now finding against it on main, run atelier merge: findings.disposed passes and the merge commit lands on main with an undisposed blocking finding (atelier land refuses afterwards, but the base already has it). The same hole covers base-side blockers or status changes in the item's scope. Fix: evaluate the record gates against the merged commit's records (or refuse when the base changed .atelier records in the owner's scope since the fork point) and add a test.
