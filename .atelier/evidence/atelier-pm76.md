---
created_at: "2026-10-15T23:00:55.566033600+00:00"
id: "atelier-pm76"
evidence_type: "test"
commit: "02fb56812ce87e436330ef557d2192798fc72fc8"
result: "pass"
captured_at: "2026-10-15T23:00:45.091094143+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-fxbg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fxbg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after review fixes"
updated_at: "2026-10-15T23:00:55.566891889+00:00"
---

## Summary

Full workspace nextest suite passes after review fixes

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

Bytes: 73613
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.20s
────────────
 Nextest run ID a7e90e2f-e229-4ff5-8662-d5480c8bea7e with nextest profile: default
    Starting 590 tests across 10 binaries
        PASS [   0.026s] (  1/590) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.035s] (  2/590) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.058s] (  3/590) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.064s] (  4/590) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.086s] (  5/590) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.108s] (  6/590) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.117s] (  7/590) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.117s] (  8/590) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.127s] (  9/590) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.185s] ( 10/590) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.130s] ( 11/590) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.262s] ( 12/590) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.294s] ( 13/590) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.296s] ( 14/590) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.060s] ( 15/590) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.007s] ( 16/590) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.123s] ( 17/590) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.010s] ( 18/590) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 19/590) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 20/590) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 21/590) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.131s] ( 22/590) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.006s] ( 23/590) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.007s] ( 24/590) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
        PASS [   0.009s] ( 25/590) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.009s] ( 26/590) atelier-app mission_overview::tests::complete_summaries_keep_counts_beyond_display_limits_and_stable_status_priority_order
        PASS [   0.011s] ( 27/590) atelier-app mission_overview::tests::directed_membership_deduplicates_roots_and_preserves_shared_scope_and_blockers
        PASS [   0.067s] ( 28/590) atelier-app git_freshness::tests::wo
```

