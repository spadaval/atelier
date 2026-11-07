---
schema: atelier.review
schema_version: 2
id: atelier-m4gv
title: 'MEDIUM crates/atelier-cli/src/commands/merge.rs run (gates via review_gate::evaluate over the owner''s scope only): under merge_commit the merged records are judged only for the owner''s scope, so a record change on the branch that breaks a claim the base completed after the fork point reaches the base unchecked. Probe: base has mission M advanced by issue W (done); branch reopens W and is approved; base closes M; atelier merge passes every gate and lands the merge commit; atelier land and atelier doctor on main then refuse ''M work.terminal: open work: W''. Under fast_forward_only merge''s gates also miss it, but atelier land on the caught-up branch sees the combination before handoff; under merge_commit no checkout holds it until it is already on the base, so the procedural check is lost. Contained to the local base until push (land refuses), but recovery needs base edits. Fix: before updating the base, run land''s affected-claims evaluation on the merge commit with base_head as the trusted point (base_head passed by induction, so only claims the merge''s changes can affect need checking), and add this probe as a test.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: fe1ea4807f2c7d27d349a570d5d437928de37b09
created_at: 2026-10-17T06:35:36.401005484Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5)
    commit: 17718c33af9cc8b05ca8c453c09f30d66e7a6a02
    recorded_at: 2026-10-17T07:04:07.446939810Z
    resolution_commit: a42f58fa095c12daee774fc6edd2289f0e28056c
    reason: Merge now runs land's affected-claims check on the merge commit's records with the base head as the trusted point and refuses naming the failing claim; tested with the reviewer's scenario.
    migrated_from: atelier-hgqs
---

MEDIUM crates/atelier-cli/src/commands/merge.rs run (gates via review_gate::evaluate over the owner's scope only): under merge_commit the merged records are judged only for the owner's scope, so a record change on the branch that breaks a claim the base completed after the fork point reaches the base unchecked. Probe: base has mission M advanced by issue W (done); branch reopens W and is approved; base closes M; atelier merge passes every gate and lands the merge commit; atelier land and atelier doctor on main then refuse 'M work.terminal: open work: W'. Under fast_forward_only merge's gates also miss it, but atelier land on the caught-up branch sees the combination before handoff; under merge_commit no checkout holds it until it is already on the base, so the procedural check is lost. Contained to the local base until push (land refuses), but recovery needs base edits. Fix: before updating the base, run land's affected-claims evaluation on the merge commit with base_head as the trusted point (base_head passed by induction, so only claims the merge's changes can affect need checking), and add this probe as a test.
