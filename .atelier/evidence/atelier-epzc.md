---
created_at: "2026-10-16T20:32:14.731132817+00:00"
id: "atelier-epzc"
evidence_type: "test"
commit: "2a91eba14106ed6289933ca426b1f2fc8f71133b"
result: "pass"
captured_at: "2026-10-16T20:31:40.828438045+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7hh4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7hh4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T20:32:14.731583258+00:00"
---

## Summary

cargo nextest run --workspace

## Command

```console
cargo nextest run --workspace
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 80912
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/7hh4/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/7hh4/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.42s
────────────
 Nextest run ID 48a31013-77a0-4874-b491-ec80b1f29a89 with nextest profile: default
    Starting 642 tests across 10 binaries
        PASS [   0.007s] (  1/642) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   1.346s] (  2/642) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   2.926s] (  3/642) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   2.938s] (  4/642) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   2.954s] (  5/642) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.147s] (  6/642) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   3.419s] (  7/642) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   3.186s] (  8/642) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   5.121s] (  9/642) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   6.458s] ( 10/642) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   4.209s] ( 11/642) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   4.359s] ( 12/642) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   7.923s] ( 13/642) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   4.959s] ( 14/642) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   9.523s] ( 15/642) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   7.166s] ( 16/642) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   2.182s] ( 17/642) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.007s] ( 18/642) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.006s] ( 19/642) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   2.974s] ( 20/642) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.021s] ( 21/642) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 22/642) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 23/642) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 24/642) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.009s] ( 25/642) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 26/642) atelier-app issue_
```

