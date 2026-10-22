---
schema: atelier.review
schema_version: 1
id: atelier-p5qp
title: 'low: docs/product/reviews-and-merge.md:261-264 says land judges approval independence ''as review.current judges it'', and the approval_accepted and author_identities_to doc comments (crates/atelier-app/src/review_gate.rs) say only commits merges bring in from the base are left out. original_independent passes HEAD as the base (review_gate.rs:1310-1318), so land and claims.reviewed leave out the side-only commits of every merge into the branch, while review.current counts a side branch that is not on the base. Scratch repro under both strategies: a side branch authored by Reviewer, merged into the work branch, close, approve as Reviewer: merge --check fails only review.current, and land after a forced git merge passes. This is the safe direction and matches the 1ztv decision note (merge can be stricter, never more lenient, because claims.reviewed runs land''s own check), but the product doc, which is meant to state the rule once, claims the two are identical. Fix: say in reviews-and-merge.md that land leaves out the authors of any merged-in side commits, which review.current counts unless they are on the base, and rename or document the base argument at the original_independent call site. Fold into the next change to this gate (atelier-r28p).'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: a8819c8a0cfbbc44f43ff284cd4cac857ef70549
created_at: 2026-10-22T22:03:37.745976726Z
event:
  event: finding
  recommendation: defer
---

low: docs/product/reviews-and-merge.md:261-264 says land judges approval independence 'as review.current judges it', and the approval_accepted and author_identities_to doc comments (crates/atelier-app/src/review_gate.rs) say only commits merges bring in from the base are left out. original_independent passes HEAD as the base (review_gate.rs:1310-1318), so land and claims.reviewed leave out the side-only commits of every merge into the branch, while review.current counts a side branch that is not on the base. Scratch repro under both strategies: a side branch authored by Reviewer, merged into the work branch, close, approve as Reviewer: merge --check fails only review.current, and land after a forced git merge passes. This is the safe direction and matches the 1ztv decision note (merge can be stricter, never more lenient, because claims.reviewed runs land's own check), but the product doc, which is meant to state the rule once, claims the two are identical. Fix: say in reviews-and-merge.md that land leaves out the authors of any merged-in side commits, which review.current counts unless they are on the base, and rename or document the base argument at the original_independent call site. Fold into the next change to this gate (atelier-r28p).
