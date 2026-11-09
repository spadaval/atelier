---
schema: atelier.review
schema_version: 3
id: atelier-7pjf
title: finding on branch release-0.4.0 @3de58711
branch: release-0.4.0
identity: gpt-6.1-sol
commit: 3de5871123b62e68a42239798e7816f3be0df175
created_at: 2026-11-09T21:00:20.010284687Z
event:
  event: finding
  recommendation: fix_now
---

low: CHANGELOG.md:87 The new claim that start --base from detached HEAD refuses unless the item has a recorded branch is too broad. With an unstarted item committed on main, no workflow_branch field, and an existing local branch named with that item's bare ID, git checkout --detach followed by target/debug/atelier start <id> --base main exits 0, switches to that branch, records it and applies start. crates/atelier-cli/src/commands/start.rs:119 refuses only when the target branch does not exist and HEAD is detached. This gives users the wrong condition for detached-HEAD compatibility. Say it refuses when it would need to create a new branch, rather than whenever the item lacks a recorded branch. Recommendation: FIX NOW; introduced by this documentation fix.

Re-review pinned to 3de5871123b62e68a42239798e7816f3be0df175. Verdict: merge after this correction; complexity low (changelog only), value medium (accurate upgrade instructions), risk low (documentation). Contract: fail on the detached-HEAD exception above. Fitness: fail until that claim is corrected. No other findings or open questions.

The four original corrections are otherwise supported: legacy [decision one-way] makes check exit 1 with the exact invalid-tag finding, rewriting to [decision] restores exit 0, and --one-way exits 2; bare-ID branch creation, current unowned branch adoption, deprecated/ignored --base and policy base_branch merge targeting match the sources; CI accepts identical-code earlier commits and record-only branches need no run; migration requires loose_end in the policy and --apply for writes.

Proof passed: cargo build -p atelier-cli; target/debug/atelier review request; all three printed merge checks (git diff --check HEAD^ HEAD -- . ':!.atelier', cargo fmt -- --check, cargo metadata --locked --format-version 1 > /dev/null). Six targeted nextest tests passed: ci_passed_passes_on_the_head_or_an_earlier_commit_with_its_code, a_branch_that_changes_only_records_needs_no_run_and_no_gh, start_takes_an_unowned_current_branch_whatever_its_name, a_recorded_branch_base_is_ignored_and_the_policy_base_is_the_base, the_merge_base_is_the_branch_policys_base_checked_against_its_own_policy, migrate_after_merge_turns_open_lines_into_loose_ends_once. Direct CLI scenarios ran in a disposable fixture under target, including detached refusal with no existing branch and success with an existing unrecorded bare-ID branch; fixture removed. Residual risk: no full suite or live GitHub CI exercised for this changelog-only re-review.
