---
created_at: "2026-10-17T19:49:35.640989035+00:00"
id: "atelier-psaf"
evidence_type: "test"
commit: "2390abccd956ba5b662ffcddcabce04cdfef6c36"
result: "pass"
captured_at: "2026-10-17T19:48:53.811274408+00:00"
command: "cargo nextest run -j 4 --workspace"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-acrt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-acrt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass without features"
updated_at: "2026-10-17T19:49:35.641736088+00:00"
---

## Summary

Workspace tests pass without features

## Command

```console
cargo nextest run -j 4 --workspace
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 100215
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-acrt/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.74s
────────────
 Nextest run ID 74e5c561-6411-45eb-9d3a-3a6300478dff with nextest profile: default
    Starting 801 tests across 10 binaries
        PASS [   0.137s] (  1/801) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.205s] (  2/801) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.227s] (  3/801) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.041s] (  4/801) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.082s] (  5/801) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.311s] (  6/801) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.023s] (  7/801) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  8/801) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.005s] (  9/801) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.024s] ( 10/801) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.040s] ( 11/801) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.097s] ( 12/801) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.115s] ( 13/801) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.040s] ( 14/801) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.398s] ( 15/801) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.156s] ( 16/801) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.277s] ( 17/801) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.090s] ( 18/801) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.050s] ( 19/801) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.290s] ( 20/801) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.087s] ( 21/801) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.093s] ( 22/801) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.067s] ( 23/801) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.054s] ( 24/801) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.067s] ( 25/801) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.030s] ( 26/801) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.046s] ( 27/801) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.120s] ( 28/801) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/801) atelier
```

