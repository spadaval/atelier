---
schema: atelier.review
schema_version: 1
id: atelier-qyqu
title: 'Reviewed git diff master...feature/atelier-ydv0 at 7185c3d0 (merge.rs, git_freshness.rs merge_tree extraction, review_gate.rs checks_root, main.rs help, merge_gates.rs tests, docs) against the atelier-ydv0 Done-when list and atelier-k8h8 constraints. Ran the five merge_commit tests (pass). Mutations: running merge_checks in the work checkout instead of the merged checkout fails the clean-merge and lockfile tests; disabling the conflict refusal fails the conflict test, so the tests exercise their claims. Ad-hoc probes (reverted, tree clean): a fix_now finding recorded on the base after the fork point is ignored and the merge commit lands (land then refuses findings.disposed); merge_checks of only lint.none_blocking satisfy merge_unproven and land while merge.merged_tree claims merged-tree proof; doctor passes after a merge commit lands. fmt clean. master''s kops trust-point change is equivalent on the base checkout (no branch_base), so land after a merge commit is unaffected. Limits: did not run the full suite or clippy, did not test squash/other strategies, did not exercise .gitattributes merge drivers or interrupted-checks cleanup.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: 7185c3d00f161519ba59396b48f26ff2595cd28b
created_at: 2026-10-17T06:10:47.559891174Z
event:
  event: review
---

Reviewed git diff master...feature/atelier-ydv0 at 7185c3d0 (merge.rs, git_freshness.rs merge_tree extraction, review_gate.rs checks_root, main.rs help, merge_gates.rs tests, docs) against the atelier-ydv0 Done-when list and atelier-k8h8 constraints. Ran the five merge_commit tests (pass). Mutations: running merge_checks in the work checkout instead of the merged checkout fails the clean-merge and lockfile tests; disabling the conflict refusal fails the conflict test, so the tests exercise their claims. Ad-hoc probes (reverted, tree clean): a fix_now finding recorded on the base after the fork point is ignored and the merge commit lands (land then refuses findings.disposed); merge_checks of only lint.none_blocking satisfy merge_unproven and land while merge.merged_tree claims merged-tree proof; doctor passes after a merge commit lands. fmt clean. master's kops trust-point change is equivalent on the base checkout (no branch_base), so land after a merge commit is unaffected. Limits: did not run the full suite or clippy, did not test squash/other strategies, did not exercise .gitattributes merge drivers or interrupted-checks cleanup.
