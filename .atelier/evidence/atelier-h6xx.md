---
created_at: "2026-10-17T17:40:45.473079675+00:00"
id: "atelier-h6xx"
evidence_type: "test"
commit: "cc8bd55a9665bf7504da8bdb128b240bab442e8a"
result: "pass"
captured_at: "2026-10-17T17:40:08.743627115+00:00"
command: "cargo nextest run -j 4 --workspace"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-fip8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fip8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass without features"
updated_at: "2026-10-17T17:40:45.474502002+00:00"
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

Bytes: 95748
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fip8/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.78s
────────────
 Nextest run ID 972c5e7e-4d36-4294-90c3-465721e06a39 with nextest profile: default
    Starting 765 tests across 10 binaries
        PASS [   0.048s] (  1/765) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.121s] (  2/765) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.153s] (  3/765) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.042s] (  4/765) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.092s] (  5/765) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.223s] (  6/765) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  7/765) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.022s] (  8/765) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.026s] (  9/765) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.039s] ( 10/765) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.098s] ( 11/765) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.118s] ( 12/765) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.040s] ( 13/765) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.411s] ( 14/765) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.123s] ( 15/765) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.272s] ( 16/765) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.097s] ( 17/765) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.053s] ( 18/765) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.264s] ( 19/765) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.079s] ( 20/765) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.094s] ( 21/765) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.068s] ( 22/765) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.053s] ( 23/765) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.070s] ( 24/765) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.030s] ( 25/765) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.052s] ( 26/765) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.107s] ( 27/765) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/765) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/765) atelier-app
```

