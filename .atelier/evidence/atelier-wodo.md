---
created_at: "2026-10-17T19:57:11.934447937+00:00"
id: "atelier-wodo"
evidence_type: "test"
commit: "8b0890bf8b0afcece4d577e89e3cd294eef81618"
result: "pass"
captured_at: "2026-10-17T19:56:36.949161916+00:00"
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
title: "Workspace tests pass without features after the review fixes"
updated_at: "2026-10-17T19:57:11.935191905+00:00"
---

## Summary

Workspace tests pass without features after the review fixes

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

Bytes: 100357
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-acrt/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.55s
────────────
 Nextest run ID 1a88aa5b-d48b-480d-9bb3-ffbf8262a9de with nextest profile: default
    Starting 802 tests across 10 binaries
        PASS [   0.063s] (  1/802) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.144s] (  2/802) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.170s] (  3/802) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.048s] (  4/802) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.089s] (  5/802) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.025s] (  6/802) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/802) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.006s] (  8/802) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.284s] (  9/802) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.032s] ( 10/802) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.048s] ( 11/802) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.127s] ( 12/802) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.139s] ( 13/802) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.048s] ( 14/802) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.507s] ( 15/802) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.107s] ( 16/802) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.286s] ( 17/802) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.106s] ( 18/802) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.058s] ( 19/802) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.267s] ( 20/802) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.073s] ( 21/802) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.074s] ( 22/802) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.113s] ( 23/802) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.058s] ( 24/802) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.035s] ( 25/802) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.074s] ( 26/802) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.049s] ( 27/802) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.110s] ( 28/802) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/802) atelier
```

