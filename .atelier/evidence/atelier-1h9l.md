---
created_at: "2026-10-16T22:05:27.033125953+00:00"
id: "atelier-1h9l"
evidence_type: "test"
commit: "9725aa88b7e8579884e7ebb09bd5836e08c2f68e"
result: "pass"
captured_at: "2026-10-16T22:04:49.146242030+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-v5tt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v5tt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T22:05:27.034173977+00:00"
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

Bytes: 80852
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/v5tt/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.19s
────────────
 Nextest run ID c02be692-a776-46f6-8a81-b43b64a7c07b with nextest profile: default
    Starting 647 tests across 10 binaries
        PASS [   0.017s] (  1/647) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.090s] (  2/647) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.109s] (  3/647) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.140s] (  4/647) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.144s] (  5/647) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.169s] (  6/647) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.163s] (  7/647) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.345s] (  8/647) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.417s] (  9/647) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.332s] ( 10/647) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.504s] ( 11/647) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.562s] ( 12/647) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.269s] ( 13/647) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.544s] ( 14/647) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.636s] ( 15/647) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.500s] ( 16/647) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.204s] ( 17/647) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.398s] ( 18/647) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.298s] ( 19/647) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.371s] ( 20/647) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.032s] ( 21/647) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.023s] ( 22/647) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.023s] ( 23/647) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.038s] ( 24/647) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.029s] ( 25/647) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.043s] ( 26/647) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.023s] ( 27/647) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.402s] ( 28/647) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.021s] ( 29/647) atelier-app
```

