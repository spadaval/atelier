---
created_at: "2026-10-17T07:47:02.215111449+00:00"
id: "atelier-39xn"
evidence_type: "test"
commit: "5927bc001a2354c3e2a5e1c54f0953843e78bc94"
result: "pass"
captured_at: "2026-10-17T07:46:11.736325170+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-krf4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-krf4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes, including the count-phrasing tests, the parent scope size test and the updated rendering tests"
updated_at: "2026-10-17T07:47:02.216475726+00:00"
---

## Summary

Full workspace nextest suite passes, including the count-phrasing tests, the parent scope size test and the updated rendering tests

## Command

```console
cargo nextest run -j 4
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 89248
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-krf4/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.55s
────────────
 Nextest run ID 5534144e-9ed7-42de-bb67-659cbc18219b with nextest profile: default
    Starting 713 tests across 10 binaries
        PASS [   0.270s] (  1/713) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.668s] (  2/713) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.802s] (  3/713) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.833s] (  4/713) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.246s] (  5/713) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.417s] (  6/713) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.016s] (  7/713) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.090s] (  8/713) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.185s] (  9/713) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.213s] ( 10/713) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.347s] ( 11/713) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.381s] ( 12/713) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.209s] ( 13/713) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.787s] ( 14/713) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.245s] ( 15/713) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.684s] ( 16/713) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.206s] ( 17/713) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.095s] ( 18/713) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.368s] ( 19/713) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.116s] ( 20/713) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.128s] ( 21/713) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.067s] ( 22/713) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.051s] ( 23/713) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.029s] ( 24/713) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.068s] ( 25/713) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   1.858s] ( 26/713) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.046s] ( 27/713) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.005s] ( 28/713) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/713) atelier-
```

