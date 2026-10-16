---
schema: atelier.review
schema_version: 1
id: atelier-z26w
title: 'Resolved: the review_gate.rs error and reviews-and-merge.md now say to cherry-pick only code or planning commits made after the rebase, leave out review and evidence records, and record proof and review again. This matches the code: an off-branch review record makes merge bail again (review_gate.rs evaluate_scope), and off-branch evidence never counts (proof_matches/evidence_is_fresh fail on a non-ancestor pin and are unwrap_or(false)).'
target: atelier-v9ve
identity: Independent reviewer (Claude subagent)
commit: 3f5e3cd9b7b091bb45f2789893844439aa38af45
created_at: 2026-10-16T20:08:38.733730342Z
event:
  event: disposition
  finding: atelier-97kx
  decision: fix_now
  supersedes: null
  resolution_commit: b3076c97ff51c496ae1c48d3dd637818743703c9
  follow_up: null
---

Resolved: the review_gate.rs error and reviews-and-merge.md now say to cherry-pick only code or planning commits made after the rebase, leave out review and evidence records, and record proof and review again. This matches the code: an off-branch review record makes merge bail again (review_gate.rs evaluate_scope), and off-branch evidence never counts (proof_matches/evidence_is_fresh fail on a non-ancestor pin and are unwrap_or(false)).
