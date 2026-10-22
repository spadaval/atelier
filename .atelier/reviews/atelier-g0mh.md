---
schema: atelier.review
schema_version: 1
id: atelier-g0mh
title: 'low: crates/atelier-app/src/review_gate.rs:139-143 decides ''new since the pin'' by absence from the pin''s tree alone, so it also exempts records that already existed on the base and arrived through a base merge after the approval. Scratch repro under both strategies: approve, create issue Q on main, merge main into the branch, rewrite Q''s Goal and Done when in a later branch commit, close the owner: review.current passes, atelier merge integrates the edit and land passes; the master binary (99ae3aa6) refuses review.current. Q''s done claim is still refused (closing Q fails claims.reviewed and land_base_done_claim under both strategies), and the same record-only edit could be committed on the base directly, so no gate is bypassed and merge and land agree. But it is wider than iaas''s Done when (''records that are new and not the item''s own'') and the decision (''a follow-up filed on an approved branch''): an approval now carries unreviewed edits to other increments'' records that reached the base after the pin. Fix if wanted: exempt only records absent at the pin and at the base sides of the walked base merges (or created by a walked non-base edge), or state in reviews-and-merge.md (Approval freshness, :85-91 and :110-111) that base records newer than the pin are editable too.'
target: atelier-r28p
identity: Independent reviewer (Claude subagent)
commit: dc4ce18d56aa480c2546165683341572b1d78b2a
created_at: 2026-10-22T22:35:48.414799596Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-app/src/review_gate.rs:139-143 decides 'new since the pin' by absence from the pin's tree alone, so it also exempts records that already existed on the base and arrived through a base merge after the approval. Scratch repro under both strategies: approve, create issue Q on main, merge main into the branch, rewrite Q's Goal and Done when in a later branch commit, close the owner: review.current passes, atelier merge integrates the edit and land passes; the master binary (99ae3aa6) refuses review.current. Q's done claim is still refused (closing Q fails claims.reviewed and land_base_done_claim under both strategies), and the same record-only edit could be committed on the base directly, so no gate is bypassed and merge and land agree. But it is wider than iaas's Done when ('records that are new and not the item's own') and the decision ('a follow-up filed on an approved branch'): an approval now carries unreviewed edits to other increments' records that reached the base after the pin. Fix if wanted: exempt only records absent at the pin and at the base sides of the walked base merges (or created by a walked non-base edge), or state in reviews-and-merge.md (Approval freshness, :85-91 and :110-111) that base records newer than the pin are editable too.
