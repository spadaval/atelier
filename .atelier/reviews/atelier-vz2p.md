---
schema: atelier.review
schema_version: 2
id: atelier-vz2p
title: 'medium: crates/atelier-cli/src/commands/git.rs:300 (land) vs merge claim gates: merge still integrates an out-of-scope done claim that land then refuses. Reproduced in a scratch copy of tests/session.rs: after approval, a branch commit that creates a new issue already done (parent none) together with the owner''s close gives ''merge --check'' pass=true (review.current current, nothing else fails) and, after the fast-forward, ''land'' fails with land_base_done_claim naming the new issue. The new-file rule in allowed_edge lets the approval stay current, and merge_claim_nodes (session.rs:488) judges only proof gates of claims outside the scope, not whether an approval covers them, which is what land asks. This is the exact failure uw4x names (refusal arrives only after integration) and it contradicts the Background rule ''merge refuses before integrating any other done claim land would refuse'', the decision note in n642/uw4x (''no done claim merge integrates that land then refuses'') and the docs claim of one rule. Fix: have merge refuse before integrating a done claim in the branch range that no current approval scope covers (reuse claim_after_current_approval / base_done_claims logic), with a test; or narrow the claim and record the remaining gap as a follow-up before closing uw4x.'
target: atelier-1ztv
identity: Independent reviewer (Claude subagent)
commit: f7e1cefa82d568b23f9b06cfc193ab350def07c1
created_at: 2026-10-22T21:12:53.184558873Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 7ad5dcecf0f7db73588dd0dce4752b4a344a6e09
    recorded_at: 2026-10-22T21:29:59.725895673Z
    resolution_commit: 23a9aab85262c52ef137700911e84d159c30d3e7
    reason: Fixed in 23a9aab8
    migrated_from: atelier-s4yz
---

medium: crates/atelier-cli/src/commands/git.rs:300 (land) vs merge claim gates: merge still integrates an out-of-scope done claim that land then refuses. Reproduced in a scratch copy of tests/session.rs: after approval, a branch commit that creates a new issue already done (parent none) together with the owner's close gives 'merge --check' pass=true (review.current current, nothing else fails) and, after the fast-forward, 'land' fails with land_base_done_claim naming the new issue. The new-file rule in allowed_edge lets the approval stay current, and merge_claim_nodes (session.rs:488) judges only proof gates of claims outside the scope, not whether an approval covers them, which is what land asks. This is the exact failure uw4x names (refusal arrives only after integration) and it contradicts the Background rule 'merge refuses before integrating any other done claim land would refuse', the decision note in n642/uw4x ('no done claim merge integrates that land then refuses') and the docs claim of one rule. Fix: have merge refuse before integrating a done claim in the branch range that no current approval scope covers (reuse claim_after_current_approval / base_done_claims logic), with a test; or narrow the claim and record the remaining gap as a follow-up before closing uw4x.
