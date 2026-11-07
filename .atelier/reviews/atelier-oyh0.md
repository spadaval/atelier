---
schema: atelier.review
schema_version: 2
id: atelier-oyh0
title: finding on atelier-ilba @51f68a16
target: atelier-ilba
identity: independent reviewer subagent (claude-opus-5-5), atelier-ilba
commit: 51f68a1609973f8c484538870ffa7948ca29f07c
created_at: 2026-11-07T07:25:46.998610714Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 95ae0eaa50df5f0efd678df170438fc981a9e5a7
    recorded_at: 2026-11-07T07:39:05.711262951Z
    resolution_commit: b9f2214cf4063d8982c068e2ab45176f469f0b43
    reason: 'Fixed in b9f2214c: approvals and trivial exemptions stay append-only from their first commit; re-pin test added.'
---

high: crates/atelier-app/src/review_gate.rs:937 An approval's pin can be moved onto unreviewed code before the base. append_only now lets any pre-base change to a review record through, approvals included, and review.current and original_independent read an approval as it reads now, so a hand edit of an approval's commit (or target or identity) re-points it at code its reviewer never saw. Reproduced with the worktree's target/debug/atelier in a merge_commit scratch repo: approve at C, commit more code (merge --check refuses: review.current stale), sed the approval's commit to HEAD and commit; merge --check, land and check then all pass, while the installed binary refuses with review_append_only. This breaks the constraint that approvals still cover the code, and the accepted-risk claim in ADR 0028 decision 4 (line 179), PRODUCT_INTENT.md:51 and docs/product/reviews-and-merge.md ('That cannot let unreviewed code merge, since an approval still has to cover the code') is no longer true. An honest agent could take this path as a correction, for example after a rebase orphans pins or after a hand-resolved sync merge makes the approval stale. Fix: keep an approval's claim fields (event, target, commit, identity) as first committed, letting the pre-base relaxation apply to an approval only when those are unchanged (its prose may still be corrected), or keep approvals append-only; add a merge_gates test that re-pinning an approval is refused. If the human prefers to accept this risk, name it in ADR 0028, PRODUCT_INTENT.md and reviews-and-merge.md in place of the current claim.
