---
schema: atelier.review
schema_version: 1
id: atelier-gq88
title: 'crates/atelier-app/src/record_history.rs:172 with crates/atelier-app/src/mission_review.rs:116: scope added after the start is checked only for items holding an advances link (linked); a child that joined an advanced epic after the mission started is never flagged, since the view''s scope (mission_issue_ids) includes children. On this repository''s done mission atelier-kgcr, 7 of its 11 scoped children joined atelier-1ioo after the start (atelier-8ud1 at 4fdb0b0c 2026-10-01 19:06 vs start 0ce67c19 17:19; cmi9, xe43, 4xyr, ksqc, 2oor and g0zm at 3936f74c on 2026-10-15), yet the debrief reports no scope added. That misses a Done-when deviation on the real-mission run. Fix: for a child, date the joining from the first version of the child or of its parent that records the parent link (parent_id on the child or the children list on the parent), the same way as advances links on either record, and add a fixture child added after the start.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:15.208500081Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/record_history.rs:172 with crates/atelier-app/src/mission_review.rs:116: scope added after the start is checked only for items holding an advances link (linked); a child that joined an advanced epic after the mission started is never flagged, since the view's scope (mission_issue_ids) includes children. On this repository's done mission atelier-kgcr, 7 of its 11 scoped children joined atelier-1ioo after the start (atelier-8ud1 at 4fdb0b0c 2026-10-01 19:06 vs start 0ce67c19 17:19; cmi9, xe43, 4xyr, ksqc, 2oor and g0zm at 3936f74c on 2026-10-15), yet the debrief reports no scope added. That misses a Done-when deviation on the real-mission run. Fix: for a child, date the joining from the first version of the child or of its parent that records the parent link (parent_id on the child or the children list on the parent), the same way as advances links on either record, and add a fixture child added after the start.
