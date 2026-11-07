---
schema: atelier.review
schema_version: 2
id: atelier-h2fn
title: 'Low: crates/atelier-app/src/mission_review.rs:1011-1014 (events) and :507-528 A follow-up filed by the review of an item that is not delivered (open, in progress or dropped) appears in no timeline entry: Filed events are emitted only for from=None or from=mission, and WorkEntry.follow_ups exists only for done items. It still shows in Found Along The Way, but the doc says the timeline carries work filed. Fix: emit Event::Filed when the source has no work entry, and test it.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:26.792115354Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    recorded_at: 2026-10-18T07:07:55.608262331Z
    resolution_commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    reason: Fixed in 8151a566
    migrated_from: atelier-krc9
---

Low: crates/atelier-app/src/mission_review.rs:1011-1014 (events) and :507-528 A follow-up filed by the review of an item that is not delivered (open, in progress or dropped) appears in no timeline entry: Filed events are emitted only for from=None or from=mission, and WorkEntry.follow_ups exists only for done items. It still shows in Found Along The Way, but the doc says the timeline carries work filed. Fix: emit Event::Filed when the source has no work entry, and test it.
