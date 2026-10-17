---
created_at: "2026-10-17T06:26:21.398067248+00:00"
id: "atelier-1x10"
evidence_type: "test"
commit: "ecdb107c7b86740ce87b65206bbde79f89d31f0f"
result: "pass"
captured_at: "2026-10-17T06:25:56.116743072+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ydv0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ydv0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full suite passes with test hooks at the R1/R2 fixes: merge_commit gates read the merged records, refuse base-side records about the item (merge_base_records), and require a shell.check to prove the merged tree"
updated_at: "2026-10-17T06:26:21.398865858+00:00"
---

## Summary

Full suite passes with test hooks at the R1/R2 fixes: merge_commit gates read the merged records, refuse base-side records about the item (merge_base_records), and require a shell.check to prove the merged tree

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

Bytes: 85791
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ydv0/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
────────────
 Nextest run ID 4dd4003d-aa42-4ae6-a3e2-2c0fd04f7961 with nextest profile: default
    Starting 684 tests across 10 binaries
        PASS [   0.015s] (  1/684) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.053s] (  2/684) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.053s] (  3/684) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.087s] (  4/684) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.092s] (  5/684) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.111s] (  6/684) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.192s] (  7/684) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.121s] (  8/684) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.210s] (  9/684) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.344s] ( 10/684) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.160s] ( 11/684) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.287s] ( 12/684) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.334s] ( 13/684) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.398s] ( 14/684) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.432s] ( 15/684) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.260s] ( 16/684) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.111s] ( 17/684) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.076s] ( 18/684) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.183s] ( 19/684) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.015s] ( 20/684) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.012s] ( 21/684) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.162s] ( 22/684) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.186s] ( 23/684) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.017s] ( 24/684) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.017s] ( 25/684) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.012s] ( 26/684) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.019s] ( 27/684) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.009s] ( 28/684) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.096s] ( 29/684) atel
```

