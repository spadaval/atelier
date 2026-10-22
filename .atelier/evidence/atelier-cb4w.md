---
created_at: "2026-10-22T20:43:13.740353588+00:00"
id: "atelier-cb4w"
evidence_type: "test"
commit: "702c76d7df5f9bad337bab4661c077b46f63efff"
result: "pass"
captured_at: "2026-10-22T20:42:47.123998942+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-rcnk"
target:
  kind: "issue"
  id: "atelier-rcnk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rcnk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes"
updated_at: "2026-10-22T20:43:13.740691288+00:00"
---

## Summary

Full workspace suite with test-hooks passes

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

Bytes: 102975
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-c3kh/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.92s
────────────
 Nextest run ID 94afeae2-532a-43e4-aef9-93d3b6e7c6ea with nextest profile: default
    Starting 821 tests across 10 binaries
        PASS [   0.050s] (  1/821) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.125s] (  2/821) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.140s] (  3/821) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.043s] (  4/821) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.077s] (  5/821) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.024s] (  6/821) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.227s] (  7/821) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  8/821) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.004s] (  9/821) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.025s] ( 10/821) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.042s] ( 11/821) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.101s] ( 12/821) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.119s] ( 13/821) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.042s] ( 14/821) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.405s] ( 15/821) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.090s] ( 16/821) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.244s] ( 17/821) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.093s] ( 18/821) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.054s] ( 19/821) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.241s] ( 20/821) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.072s] ( 21/821) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.097s] ( 22/821) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.071s] ( 23/821) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.055s] ( 24/821) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.072s] ( 25/821) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 26/821) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.046s] ( 27/821) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.103s] ( 28/821) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/821) atelier
```

