---
created_at: "2026-10-17T19:23:29.458488510+00:00"
id: "atelier-i1t1"
evidence_type: "test"
commit: "3df0cdd82f5732dddf96243f89c73de69ee7bf57"
result: "pass"
captured_at: "2026-10-17T19:22:43.930110488+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-rx22"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rx22"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass with test-hooks, including the debrief fixture tests of both forms and the facts-match test"
updated_at: "2026-10-17T19:23:29.459287240+00:00"
---

## Summary

Workspace tests pass with test-hooks, including the debrief fixture tests of both forms and the facts-match test

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

Bytes: 100073
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-rx22/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.95s
────────────
 Nextest run ID 05642b42-d2b7-4ab0-b478-a9651673678d with nextest profile: default
    Starting 799 tests across 10 binaries
        PASS [   0.093s] (  1/799) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.245s] (  2/799) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.277s] (  3/799) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.071s] (  4/799) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.132s] (  5/799) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.393s] (  6/799) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.007s] (  7/799) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.008s] (  8/799) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.043s] (  9/799) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.047s] ( 10/799) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.067s] ( 11/799) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.161s] ( 12/799) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.194s] ( 13/799) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.064s] ( 14/799) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.660s] ( 15/799) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.163s] ( 16/799) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.409s] ( 17/799) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.151s] ( 18/799) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.094s] ( 19/799) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.394s] ( 20/799) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.115s] ( 21/799) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.151s] ( 22/799) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.112s] ( 23/799) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.092s] ( 24/799) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.057s] ( 25/799) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.128s] ( 26/799) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.080s] ( 27/799) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.189s] ( 28/799) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.009s] ( 29/799) atelier
```

