---
schema: atelier.review
schema_version: 1
id: atelier-plu8
title: 'low: crates/atelier-cli/tests/session.rs:5845-5930 (adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it) and evidence atelier-5aj4 claim merge --check agrees with land, but the test runs only under the Fixture''s fast_forward_only policy, where merge never runs the out-of-scope claim check (merge_claim_nodes is called only when a merge commit exists, crates/atelier-cli/src/commands/merge.rs:493). The merge assertion at :5928 therefore cannot fail: with the master binary (99ae3aa6) the same scenario under fast_forward_only also passes merge --check (0 reports of the old sibling) while land on the base and on the branch report it. Under merge_commit the assertion is meaningful (master: merge --check reports the sibling; this branch: it does not), so the behavior is right but its only merge/land agreement proof for ss25 is vacuous, and r28p''s Done when asks for land and merge tests that check the two agree. Fix: run the scenario under both strategies, as close_after_approval does (merge_commit with a true shell.check), so the merge leg exercises touched_claims.'
target: atelier-r28p
identity: Independent reviewer (Claude subagent)
commit: dc4ce18d56aa480c2546165683341572b1d78b2a
created_at: 2026-10-22T22:35:47.840063956Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/tests/session.rs:5845-5930 (adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it) and evidence atelier-5aj4 claim merge --check agrees with land, but the test runs only under the Fixture's fast_forward_only policy, where merge never runs the out-of-scope claim check (merge_claim_nodes is called only when a merge commit exists, crates/atelier-cli/src/commands/merge.rs:493). The merge assertion at :5928 therefore cannot fail: with the master binary (99ae3aa6) the same scenario under fast_forward_only also passes merge --check (0 reports of the old sibling) while land on the base and on the branch report it. Under merge_commit the assertion is meaningful (master: merge --check reports the sibling; this branch: it does not), so the behavior is right but its only merge/land agreement proof for ss25 is vacuous, and r28p's Done when asks for land and merge tests that check the two agree. Fix: run the scenario under both strategies, as close_after_approval does (merge_commit with a true shell.check), so the merge leg exercises touched_claims.
