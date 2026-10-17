---
schema: atelier.review
schema_version: 1
id: atelier-m4gv
title: 'MEDIUM crates/atelier-cli/src/commands/merge.rs run (gates via review_gate::evaluate over the owner''s scope only): under merge_commit the merged records are judged only for the owner''s scope, so a record change on the branch that breaks a claim the base completed after the fork point reaches the base unchecked. Probe: base has mission M advanced by issue W (done); branch reopens W and is approved; base closes M; atelier merge passes every gate and lands the merge commit; atelier land and atelier doctor on main then refuse ''M work.terminal: open work: W''. Under fast_forward_only merge''s gates also miss it, but atelier land on the caught-up branch sees the combination before handoff; under merge_commit no checkout holds it until it is already on the base, so the procedural check is lost. Contained to the local base until push (land refuses), but recovery needs base edits. Fix: before updating the base, run land''s affected-claims evaluation on the merge commit with base_head as the trusted point (base_head passed by induction, so only claims the merge''s changes can affect need checking), and add this probe as a test.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: fe1ea4807f2c7d27d349a570d5d437928de37b09
created_at: 2026-10-17T06:35:36.401005484Z
event:
  event: finding
  recommendation: fix_now
---

MEDIUM crates/atelier-cli/src/commands/merge.rs run (gates via review_gate::evaluate over the owner's scope only): under merge_commit the merged records are judged only for the owner's scope, so a record change on the branch that breaks a claim the base completed after the fork point reaches the base unchecked. Probe: base has mission M advanced by issue W (done); branch reopens W and is approved; base closes M; atelier merge passes every gate and lands the merge commit; atelier land and atelier doctor on main then refuse 'M work.terminal: open work: W'. Under fast_forward_only merge's gates also miss it, but atelier land on the caught-up branch sees the combination before handoff; under merge_commit no checkout holds it until it is already on the base, so the procedural check is lost. Contained to the local base until push (land refuses), but recovery needs base edits. Fix: before updating the base, run land's affected-claims evaluation on the merge commit with base_head as the trusted point (base_head passed by induction, so only claims the merge's changes can affect need checking), and add this probe as a test.
