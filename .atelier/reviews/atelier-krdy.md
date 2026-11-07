---
schema: atelier.review
schema_version: 2
id: atelier-krdy
title: 'Low: crates/atelier-app/src/mission_review.rs:986-1050 The single timeline mixes clocks: delivered items and validation runs use record times (closed_at, captured_at, often on a branch) while start, added and close use first-parent merge times. On atelier-r6bt it reads ''14:44 delivered atelier-ho7o'' (with child atelier-stsa) and then ''15:04 added atelier-stsa ✓ done'', i.e. added after it was delivered. Consider dating Added events no later than the item''s or its parent''s delivery, or noting merge time, or hiding Added for children added on their parent''s branch.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:29.185093816Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Independent reviewer (Claude subagent)
    commit: ac2b7aa0845ff29fc02edc61d7d3a6649d5458d0
    recorded_at: 2026-10-18T07:09:41.519080869Z
    follow_up: atelier-b3qt
    reason: 'Agreed: the mixed clocks come from the pre-existing first-parent timeline semantics, the facts are all present, and atelier-b3qt carries a testable done-when for ordering additions no later than delivery.'
    migrated_from: atelier-h7yx
---

Low: crates/atelier-app/src/mission_review.rs:986-1050 The single timeline mixes clocks: delivered items and validation runs use record times (closed_at, captured_at, often on a branch) while start, added and close use first-parent merge times. On atelier-r6bt it reads '14:44 delivered atelier-ho7o' (with child atelier-stsa) and then '15:04 added atelier-stsa ✓ done', i.e. added after it was delivered. Consider dating Added events no later than the item's or its parent's delivery, or noting merge time, or hiding Added for children added on their parent's branch.
