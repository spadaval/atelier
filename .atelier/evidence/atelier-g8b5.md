---
created_at: "2026-10-17T02:59:41.385651386+00:00"
id: "atelier-g8b5"
evidence_type: "test"
commit: "dee0d2478d836bef878bd5cc36a7ea32cc695385"
result: "pass"
captured_at: "2026-10-17T02:59:22.969134893+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7u4o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7u4o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Failure-path suite (test-hooks) after merging master"
updated_at: "2026-10-17T02:59:41.386835814+00:00"
---

## Summary

Failure-path suite (test-hooks) after merging master

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

Bytes: 83639
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-7u4o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.65s
────────────
 Nextest run ID cc8b5150-201e-4253-870a-d3dc7612922d with nextest profile: default
    Starting 668 tests across 10 binaries
        PASS [   0.008s] (  1/668) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.037s] (  2/668) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.046s] (  3/668) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.068s] (  4/668) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.077s] (  5/668) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.089s] (  6/668) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.135s] (  7/668) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.072s] (  8/668) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.162s] (  9/668) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.208s] ( 10/668) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.087s] ( 11/668) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.155s] ( 12/668) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.232s] ( 13/668) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.197s] ( 14/668) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.149s] ( 15/668) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.085s] ( 16/668) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.113s] ( 17/668) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.106s] ( 18/668) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.057s] ( 19/668) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.349s] ( 20/668) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.007s] ( 21/668) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.008s] ( 22/668) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.011s] ( 23/668) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.126s] ( 24/668) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 25/668) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 26/668) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 27/668) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.015s] ( 28/668) atelier-app issue_read::tests::evidence_gate_counts_validating_evid
```

