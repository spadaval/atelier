---
schema: atelier.review
schema_version: 1
id: atelier-9ykj
title: 'Low: crates/atelier-cli/src/commands/mission_review.rs:100-116 and :82-85 The quiet deviations count now sums left and found only, so asked items added after the start (atelier-9y66 on r6bt) stop counting as a scope-added deviation, which the old count (over all open work) included. Also a needs-human or decision child of a done parent is now counted twice in items= (once as a work child, once via the new + asked term). Fix: count added_after_start over asks too, and count asked items not already under a work entry.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:29.779551120Z
event:
  event: finding
  recommendation: defer
---

Low: crates/atelier-cli/src/commands/mission_review.rs:100-116 and :82-85 The quiet deviations count now sums left and found only, so asked items added after the start (atelier-9y66 on r6bt) stop counting as a scope-added deviation, which the old count (over all open work) included. Also a needs-human or decision child of a done parent is now counted twice in items= (once as a work child, once via the new + asked term). Fix: count added_after_start over asks too, and count asked items not already under a work entry.
