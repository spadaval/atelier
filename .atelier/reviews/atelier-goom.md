---
schema: atelier.review
schema_version: 1
id: atelier-goom
title: 'MEDIUM crates/atelier-cli/src/commands/merge.rs:341-343 (merge_claim_nodes called with the branch-head ReadContext): review_gate::evaluate_scope checks every review on a touched claim with ancestor(review.commit, "HEAD"), and HEAD is the work branch head, so any claim the base completed after the fork point with its own review records, and that the branch''s record changes reach (e.g. a sibling advancing the same mission, reached through declared_by), bails with ''review ... pins a commit outside the work branch, usually because the branch was rebased''. Scenario reproduced: mission M planned on base with owner and sibling X advancing it; branch merges base, is approved; X lands on base with an approval; atelier merge --check refuses. This is fail-closed, but it blocks the main use case of merge_commit (landing mission work behind a moved base) and gives wrong recovery advice. Fix: pass git::ReadContext::from_head(repo, commit.clone()) (the merge commit) to merge_claim_nodes; trial patch passes the probe and existing tests. Add the probe as a test.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: 95f9df974c3f8c689c776725a19d3792a3f87ae6
created_at: 2026-10-17T07:09:53.108795502Z
event:
  event: finding
  recommendation: fix_now
---

MEDIUM crates/atelier-cli/src/commands/merge.rs:341-343 (merge_claim_nodes called with the branch-head ReadContext): review_gate::evaluate_scope checks every review on a touched claim with ancestor(review.commit, "HEAD"), and HEAD is the work branch head, so any claim the base completed after the fork point with its own review records, and that the branch's record changes reach (e.g. a sibling advancing the same mission, reached through declared_by), bails with 'review ... pins a commit outside the work branch, usually because the branch was rebased'. Scenario reproduced: mission M planned on base with owner and sibling X advancing it; branch merges base, is approved; X lands on base with an approval; atelier merge --check refuses. This is fail-closed, but it blocks the main use case of merge_commit (landing mission work behind a moved base) and gives wrong recovery advice. Fix: pass git::ReadContext::from_head(repo, commit.clone()) (the merge commit) to merge_claim_nodes; trial patch passes the probe and existing tests. Add the probe as a test.
