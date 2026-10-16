---
created_at: "2026-10-16T20:05:29.096112637+00:00"
id: "atelier-2w0x"
evidence_type: "test"
commit: "29fdebc259e5f48de222618eaa2591442695e49f"
result: "pass"
captured_at: "2026-10-16T20:05:07.463876390+00:00"
command: "cargo nextest run --workspace --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-1isk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1isk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace --features test-hooks"
updated_at: "2026-10-16T20:05:29.096406366+00:00"
---

## Summary

cargo nextest run --workspace --features test-hooks

## Command

```console
cargo nextest run --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 80324
Truncated: yes

```text
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/1isk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.65s
────────────
 Nextest run ID 72c901e5-edcc-4774-8340-92033c22eb84 with nextest profile: default
    Starting 641 tests across 10 binaries
        PASS [   0.011s] (  1/641) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.037s] (  2/641) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.059s] (  3/641) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.092s] (  4/641) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.107s] (  5/641) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.121s] (  6/641) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.110s] (  7/641) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.181s] (  8/641) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.104s] (  9/641) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.170s] ( 10/641) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.273s] ( 11/641) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.288s] ( 12/641) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.334s] ( 13/641) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.164s] ( 14/641) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.345s] ( 15/641) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.242s] ( 16/641) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.155s] ( 17/641) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.012s] ( 18/641) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.068s] ( 19/641) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.012s] ( 20/641) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.011s] ( 21/641) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 22/641) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 23/641) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.139s] ( 24/641) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.008s] ( 25/641) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.010s] ( 26/641) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.008s] ( 27/641) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
```

