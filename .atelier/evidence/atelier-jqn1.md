---
created_at: "2026-10-17T17:40:08.482199597+00:00"
id: "atelier-jqn1"
evidence_type: "test"
commit: "cc8bd55a9665bf7504da8bdb128b240bab442e8a"
result: "pass"
captured_at: "2026-10-17T17:39:37.192276103+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
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
title: "Workspace tests pass with test-hooks, including the decision note, ADR and supersession CLI tests"
updated_at: "2026-10-17T17:40:08.486190317+00:00"
---

## Summary

Workspace tests pass with test-hooks, including the decision note, ADR and supersession CLI tests

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

Bytes: 96353
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fip8/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.88s
────────────
 Nextest run ID e8c6ec46-74d6-40e5-8cfd-0d2a2201daa8 with nextest profile: default
    Starting 769 tests across 10 binaries
        PASS [   1.108s] (  1/769) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.110s] (  2/769) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   1.178s] (  3/769) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.076s] (  4/769) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.198s] (  5/769) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.022s] (  6/769) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.004s] (  7/769) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.047s] (  8/769) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.025s] (  9/769) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.041s] ( 10/769) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.095s] ( 11/769) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.116s] ( 12/769) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.047s] ( 13/769) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.098s] ( 14/769) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.419s] ( 15/769) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.259s] ( 16/769) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.090s] ( 17/769) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.053s] ( 18/769) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.069s] ( 19/769) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.238s] ( 20/769) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.094s] ( 21/769) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.067s] ( 22/769) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.054s] ( 23/769) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.069s] ( 24/769) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 25/769) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.044s] ( 26/769) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.102s] ( 27/769) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/769) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/769) atelier-app
```

