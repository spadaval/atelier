---
created_at: "2026-10-16T20:42:13.515525146+00:00"
id: "atelier-bp1i"
evidence_type: "test"
commit: "3f3211cbc313db4f54e69b6cbca797a6a15c98a6"
result: "pass"
captured_at: "2026-10-16T20:41:22.516889581+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-hyew"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-hyew"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T20:42:13.516151266+00:00"
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

Bytes: 81485
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/hyew/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/hyew/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.59s
────────────
 Nextest run ID 84abde54-0262-4667-8777-d2c0225c8180 with nextest profile: default
    Starting 643 tests across 10 binaries
        PASS [   0.015s] (  1/643) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.058s] (  2/643) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.071s] (  3/643) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.118s] (  4/643) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.131s] (  5/643) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.139s] (  6/643) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.118s] (  7/643) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.785s] (  8/643) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   1.385s] (  9/643) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.469s] ( 10/643) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.497s] ( 11/643) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   1.548s] ( 12/643) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   1.618s] ( 13/643) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   1.810s] ( 14/643) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.384s] ( 15/643) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   1.345s] ( 16/643) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   2.255s] ( 17/643) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.006s] ( 18/643) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 19/643) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.009s] ( 20/643) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.006s] ( 21/643) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.006s] ( 22/643) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.005s] ( 23/643) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.005s] ( 24/643) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.005s] ( 25/643) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.007s] ( 26/643) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
        PASS [   0.008s] ( 27/643) atelier-app mission_overview::tests::complete_summaries_keep_counts_beyond_display_limits_and_stable_status_priority_order
        PASS [   0.007s] ( 28/643) atelier-app mission_ove
```

