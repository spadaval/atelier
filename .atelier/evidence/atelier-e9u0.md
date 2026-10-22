---
created_at: "2026-10-22T21:56:20.161489025+00:00"
id: "atelier-e9u0"
evidence_type: "test"
commit: "699b0732ca32364cafcb65829dc50fa2cab9b40f"
result: "pass"
captured_at: "2026-10-22T21:55:49.044964211+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-n642-uw4x"
target:
  kind: "issue"
  id: "atelier-n642"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n642"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test hooks passes on the merge of master (861/861)"
updated_at: "2026-10-22T21:56:20.162170224+00:00"
---

## Summary

Full workspace suite with test hooks passes on the merge of master (861/861)

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

Bytes: 108117
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1ztv/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s
────────────
 Nextest run ID 81d7cfe1-5da5-4736-9235-9eac8f09d6df with nextest profile: default
    Starting 861 tests across 10 binaries
        PASS [   0.049s] (  1/861) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.123s] (  2/861) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.139s] (  3/861) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.042s] (  4/861) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.078s] (  5/861) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.023s] (  6/861) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.004s] (  7/861) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.229s] (  8/861) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.004s] (  9/861) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.026s] ( 10/861) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.041s] ( 11/861) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.098s] ( 12/861) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.119s] ( 13/861) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.040s] ( 14/861) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.403s] ( 15/861) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.089s] ( 16/861) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.237s] ( 17/861) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.092s] ( 18/861) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.051s] ( 19/861) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.239s] ( 20/861) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.072s] ( 21/861) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.070s] ( 22/861) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.095s] ( 23/861) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.054s] ( 24/861) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.068s] ( 25/861) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 26/861) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.045s] ( 27/861) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.101s] ( 28/861) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/861) atelier
```

