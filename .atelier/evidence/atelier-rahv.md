---
created_at: "2026-10-22T23:28:03.353921252+00:00"
id: "atelier-rahv"
evidence_type: "test"
commit: "9d682eba58800134ce73ba81f6851b4886edbf96"
result: "pass"
captured_at: "2026-10-22T23:26:59.717825926+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "## Summary\n\nFull workspace suite with test-hooks passes on the branch (all four fngz items)\n\n## Command\n\n```console\ncargo nextest run -j 4 --workspace --features test-hooks\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nTruncated: no\n\n```text\n```\n\n## Stderr\n\nBytes: 110435\nTruncated: yes\n\n```text\ncargo-gate: 2 builds already running, waiting for a slot...\n   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.92s\n────────────\n Nextest run ID abcaac48-70d4-4657-8389-04d78834de92 with nextest profile: default\n    Starting 876 tests across 10 binaries\n        PASS [   0.115s] (  1/876) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry\n        PASS [   0.236s] (  2/876) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind\n        PASS [   0.292s] (  3/876) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths\n        PASS [   0.099s] (  4/876) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error\n        PASS [   0.177s] (  5/876) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification\n        PASS [   0.446s] (  6/876) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files\n        PASS [   0.016s] (  7/876) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems\n        PASS [   0.014s] (  8/876) atelier-app epic_migration::tests::migration_refuses_to_drop_sections\n        PASS [   0.065s] (  9/876) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches\n        PASS [   0.064s] ( 10/876) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy\n        PASS [   0.090s] ( 11/876) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim\n        PASS [   0.226s] ( 12/876) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation\n        PASS [   0.200s] ( 13/876) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles\n        PASS [   0.072s] ( 14/876) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head\n        PASS [   0.755s] ( 15/876) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors\n        PASS [   0.199s] ( 16/876) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts\n        PASS [   0.203s] ( 17/876) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities\n        PASS [   0.526s] ( 18/876) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history\n        PASS [   0.082s] ( 19/876) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8\n        PASS [   0.515s] ( 20/876) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries\n        PASS [   0.120s] ( 21/876) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts\n        PASS [   0.159s] ( 22/876) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins\n        PASS [   0.130s] ( 23/876) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats\n        PASS [   0.085s] ( 24/876) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins\n        PASS [   0.049s] ( 25/876) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history\n        PASS [   0.146s] ( 26/876) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true\n        PASS [   0.092s] ( 27/876) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints\n        PASS [   0.167s] ( 28/876) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type\n```\n (rerun after merging master)"
updated_at: "2026-10-22T23:28:03.354726172+00:00"
---

## Summary

## Summary

Full workspace suite with test-hooks passes on the branch (all four fngz items)

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

Bytes: 110435
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.92s
────────────
 Nextest run ID abcaac48-70d4-4657-8389-04d78834de92 with nextest profile: default
    Starting 876 tests across 10 binaries
        PASS [   0.115s] (  1/876) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.236s] (  2/876) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.292s] (  3/876) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.099s] (  4/876) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.177s] (  5/876) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.446s] (  6/876) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.016s] (  7/876) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.014s] (  8/876) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.065s] (  9/876) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.064s] ( 10/876) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.090s] ( 11/876) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.226s] ( 12/876) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.200s] ( 13/876) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.072s] ( 14/876) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.755s] ( 15/876) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.199s] ( 16/876) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.203s] ( 17/876) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.526s] ( 18/876) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.082s] ( 19/876) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.515s] ( 20/876) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.120s] ( 21/876) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.159s] ( 22/876) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.130s] ( 23/876) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.085s] ( 24/876) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.049s] ( 25/876) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.146s] ( 26/876) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.092s] ( 27/876) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.167s] ( 28/876) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type
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

Bytes: 111819
Truncated: yes

```text
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.71s
────────────
 Nextest run ID 70cf73cc-ef9e-49f6-bb06-f157fb3d6a0a with nextest profile: default
    Starting 888 tests across 10 binaries
        PASS [   0.087s] (  1/888) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.212s] (  2/888) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.242s] (  3/888) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.073s] (  4/888) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.128s] (  5/888) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.368s] (  6/888) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.011s] (  7/888) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.040s] (  8/888) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.010s] (  9/888) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.046s] ( 10/888) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.074s] ( 11/888) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.169s] ( 12/888) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.211s] ( 13/888) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.653s] ( 14/888) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.071s] ( 15/888) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.145s] ( 16/888) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.396s] ( 17/888) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.145s] ( 18/888) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.074s] ( 19/888) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.365s] ( 20/888) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.107s] ( 21/888) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.876s] ( 22/888) atelier-app git_freshness::tests::ancestry_path_matches_git_when_a_side_branch_is_dated_before_its_parents
        PASS [   0.141s] ( 23/888) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.083s] ( 24/888) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.105s] ( 25/888) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.045s] ( 26/888) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.104s] ( 27/888) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.067s] ( 28/888) atelier-app git_freshness::tests::wor
```

