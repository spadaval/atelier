---
schema: atelier.review
schema_version: 2
id: atelier-9ykj
title: 'Low: crates/atelier-cli/src/commands/mission_review.rs:100-116 and :82-85 The quiet deviations count now sums left and found only, so asked items added after the start (atelier-9y66 on r6bt) stop counting as a scope-added deviation, which the old count (over all open work) included. Also a needs-human or decision child of a done parent is now counted twice in items= (once as a work child, once via the new + asked term). Fix: count added_after_start over asks too, and count asked items not already under a work entry.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:29.779551120Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    recorded_at: 2026-10-18T07:07:56.568147294Z
    resolution_commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    reason: Fixed in 8151a566
    migrated_from: atelier-pi1v
---

Low: crates/atelier-cli/src/commands/mission_review.rs:100-116 and :82-85 The quiet deviations count now sums left and found only, so asked items added after the start (atelier-9y66 on r6bt) stop counting as a scope-added deviation, which the old count (over all open work) included. Also a needs-human or decision child of a done parent is now counted twice in items= (once as a work child, once via the new + asked term). Fix: count added_after_start over asks too, and count asked items not already under a work entry.
