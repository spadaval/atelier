---
schema: atelier.review
schema_version: 1
id: atelier-h2fn
title: 'Low: crates/atelier-app/src/mission_review.rs:1011-1014 (events) and :507-528 A follow-up filed by the review of an item that is not delivered (open, in progress or dropped) appears in no timeline entry: Filed events are emitted only for from=None or from=mission, and WorkEntry.follow_ups exists only for done items. It still shows in Found Along The Way, but the doc says the timeline carries work filed. Fix: emit Event::Filed when the source has no work entry, and test it.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:26.792115354Z
event:
  event: finding
  recommendation: defer
---

Low: crates/atelier-app/src/mission_review.rs:1011-1014 (events) and :507-528 A follow-up filed by the review of an item that is not delivered (open, in progress or dropped) appears in no timeline entry: Filed events are emitted only for from=None or from=mission, and WorkEntry.follow_ups exists only for done items. It still shows in Found Along The Way, but the doc says the timeline carries work filed. Fix: emit Event::Filed when the source has no work entry, and test it.
