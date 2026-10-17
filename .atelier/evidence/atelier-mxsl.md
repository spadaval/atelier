---
created_at: "2026-10-17T03:15:12.230019360+00:00"
id: "atelier-mxsl"
evidence_type: "test"
commit: "860911721eedd8fe26f5472db0e1c06e8e8209bc"
result: "pass"
captured_at: "2026-10-17T03:14:50.768086422+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ihns"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ihns"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Failure-path suite (test-hooks) passes at the final code commit"
updated_at: "2026-10-17T03:15:12.230743039+00:00"
---

## Summary

Failure-path suite (test-hooks) passes at the final code commit

## Command

```console
cargo nextest run --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 84046
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-ihns/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.50s
────────────
 Nextest run ID 7b04a025-27f8-4340-91e0-874f95337e29 with nextest profile: default
    Starting 671 tests across 10 binaries
        PASS [   0.011s] (  1/671) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.405s] (  2/671) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.817s] (  3/671) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.870s] (  4/671) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.059s] (  5/671) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   1.525s] (  6/671) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   2.130s] (  7/671) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   2.558s] (  8/671) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   2.410s] (  9/671) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   3.293s] ( 10/671) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   3.449s] ( 11/671) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   3.854s] ( 12/671) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.744s] ( 13/671) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   2.808s] ( 14/671) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   1.465s] ( 15/671) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   5.012s] ( 16/671) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   1.179s] ( 17/671) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   1.764s] ( 18/671) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   1.047s] ( 19/671) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.004s] ( 20/671) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.006s] ( 21/671) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.008s] ( 22/671) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.006s] ( 23/671) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.005s] ( 24/671) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.005s] ( 25/671) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.005s] ( 26/671) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.005s] ( 27/671) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.004s] ( 28/671) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
        PASS [   0.009s] ( 29/671) at
```

