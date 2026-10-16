---
created_at: "2026-10-16T22:00:49.822112008+00:00"
id: "atelier-0do3"
evidence_type: "test"
commit: "51821bb620821dcacb8314185b98dbcad7ae90f2"
result: "pass"
captured_at: "2026-10-16T22:00:38.328838237+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-tqru"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tqru"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T22:00:49.822817513+00:00"
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

Bytes: 80735
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/tqru/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.33s
────────────
 Nextest run ID 62f13b73-c376-4286-b7f7-eb493acb194f with nextest profile: default
    Starting 646 tests across 10 binaries
        PASS [   0.010s] (  1/646) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.042s] (  2/646) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.044s] (  3/646) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.064s] (  4/646) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.066s] (  5/646) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.086s] (  6/646) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.064s] (  7/646) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.130s] (  8/646) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.080s] (  9/646) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.170s] ( 10/646) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.138s] ( 11/646) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.185s] ( 12/646) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.150s] ( 13/646) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.217s] ( 14/646) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.114s] ( 15/646) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.049s] ( 16/646) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.089s] ( 17/646) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.007s] ( 18/646) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.008s] ( 19/646) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.111s] ( 20/646) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.010s] ( 21/646) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 22/646) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 23/646) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 24/646) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.008s] ( 25/646) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.007s] ( 26/646) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.127s] ( 27/646) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.069s] ( 28/646) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.154s] ( 29/646) atelier-ap
```

