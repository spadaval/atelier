---
schema: atelier.review
schema_version: 2
id: atelier-bu66
title: 'medium (pre-existing, not introduced here): crates/atelier-cli/src/commands/merge.rs:493 runs merge_claim_nodes, merge''s check of done claims outside the owner''s scope, only when a merge commit is built, so under fast_forward_only merge integrates a done claim land refuses. Scratch repro with this branch and with master 99ae3aa6: a policy whose mission close requires work.terminal, a done mission on main, a branch that files an open issue advancing it, approve, close: land on the branch refuses ''<mission> work.terminal: open work'', merge --check passes (exit 0) and after integration land on main refuses; under merge_commit merge --check refuses the same claim. This breaks the 1ztv rule that merge refuses before integrating any done claim land would refuse, and docs/product/reviews-and-merge.md:208-213 says merge checks these claims as land does without naming the strategy. Fix: under fast_forward_only run the same claim check against the branch head (the would-be base head), trusting base_head, and add a both-strategies test; file as a follow-up under atelier-k8h8.'
target: atelier-r28p
identity: Independent reviewer (Claude subagent)
commit: dc4ce18d56aa480c2546165683341572b1d78b2a
created_at: 2026-10-22T22:35:48.952003505Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: b6193bb3ebb3ae9fea10e04df4233c996ce5e196
    recorded_at: 2026-10-22T22:43:26.004223255Z
    resolution_commit: 8a2a436a1a058d6ceb6775aafb2695684e6453eb
    reason: Fixed in 8a2a436a
    migrated_from: atelier-djml
---

medium (pre-existing, not introduced here): crates/atelier-cli/src/commands/merge.rs:493 runs merge_claim_nodes, merge's check of done claims outside the owner's scope, only when a merge commit is built, so under fast_forward_only merge integrates a done claim land refuses. Scratch repro with this branch and with master 99ae3aa6: a policy whose mission close requires work.terminal, a done mission on main, a branch that files an open issue advancing it, approve, close: land on the branch refuses '<mission> work.terminal: open work', merge --check passes (exit 0) and after integration land on main refuses; under merge_commit merge --check refuses the same claim. This breaks the 1ztv rule that merge refuses before integrating any done claim land would refuse, and docs/product/reviews-and-merge.md:208-213 says merge checks these claims as land does without naming the strategy. Fix: under fast_forward_only run the same claim check against the branch head (the would-be base head), trusting base_head, and add a both-strategies test; file as a follow-up under atelier-k8h8.
