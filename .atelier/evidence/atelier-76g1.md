---
created_at: "2026-10-16T18:21:37.140872640+00:00"
id: "atelier-76g1"
evidence_type: "test"
commit: "bc2dd3116681cf3d0dccbfd720e3b71eb23be738"
result: "fail"
captured_at: "2026-10-16T18:21:24.593078553+00:00"
command: "cargo nextest run --workspace"
exit_status: "100"
target:
  kind: "issue"
  id: "atelier-cafy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cafy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T18:21:37.141094964+00:00"
---

## Summary

cargo nextest run --workspace

## Command

```console
cargo nextest run --workspace
```

Exit status: 100

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 80282
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/cafy/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/cafy/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/cafy/crates/atelier-workflow)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/cafy/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.46s
────────────
 Nextest run ID 27f466c9-8376-4395-88e8-a16b642ec1da with nextest profile: default
    Starting 619 tests across 10 binaries
        PASS [   0.007s] (  1/619) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.025s] (  2/619) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.035s] (  3/619) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.059s] (  4/619) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.061s] (  5/619) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.068s] (  6/619) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.058s] (  7/619) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.106s] (  8/619) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.070s] (  9/619) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.106s] ( 10/619) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.156s] ( 11/619) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.178s] ( 12/619) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.194s] ( 13/619) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.134s] ( 14/619) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.105s] ( 15/619) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.095s] ( 16/619) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.044s] ( 17/619) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.007s] ( 18/619) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.085s] ( 19/619) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.007s] ( 20/619) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.011s] ( 21/619) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 22/619) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.263s] ( 23/619) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.007s] ( 24/619) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 25/619) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 26/619) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [
```

