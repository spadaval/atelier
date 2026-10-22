---
schema: atelier.review
schema_version: 1
id: atelier-kotb
title: 'low: docs/product/reviews-and-merge.md:85-94, crates/atelier-app/src/review_gate.rs:134-139 and the r28p g0mh decision note describe the widened exemption as records filed ''on the base after the pin'' / ''after the approval'', but the code''s criterion is only absence from the pin''s tree (review_gate.rs:144), so it also exempts a record the base filed BEFORE the approval that the branch merges in after it. Scratch repro under both strategies: start owner, file Q on main, approve on the branch, merge main in, rewrite Q''s Goal/Done when on the branch: merge --check passes review.current. Behaviour is consistent with the orchestrator''s rationale (the base may take such edits unreviewed) and opens nothing else: scope only grows by editing an in-scope record (children are stored on the parent), and a new record''s own links (blocks, advances) can only add constraints that land and merge both check. Fix: say ''not in the approval''s pinned tree, whenever the base filed it'' in the doc, the comment and the note.'
target: atelier-r28p
identity: Independent reviewer (Claude subagent)
commit: 2239abecc5796d58309d9ba151bae100f131537b
created_at: 2026-10-22T22:48:19.913030837Z
event:
  event: finding
  recommendation: defer
---

low: docs/product/reviews-and-merge.md:85-94, crates/atelier-app/src/review_gate.rs:134-139 and the r28p g0mh decision note describe the widened exemption as records filed 'on the base after the pin' / 'after the approval', but the code's criterion is only absence from the pin's tree (review_gate.rs:144), so it also exempts a record the base filed BEFORE the approval that the branch merges in after it. Scratch repro under both strategies: start owner, file Q on main, approve on the branch, merge main in, rewrite Q's Goal/Done when on the branch: merge --check passes review.current. Behaviour is consistent with the orchestrator's rationale (the base may take such edits unreviewed) and opens nothing else: scope only grows by editing an in-scope record (children are stored on the parent), and a new record's own links (blocks, advances) can only add constraints that land and merge both check. Fix: say 'not in the approval's pinned tree, whenever the base filed it' in the doc, the comment and the note.
