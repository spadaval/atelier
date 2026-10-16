---
created_at: "2026-10-16T19:06:53.985955841+00:00"
id: "atelier-8bk1"
evidence_type: "test"
commit: "f83875561add0bfd47cf2e19e15b88198e515e5d"
result: "pass"
captured_at: "2026-10-16T19:06:41.599941397+00:00"
command: "cargo nextest run --workspace --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-lq42"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-lq42"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace --features test-hooks"
updated_at: "2026-10-16T19:06:53.986259307+00:00"
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

Bytes: 79174
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/lq42/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.46s
────────────
 Nextest run ID bc4ddb99-c085-4017-b683-7b764a66b72a with nextest profile: default
    Starting 633 tests across 10 binaries
        PASS [   0.008s] (  1/633) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.023s] (  2/633) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.049s] (  3/633) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.079s] (  4/633) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.057s] (  5/633) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.082s] (  6/633) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.091s] (  7/633) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.146s] (  8/633) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.087s] (  9/633) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.122s] ( 10/633) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.183s] ( 11/633) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.231s] ( 12/633) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.231s] ( 13/633) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.166s] ( 14/633) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.123s] ( 15/633) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.273s] ( 16/633) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.109s] ( 17/633) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.051s] ( 18/633) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.006s] ( 19/633) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 20/633) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.105s] ( 21/633) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.007s] ( 22/633) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.012s] ( 23/633) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 24/633) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.008s] ( 25/633) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.008s] ( 26/633) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.008s] ( 27/633) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.009s] ( 28/633) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
        PASS [   0.012s] ( 29/633) at
```

