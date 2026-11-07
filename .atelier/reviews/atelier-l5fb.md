---
schema: atelier.review
schema_version: 2
id: atelier-l5fb
title: finding on atelier-3jdm @e833abea
target: atelier-3jdm
identity: independent reviewer subagent (claude-opus-5-5), atelier-3jdm
commit: e833abeab005d32e67e80423c216405a08b080e6
created_at: 2026-11-07T05:51:18.464947553Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 1d4ab895bbf1a33c8b935a4af9d6a8ca810a00fc
    recorded_at: 2026-11-07T05:52:30.052896849Z
    follow_up: atelier-jnrh
    reason: atelier-jnrh owns how Done when edits are shown; its Done when line now covers an edit merged in from the base (master df2d7c0b).
---

low: crates/atelier-app/src/record_history.rs:493 (done_when_edits) with review_gate.rs work_edges: a Done when changed on the base after the item was started, merged into an approved branch, is now neither stale nor shown. Reproduced with this branch's binary: start, implement, tick, close, approve; on main add '- [ ] Also passes on macOS.' to the owner's Done when; git merge main on the branch, resolving the record conflict to keep both lines; merge --check passes 'review.current via <approval>' and the only Note names the branch's own tick commit, not the base edit. The installed binary refuses the same history ('base merge ... changed in-scope record ... beyond status'). r9hb counts only edits that descend from the start commit and skips merges, so the removed in-scope rule of allowed_base_merge was the only thing that surfaced such an edit; an added, unticked line can now merge on an approval that never saw it. The strategy says a Done when changed after start is shown 'including after approval', but atelier-jnrh's matching line says 'as atelier-r9hb does today', which keeps the gap. Suggest deferring to atelier-jnrh and widening that line so a Done when edit merged in from the base after the start is reported too (for example, compare the Done when at the start commit, or at the approval pin, with HEAD's).
