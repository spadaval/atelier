---
schema: atelier.review
schema_version: 1
id: atelier-krdy
title: 'Low: crates/atelier-app/src/mission_review.rs:986-1050 The single timeline mixes clocks: delivered items and validation runs use record times (closed_at, captured_at, often on a branch) while start, added and close use first-parent merge times. On atelier-r6bt it reads ''14:44 delivered atelier-ho7o'' (with child atelier-stsa) and then ''15:04 added atelier-stsa ✓ done'', i.e. added after it was delivered. Consider dating Added events no later than the item''s or its parent''s delivery, or noting merge time, or hiding Added for children added on their parent''s branch.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:29.185093816Z
event:
  event: finding
  recommendation: defer
---

Low: crates/atelier-app/src/mission_review.rs:986-1050 The single timeline mixes clocks: delivered items and validation runs use record times (closed_at, captured_at, often on a branch) while start, added and close use first-parent merge times. On atelier-r6bt it reads '14:44 delivered atelier-ho7o' (with child atelier-stsa) and then '15:04 added atelier-stsa ✓ done', i.e. added after it was delivered. Consider dating Added events no later than the item's or its parent's delivery, or noting merge time, or hiding Added for children added on their parent's branch.
