---
schema: atelier.review
schema_version: 1
id: atelier-ns03
title: 'low (pre-existing on master from atelier-s2ei, not introduced by this merge or the pin threading): crates/atelier-app/src/review_gate.rs:238-244 and crates/atelier-cli/src/commands/merge_sync.rs:369-386. merge --sync''s ''Approval kept'' verdict (approval_accepted + approval_stale_edges) and merge''s review.current (current_on_branch: fork-point condition + approval_is_current, which has no ancestor-of-base skip) can disagree. Repro (scratch test in merge_gates.rs using synced_fixture, fails identically on master 1a066d44 and on this branch): approve, commit a code change C on the branch, merge the branch into main with --no-ff, then merge --sync on the branch: sync prints ''Approval kept: ... still covers the branch.'' and ''merge: passed'', but merge --check fails review.current. stale_edges skips C because the base holds it, and the approval also predates the new fork point. Impact: a misleading sync report only when the base has already taken part of the branch; land/merge still refuse, so no unreviewed merge. Suggested fix: make report_approval apply current_on_branch''s fork-point condition, or drop the ancestor skip so stale_edges is exactly the complement of approval_is_current_to; add the scenario as an agreement test.'
target: atelier-r28p
identity: Independent reviewer (Claude subagent)
commit: 1cb41498ad5694ca454fee6d93c4a6c81ad339bf
created_at: 2026-10-22T22:58:23.018295972Z
event:
  event: finding
  recommendation: defer
---

low (pre-existing on master from atelier-s2ei, not introduced by this merge or the pin threading): crates/atelier-app/src/review_gate.rs:238-244 and crates/atelier-cli/src/commands/merge_sync.rs:369-386. merge --sync's 'Approval kept' verdict (approval_accepted + approval_stale_edges) and merge's review.current (current_on_branch: fork-point condition + approval_is_current, which has no ancestor-of-base skip) can disagree. Repro (scratch test in merge_gates.rs using synced_fixture, fails identically on master 1a066d44 and on this branch): approve, commit a code change C on the branch, merge the branch into main with --no-ff, then merge --sync on the branch: sync prints 'Approval kept: ... still covers the branch.' and 'merge: passed', but merge --check fails review.current. stale_edges skips C because the base holds it, and the approval also predates the new fork point. Impact: a misleading sync report only when the base has already taken part of the branch; land/merge still refuse, so no unreviewed merge. Suggested fix: make report_approval apply current_on_branch's fork-point condition, or drop the ancestor skip so stale_edges is exactly the complement of approval_is_current_to; add the scenario as an agreement test.
