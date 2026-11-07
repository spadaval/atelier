---
schema: atelier.review
schema_version: 2
id: atelier-jjqr
title: 'The test does not show that the advice works. rebasing_a_reviewed_branch_explains_the_cause_and_recovery (crates/atelier-cli/tests/merge_gates.rs:589) records proof after the rebase, cherry-picks it, merges, and checks only that the evidence file exists on the base. Fixture::new uses the starter policy, which has no evidence.attached gate, so the merge passes whether or not the cherry-picked proof counts, and it does not count: it pins a rewritten commit (see the companion finding). The assertion confirms that git cherry-pick copies a file. It does not show the recovery keeps proof valid. It also never records a review event after the rebase, so the ''record review events again'' half of the advice is untested. Once the advice is corrected, the test should gate on evidence: add evidence.attached to the base policy, or assert merge --check reports a fresh pass from proof recorded after the recovery. It should also cover a post-rebase review event that is dropped and recorded again.'
target: atelier-v9ve
identity: Independent reviewer (Claude subagent)
commit: 501e6b6a81c45883ce218b3a3cbe73ce47f29e9c
created_at: 2026-10-16T20:00:55.493719664Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 3f5e3cd9b7b091bb45f2789893844439aa38af45
    recorded_at: 2026-10-16T20:08:38.847940468Z
    resolution_commit: b3076c97ff51c496ae1c48d3dd637818743703c9
    reason: 'Resolved: the test now adds evidence.attached to the base policy, makes a fix and proof after the rebase, follows the advice by cherry-picking only the fix, asserts FAIL review.current and FAIL evidence.attached, then records proof and approval again and merges. cargo nextest run --workspace passes (636/636).'
    migrated_from: atelier-7yqx
---

The test does not show that the advice works. rebasing_a_reviewed_branch_explains_the_cause_and_recovery (crates/atelier-cli/tests/merge_gates.rs:589) records proof after the rebase, cherry-picks it, merges, and checks only that the evidence file exists on the base. Fixture::new uses the starter policy, which has no evidence.attached gate, so the merge passes whether or not the cherry-picked proof counts, and it does not count: it pins a rewritten commit (see the companion finding). The assertion confirms that git cherry-pick copies a file. It does not show the recovery keeps proof valid. It also never records a review event after the rebase, so the 'record review events again' half of the advice is untested. Once the advice is corrected, the test should gate on evidence: add evidence.attached to the base policy, or assert merge --check reports a fresh pass from proof recorded after the recovery. It should also cover a post-rebase review event that is dropped and recorded again.
