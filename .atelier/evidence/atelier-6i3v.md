---
created_at: "2026-10-17T08:34:18.937324404+00:00"
id: "atelier-6i3v"
evidence_type: "test"
commit: "6238815d257889ba9791e100a47d7c4858abf34e"
result: "pass"
captured_at: "2026-10-17T08:33:45.877835879+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-91w1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-91w1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-pr4k) with check's output resolved by hand"
updated_at: "2026-10-17T08:34:18.938053925+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-pr4k) with check's output resolved by hand

## Command

```console
cargo nextest run -j 4
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 91059
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-91w1/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.73s
────────────
 Nextest run ID 4ea0fca0-a36e-45a8-b81a-48c6138acdde with nextest profile: default
    Starting 728 tests across 10 binaries
        PASS [   2.060s] (  1/728) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   2.760s] (  2/728) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   5.603s] (  3/728) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   2.907s] (  4/728) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   5.689s] (  5/728) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.108s] (  6/728) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.009s] (  7/728) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.039s] (  8/728) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.040s] (  9/728) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.069s] ( 10/728) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.169s] ( 11/728) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.212s] ( 12/728) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.073s] ( 13/728) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   4.113s] ( 14/728) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.156s] ( 15/728) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.385s] ( 16/728) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.127s] ( 17/728) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.082s] ( 18/728) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.112s] ( 19/728) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.366s] ( 20/728) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.119s] ( 21/728) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.089s] ( 22/728) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.066s] ( 23/728) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.087s] ( 24/728) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.041s] ( 25/728) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.054s] ( 26/728) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.138s] ( 27/728) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.006s] ( 28/728) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 29/728) atelier-app
```

