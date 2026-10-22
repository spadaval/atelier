---
created_at: "2026-10-22T22:52:18.961030714+00:00"
id: "atelier-r4zo"
evidence_type: "test"
commit: "75d2dc1389174d45d8362abf14b10dc0eef309a7"
result: "pass"
captured_at: "2026-10-22T22:51:34.322064518+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-ss25"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ss25"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "## Summary\n\nFull workspace suite with test hooks passes after the review fixes\n\n## Command\n\n```console\ncargo nextest run -j 4 --workspace --features test-hooks\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nTruncated: no\n\n```text\n```\n\n## Stderr\n\nBytes: 108598\nTruncated: yes\n\n```text\n   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.06s\n────────────\n Nextest run ID 008aa99b-c259-4540-b59e-2a767c87389d with nextest profile: default\n    Starting 863 tests across 10 binaries\n        PASS [   0.126s] (  1/863) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry\n        PASS [   0.262s] (  2/863) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind\n        PASS [   0.341s] (  3/863) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths\n        PASS [   0.081s] (  4/863) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error\n        PASS [   0.452s] (  5/863) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files\n        PASS [   0.232s] (  6/863) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification\n        PASS [   0.045s] (  7/863) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches\n        PASS [   0.010s] (  8/863) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems\n        PASS [   0.017s] (  9/863) atelier-app epic_migration::tests::migration_refuses_to_drop_sections\n        PASS [   0.089s] ( 10/863) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy\n        PASS [   0.085s] ( 11/863) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim\n        PASS [   0.266s] ( 12/863) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation\n        PASS [   0.330s] ( 13/863) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles\n        PASS [   0.084s] ( 14/863) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head\n        PASS [   1.085s] ( 15/863) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors\n        PASS [   0.241s] ( 16/863) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts\n        PASS [   0.289s] ( 17/863) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities\n        PASS [   0.706s] ( 18/863) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history\n        PASS [   0.132s] ( 19/863) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8\n        PASS [   0.268s] ( 20/863) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts\n        PASS [   0.798s] ( 21/863) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries\n        PASS [   0.144s] ( 22/863) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats\n        PASS [   0.217s] ( 23/863) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins\n        PASS [   0.142s] ( 24/863) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins\n        PASS [   0.152s] ( 25/863) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true\n        PASS [   0.101s] ( 26/863) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history\n        PASS [   0.150s] ( 27/863) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints\n        PASS [   2.126s] ( 28/863) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters\n        PASS [   0.007s] ( 29/863) ate\n```\n (rerun after merging master)"
updated_at: "2026-10-22T22:52:18.961525285+00:00"
---

## Summary

## Summary

Full workspace suite with test hooks passes after the review fixes

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 108598
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.06s
────────────
 Nextest run ID 008aa99b-c259-4540-b59e-2a767c87389d with nextest profile: default
    Starting 863 tests across 10 binaries
        PASS [   0.126s] (  1/863) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.262s] (  2/863) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.341s] (  3/863) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.081s] (  4/863) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.452s] (  5/863) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.232s] (  6/863) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.045s] (  7/863) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.010s] (  8/863) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.017s] (  9/863) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.089s] ( 10/863) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.085s] ( 11/863) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.266s] ( 12/863) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.330s] ( 13/863) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.084s] ( 14/863) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.085s] ( 15/863) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.241s] ( 16/863) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.289s] ( 17/863) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.706s] ( 18/863) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.132s] ( 19/863) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.268s] ( 20/863) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.798s] ( 21/863) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.144s] ( 22/863) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.217s] ( 23/863) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.142s] ( 24/863) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.152s] ( 25/863) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.101s] ( 26/863) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.150s] ( 27/863) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   2.126s] ( 28/863) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.007s] ( 29/863) ate
```
 (rerun after merging master)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 109967
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.08s
────────────
 Nextest run ID fa4e38f9-559e-4a37-ba40-a31d08036b40 with nextest profile: default
    Starting 874 tests across 10 binaries
        PASS [   0.107s] (  1/874) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.214s] (  2/874) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.252s] (  3/874) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.079s] (  4/874) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.129s] (  5/874) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.347s] (  6/874) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.011s] (  7/874) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.009s] (  8/874) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.044s] (  9/874) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.073s] ( 10/874) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.087s] ( 11/874) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.166s] ( 12/874) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.210s] ( 13/874) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.089s] ( 14/874) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.739s] ( 15/874) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.136s] ( 16/874) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.395s] ( 17/874) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.122s] ( 18/874) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.075s] ( 19/874) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.158s] ( 20/874) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.436s] ( 21/874) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.220s] ( 22/874) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.176s] ( 23/874) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.446s] ( 24/874) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   1.761s] ( 25/874) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.260s] ( 26/874) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.804s] ( 27/874) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.387s] ( 28/874) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.005s] ( 29/874) ate
```

