---
created_at: "2026-10-22T23:00:37.775842772+00:00"
id: "atelier-7typ"
evidence_type: "test"
commit: "e86baad316e34c0b258402ac6c9c8d00df9320ee"
result: "pass"
captured_at: "2026-10-22T23:00:28.630830031+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks --test merge_gates"
exit_status: "0"
agent_identity: "claude-implementer-fngz"
target:
  kind: "issue"
  id: "atelier-1hjj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1hjj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A records-only base move just before the locks yields one JSON report from the attempt that returns; the merge gate suite still passes"
updated_at: "2026-10-22T23:00:37.776175902+00:00"
---

## Summary

A records-only base move just before the locks yields one JSON report from the attempt that returns; the merge gate suite still passes

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks --test merge_gates
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 9647
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.33s
────────────
 Nextest run ID 26e6c76f-159a-48c5-af53-7473faad9b9f with nextest profile: default
    Starting 72 tests across 1 binary
        PASS [   0.196s] ( 1/72) atelier-cli::merge_gates approval_records_committed_by_author_remain_independent_and_merge_has_no_writes
        PASS [   0.291s] ( 2/72) atelier-cli::merge_gates an_approval_from_before_the_branch_left_the_base_is_refused_after_a_base_merge
        PASS [   0.198s] ( 3/72) atelier-cli::merge_gates branch_cleanup_does_not_require_an_outdated_upstream_to_contain_merged_work
        PASS [   0.404s] ( 4/72) atelier-cli::merge_gates base_policy_controls_shell_checks_and_candidate_cannot_loosen_them
        PASS [   0.236s] ( 5/72) atelier-cli::merge_gates candidate_cannot_remove_base_completion_checks_for_transitions_or_direct_done_claims
        PASS [   0.264s] ( 6/72) atelier-cli::merge_gates captured_checkout_refs_preserve_head_tags_aliases_and_base_worktrees
        PASS [   0.899s] ( 7/72) atelier-cli::merge_gates base_merge_resolutions_must_keep_the_blocker_and_child_the_base_added
        PASS [   1.311s] ( 8/72) atelier-cli::merge_gates base_merge_record_resolutions_keep_approval_only_when_they_lose_nothing
        PASS [   0.536s] ( 9/72) atelier-cli::merge_gates direct_terminal_claim_requires_trusted_completion_fields
        PASS [   0.207s] (10/72) atelier-cli::merge_gates divergence_and_dirty_candidate_or_base_checkout_are_refused
        PASS [   0.945s] (11/72) atelier-cli::merge_gates clean_merge_of_the_base_keeps_approval_but_resolved_or_edited_merges_do_not
        PASS [   0.219s] (12/72) atelier-cli::merge_gates evidence_capture_follows_core_filemode_for_execute_bits
        PASS [   0.321s] (13/72) atelier-cli::merge_gates edit_then_revert_and_second_author_cannot_reuse_an_approval
        PASS [   0.254s] (14/72) atelier-cli::merge_gates fast_forward_strategy_does_not_rerun_checks_for_a_base_that_moved_by_records
        PASS [   0.402s] (15/72) atelier-cli::merge_gates evidence_capture_rejects_hidden_flagged_code_before_and_during_command_without_append
        PASS [   1.298s] (16/72) atelier-cli::merge_gates code_policy_acceptance_and_notes_changes_require_fresh_approval
        PASS [   0.181s] (17/72) atelier-cli::merge_gates fast_forward_strategy_names_records_only_base_commits_it_cannot_land_on
        PASS [   0.117s] (18/72) atelier-cli::merge_gates flagged_code_executable_mode_changes_are_rejected_despite_core_filemode_false
        PASS [   0.283s] (19/72) atelier-cli::merge_gates fixed_finding_requires_a_resolution_commit_and_current_independent_approval
        PASS [   0.401s] (20/72) atelier-cli::merge_gates findings_require_dispositions_and_author_cannot_clear_fix_now
        PASS [   0.178s] (21/72) atelier-cli::merge_gates git_hooks_are_explicitly_disabled_in_the_final_integration_fence
        PASS [   0.807s] (22/72) atelier-cli::merge_gates execute_bits_hidden_by_core_filemode_do_not_block_merge_in_either_checkout
        PASS [   0.224s] (23/72) atelier-cli::merge_gates git_index_flags_cannot_hide_uncommitted_completion_claim_bytes_or_modes
        PASS [   0.193s] (24/72) atelier-cli::merge_gates ignored_canonical_file_created_by_shell_is_rejected_at_final_fence
        PASS [   0.346s] (25/72) atelier-cli::merge_gates ignored_canonical_changes_in_base_checkout_refuse_before_and_after_checks
        PASS [   0.214s] (26/72) atelier-cli::merge_gates introduced_side_code_edit_then_revert_stales_approval_after_merge
        PASS [   0.354s] (27/72) atelier-cli::merge_gates ignored_uncommitted_approval_issue_and_evidence_are_not_merge_inputs
        PASS [   0.375s] (28/72) atelier-cli::merge_gates invalid_candidate_policy_is_rejected_and_base_fallback_is_visible
        PASS [   0.350s] (29/72) atelier-cli::merge_gates merge_commit_strategy_accepts_a_sibling_the_base_completed_with_its_own_review
        PASS [   0.590s] (3
```

