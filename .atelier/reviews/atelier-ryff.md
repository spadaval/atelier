---
schema: atelier.review
schema_version: 2
id: atelier-ryff
title: finding on atelier-n0qh @1dc540c6
target: atelier-n0qh
identity: lighter-workflow reviewer for atelier-n0qh (claude-opus-5-5)
commit: 1dc540c65e66c1445b741d4920a979592d02b7e2
created_at: 2026-11-07T02:19:12.380916800Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: atelier-n0qh implementer (claude-opus-5-5)
    commit: 44243f7d20d2a36af8deecc8ca198fbcd9979403
    recorded_at: 2026-11-07T02:29:06.293482249Z
    resolution_commit: 44243f7d20d2a36af8deecc8ca198fbcd9979403
    reason: only_validated now requires the check, reason and stand-in unchanged apart from the box and an added carried-to clause naming an issue; unit test changing_a_check_while_ticking_or_carrying_it_is_a_done_when_edit and the proof_timing tick-and-loosen step; the brief marks a check ticked before merge
---

medium: crates/atelier-app/src/after_merge.rs:72 only_validated exempts any rewrite of an open after-merge line as long as its new state is validated or carried, so ticking a box while changing the check text is hidden from atelier-r9hb's Done-when-edited notes. Reproduced with the debug binary: on the work branch, '- [ ] After merge (device): the launcher opens on the phone.' -> '- [x] After merge (device): it compiles.' produced no 'Done when edited' section in the brief and no note in merge --check; the same tick-and-rewrite on the base after merge passed land and was absent from the debrief (a plain deletion on the base did show). Impact: the one edit r9hb exists to show, loosening what done means, goes unseen when paired with a tick. Fix: exempt a line only when check text, reason and stand-in are unchanged apart from the box and an appended 'carried to <id>' clause; consider also marking in the brief a check already ticked before merge, since validation belongs on the base.
