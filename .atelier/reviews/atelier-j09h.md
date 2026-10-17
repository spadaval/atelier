---
schema: atelier.review
schema_version: 1
id: atelier-j09h
title: 'Approved at a5255030 after reviewing bd6fbfaf..HEAD. atelier-goom fix (7384d0dd) reads merge_claim_nodes at the merge commit via ReadContext::from_snapshot(repo, commit, &[]); from_snapshot over from_head is right, since it marks the read clean instead of consulting the work checkout''s working tree for a commit that checkout does not hold (merge already requires a clean checkout). The new test merge_commit_strategy_accepts_a_sibling_the_base_completed_with_its_own_review fails when the call is reverted to the branch-head context (mutation, reverted) and passes with the fix. atelier-liml docs rewrite reads clearly and accurately describes the merge-commit read. git show --remerge-diff 1d90c9f5 is empty (clean master merge; atelier-7iyw local-day dating does not touch merge). The close commit''s Close Reason matches the code and tests, including the land_base_changed/merge_snapshot_changed deviation. Evidence atelier-e8aj at 1d90c9f5: full suite with test hooks, 701 tests, exit 0; I re-ran merge_gates and session with test hooks at HEAD (139 pass). Earlier findings all have dispositions; deferred atelier-gle3 is tracked in atelier-rcnk.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: a5255030fe6268564275aab07d104940f704bd0d
created_at: 2026-10-17T07:15:32.449838740Z
event:
  event: approval
---

Approved at a5255030 after reviewing bd6fbfaf..HEAD. atelier-goom fix (7384d0dd) reads merge_claim_nodes at the merge commit via ReadContext::from_snapshot(repo, commit, &[]); from_snapshot over from_head is right, since it marks the read clean instead of consulting the work checkout's working tree for a commit that checkout does not hold (merge already requires a clean checkout). The new test merge_commit_strategy_accepts_a_sibling_the_base_completed_with_its_own_review fails when the call is reverted to the branch-head context (mutation, reverted) and passes with the fix. atelier-liml docs rewrite reads clearly and accurately describes the merge-commit read. git show --remerge-diff 1d90c9f5 is empty (clean master merge; atelier-7iyw local-day dating does not touch merge). The close commit's Close Reason matches the code and tests, including the land_base_changed/merge_snapshot_changed deviation. Evidence atelier-e8aj at 1d90c9f5: full suite with test hooks, 701 tests, exit 0; I re-ran merge_gates and session with test hooks at HEAD (139 pass). Earlier findings all have dispositions; deferred atelier-gle3 is tracked in atelier-rcnk.
