---
created_at: "2026-10-17T22:10:41.901411517+00:00"
id: "atelier-xc5e"
evidence_type: "test"
commit: "b338068ca4008bb43d9c63f348cfd7742ade61ba"
result: "pass"
captured_at: "2026-10-17T22:10:14.523873137+00:00"
command: "cargo nextest run -j 4 --workspace"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-nexd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nexd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass without features after labelling mentions by summary"
updated_at: "2026-10-17T22:10:41.902169951+00:00"
---

## Summary

Workspace tests pass without features after labelling mentions by summary

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

Bytes: 101257
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-nexd/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.73s
────────────
 Nextest run ID 77077d3f-2674-4792-8a2f-88dc1c8a431a with nextest profile: default
    Starting 809 tests across 10 binaries
        PASS [   0.054s] (  1/809) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.129s] (  2/809) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.148s] (  3/809) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.046s] (  4/809) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.083s] (  5/809) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.024s] (  6/809) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/809) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.244s] (  8/809) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  9/809) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.027s] ( 10/809) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.044s] ( 11/809) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.109s] ( 12/809) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.128s] ( 13/809) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.043s] ( 14/809) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.441s] ( 15/809) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.095s] ( 16/809) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.257s] ( 17/809) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.100s] ( 18/809) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.055s] ( 19/809) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.256s] ( 20/809) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.074s] ( 21/809) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.075s] ( 22/809) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.103s] ( 23/809) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.061s] ( 24/809) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.076s] ( 25/809) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.033s] ( 26/809) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.048s] ( 27/809) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.110s] ( 28/809) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/809) atelier
```

