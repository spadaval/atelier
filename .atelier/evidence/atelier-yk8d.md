---
created_at: "2026-10-22T20:55:33.690883273+00:00"
id: "atelier-yk8d"
evidence_type: "test"
commit: "2c18ab441b211bbe673db2fa821112da5c50d42e"
result: "pass"
captured_at: "2026-10-22T20:54:59.318633276+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-p189"
target:
  kind: "issue"
  id: "atelier-p189"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p189"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes after review fixes"
updated_at: "2026-10-22T20:55:33.691148547+00:00"
---

## Summary

Full workspace suite with test-hooks passes after review fixes

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

Bytes: 103859
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-c3kh/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.09s
────────────
 Nextest run ID 0e4d62dd-d420-46c1-88f3-57ebd6007a44 with nextest profile: default
    Starting 828 tests across 10 binaries
        PASS [   0.082s] (  1/828) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.196s] (  2/828) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.220s] (  3/828) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.068s] (  4/828) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.120s] (  5/828) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.333s] (  6/828) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.009s] (  7/828) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.007s] (  8/828) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.037s] (  9/828) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.038s] ( 10/828) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.065s] ( 11/828) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.146s] ( 12/828) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.177s] ( 13/828) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.063s] ( 14/828) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.602s] ( 15/828) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.131s] ( 16/828) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.384s] ( 17/828) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.163s] ( 18/828) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.096s] ( 19/828) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.410s] ( 20/828) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.132s] ( 21/828) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.145s] ( 22/828) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.104s] ( 23/828) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.076s] ( 24/828) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.100s] ( 25/828) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.047s] ( 26/828) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.205s] ( 27/828) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.385s] ( 28/828) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.016s] ( 29/828) atelier
```

