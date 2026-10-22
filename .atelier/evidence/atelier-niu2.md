---
created_at: "2026-10-22T20:39:58.307291880+00:00"
id: "atelier-niu2"
evidence_type: "test"
commit: "8ef209697fd5a28a9241c8fadf66a1452368839b"
result: "pass"
captured_at: "2026-10-22T20:39:31.184910647+00:00"
command: "cargo nextest run -j4 --features test-hooks -p atelier-cli -E 'binary(session) or test(branches_without_a_tracker)'"
exit_status: "0"
agent_identity: "claude-implementer-ftqn"
target:
  kind: "issue"
  id: "atelier-ftqn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ftqn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Branches section rows, Unowned line and updated status tests pass"
updated_at: "2026-10-22T20:39:58.307807431+00:00"
---

## Summary

Branches section rows, Unowned line and updated status tests pass

## Command

```console
cargo nextest run -j4 --features test-hooks -p atelier-cli -E 'binary(session) or test(branches_without_a_tracker)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 11202
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-9g48/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.02s
────────────
 Nextest run ID 75efef4b-0c21-447f-a299-eecee05442bb with nextest profile: default
    Starting 90 tests across 6 binaries (538 tests skipped)
        PASS [   0.177s] ( 1/90) atelier-cli::cli_integration mission_worktree::branches_without_a_tracker_are_summarised_without_git_errors
        PASS [   0.315s] ( 2/90) atelier-cli::session a_branch_cannot_weaken_the_policy_around_a_claim_to_skip_the_report
        PASS [   0.323s] ( 3/90) atelier-cli::session a_current_claim_whose_pass_a_later_commit_staled_does_not_blame_the_checkout
        PASS [   0.300s] ( 4/90) atelier-cli::session a_retained_claim_whose_pass_later_commits_staled_does_not_blame_the_checkout
        PASS [   0.508s] ( 5/90) atelier-cli::session a_kept_branch_whose_done_owner_was_pruned_from_the_base_shows_merged
        PASS [   0.331s] ( 6/90) atelier-cli::session a_superseded_childs_own_blockers_do_not_hold_its_parents_merge
        PASS [   0.241s] ( 7/90) atelier-cli::session active_owner_skips_merge_shell_checks_but_terminal_owner_enforces_them
        PASS [   0.120s] ( 8/90) atelier-cli::session ambiguous_remote_branch_refuses_before_switching_or_writing
        PASS [   0.350s] ( 9/90) atelier-cli::session active_epic_hands_off_completed_children_without_batch_approval
        PASS [   0.549s] (10/90) atelier-cli::session a_task_parent_owns_its_children_through_start_land_and_merge
        PASS [   0.308s] (11/90) atelier-cli::session an_adopted_repositorys_branch_cannot_pass_as_an_install
        PASS [   0.238s] (12/90) atelier-cli::session base_commit_that_installs_atelier_is_exempt_from_review_coverage
        PASS [   1.086s] (13/90) atelier-cli::session approval_from_before_a_clean_base_merge_lands_on_both_branches
        PASS [   1.215s] (14/90) atelier-cli::session base_done_claims_survive_a_broken_record_and_merges_judge_every_parent
        PASS [   1.283s] (15/90) atelier-cli::session base_done_claim_check_reads_the_nearest_parseable_earlier_copy
        PASS [   1.992s] (16/90) atelier-cli::session base_branch_accepts_record_only_commits_and_reviewed_merges_but_not_direct_code
        PASS [   0.923s] (17/90) atelier-cli::session base_pull_merge_counts_only_the_paths_it_introduces
        PASS [   1.622s] (18/90) atelier-cli::session base_land_retains_claims_a_merge_tip_brings_in_through_its_other_parent
        PASS [   0.685s] (19/90) atelier-cli::session completed_owner_shell_cannot_hide_code_changes_from_landing_fence
        PASS [   0.612s] (20/90) atelier-cli::session cross_branch_status_keeps_base_clean_and_reports_legacy_without_loading_it
        PASS [   0.531s] (21/90) atelier-cli::session custom_branch_only_owner_is_discovered_without_overlaying_base
        PASS [   1.786s] (22/90) atelier-cli::session canonical_changes_refuse_landing_in_every_git_state
        PASS [   1.075s] (23/90) atelier-cli::session documented_hook_events_use_nested_cwd_and_stop_loop_protection
        PASS [   1.107s] (24/90) atelier-cli::session done_claims_on_work_branches_say_they_are_not_merged
        PASS [   0.604s] (25/90) atelier-cli::session done_owner_full_gates_and_invalid_policies_are_checked_without_writes
        PASS [   1.481s] (26/90) atelier-cli::session done_child_cannot_hide_open_dependencies_new_failure_or_missing_required_fields
        PASS [   0.511s] (27/90) atelier-cli::session done_owner_requires_approval_and_active_failures_can_handoff
        PASS [   3.936s] (28/90) atelier-cli::session base_refuses_done_claims_outside_review_but_allows_supersede_and_decisions
        PASS [   0.638s] (29/90) atelier-cli::session execute_bits_hidden_by_core_filemode_follow_git_status_in_land
        PASS [   0.482s] (30/90) atelier-cli::session global_terminal_claims_require_fields_and_resolved_dependencies_on_any_checkout
        PASS [   0.264s] (31/90) atelier-c
```

