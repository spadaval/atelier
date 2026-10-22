---
created_at: "2026-10-22T22:23:06.793392551+00:00"
id: "atelier-aai4"
evidence_type: "test"
commit: "0e27144c12e18dc3481177f476bd83b8b74d3501"
result: "pass"
captured_at: "2026-10-22T22:22:28.203526269+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-s2ei"
target:
  kind: "issue"
  id: "atelier-s2ei"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-s2ei"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes (866/866) with test-hooks"
updated_at: "2026-10-22T22:23:06.793759304+00:00"
---

## Summary

Full workspace suite passes (866/866) with test-hooks

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

Bytes: 109003
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-s2ei/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.48s
────────────
 Nextest run ID e36770b7-e2b0-4381-ae82-eb9494b6dcab with nextest profile: default
    Starting 866 tests across 10 binaries
        PASS [   0.046s] (  1/866) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.117s] (  2/866) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.131s] (  3/866) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.172s] (  4/866) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.042s] (  5/866) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.017s] (  6/866) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.076s] (  7/866) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.005s] (  8/866) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.005s] (  9/866) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.024s] ( 10/866) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.039s] ( 11/866) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.099s] ( 12/866) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.116s] ( 13/866) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.038s] ( 14/866) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.373s] ( 15/866) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.073s] ( 16/866) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.236s] ( 17/866) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.097s] ( 18/866) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.049s] ( 19/866) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.220s] ( 20/866) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.069s] ( 21/866) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.093s] ( 22/866) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.068s] ( 23/866) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.056s] ( 24/866) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.069s] ( 25/866) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.031s] ( 26/866) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.043s] ( 27/866) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.096s] ( 28/866) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/866) atelier
```

