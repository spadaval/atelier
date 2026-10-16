---
created_at: "2026-10-16T18:34:29.197027436+00:00"
id: "atelier-719c"
evidence_type: "test"
commit: "455db2c63bd4093a4c5effde6127f3e522977f7c"
result: "pass"
captured_at: "2026-10-16T18:34:09.942959951+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-n7g1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n7g1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T18:34:29.197279526+00:00"
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

Bytes: 77689
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/n7g1/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/n7g1/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.96s
────────────
 Nextest run ID adbcf2bc-d452-453b-a106-66b07b370cf7 with nextest profile: default
    Starting 621 tests across 10 binaries
        PASS [   0.008s] (  1/621) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.048s] (  2/621) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.073s] (  3/621) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.115s] (  4/621) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.127s] (  5/621) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.131s] (  6/621) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.128s] (  7/621) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.233s] (  8/621) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.202s] (  9/621) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.157s] ( 10/621) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.366s] ( 11/621) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.373s] ( 12/621) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.396s] ( 13/621) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.441s] ( 14/621) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.325s] ( 15/621) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.200s] ( 16/621) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.270s] ( 17/621) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.114s] ( 18/621) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.012s] ( 19/621) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.019s] ( 20/621) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.029s] ( 21/621) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.018s] ( 22/621) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.020s] ( 23/621) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.022s] ( 24/621) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.015s] ( 25/621) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.224s] ( 26/621) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.021s] ( 27/621) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.026s] ( 28/621) atelier-app issue_read::tests::objective_scope_keeps_all_rows_be
```

