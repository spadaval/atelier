---
schema: atelier.review
schema_version: 2
id: atelier-q4d2
title: 'review_gate.rs:122-130 also changes authorship: author_identities_to (review_gate.rs:247) uses allowed_edge, so a commit that only creates an out-of-scope issue no longer makes its committer an author of the scope. This is intended (a reviewer who files the defer follow-up keeps independence) and safe, since any other path in the same commit still counts. docs/product/reviews-and-merge.md (Approval freshness) does not say so, and its new paragraph has one unwrapped long line. Cosmetic only.'
target: atelier-cafy
identity: Independent reviewer (Claude subagent)
commit: b1537aef4483d7a6867ef4924b3b16f603f550e5
created_at: 2026-10-16T18:27:43.435024478Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Claude Opus 5.5 (orchestrator)
    commit: f7ae95951b94ee98e6dcc1b8341020b1cf098692
    recorded_at: 2026-10-16T18:28:08.119220640Z
    reason: 'Accepted: a reviewer who files the follow-up staying independent is intended; the docs describe freshness, and authorship follows the same allowed edges.'
    migrated_from: atelier-6772
---

review_gate.rs:122-130 also changes authorship: author_identities_to (review_gate.rs:247) uses allowed_edge, so a commit that only creates an out-of-scope issue no longer makes its committer an author of the scope. This is intended (a reviewer who files the defer follow-up keeps independence) and safe, since any other path in the same commit still counts. docs/product/reviews-and-merge.md (Approval freshness) does not say so, and its new paragraph has one unwrapped long line. Cosmetic only.
