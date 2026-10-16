---
created_at: "2026-10-16T20:26:17.807190436+00:00"
id: "atelier-zqoz"
evidence_type: "test"
commit: "cb721cd381ecbb460d463094f390d6aee3db1dbe"
result: "pass"
captured_at: "2026-10-16T20:25:59.586316715+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-x3bj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-x3bj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T20:26:17.807427468+00:00"
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

Bytes: 80052
Truncated: yes

```text
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/x3bj/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/x3bj/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.19s
────────────
 Nextest run ID afa67140-93da-4b09-994a-ac6d0a33006d with nextest profile: default
    Starting 640 tests across 10 binaries
        PASS [   0.007s] (  1/640) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.060s] (  2/640) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.099s] (  3/640) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.125s] (  4/640) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.149s] (  5/640) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.186s] (  6/640) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.140s] (  7/640) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.225s] (  8/640) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.271s] (  9/640) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.311s] ( 10/640) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.655s] ( 11/640) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.801s] ( 12/640) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.857s] ( 13/640) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   1.058s] ( 14/640) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   1.193s] ( 15/640) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   1.312s] ( 16/640) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.313s] ( 17/640) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.672s] ( 18/640) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.017s] ( 19/640) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.016s] ( 20/640) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.011s] ( 21/640) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 22/640) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.011s] ( 23/640) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.009s] ( 24/640) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.013s] ( 25/640) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.872s] ( 26/640) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.010s] ( 27/640) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.009s] ( 28/640) atelier-app issue_read::tests::objective_scope_keeps_all_rows_be
```

