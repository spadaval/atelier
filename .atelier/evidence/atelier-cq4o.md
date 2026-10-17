---
created_at: "2026-10-17T07:10:17.167736489+00:00"
id: "atelier-cq4o"
evidence_type: "test"
commit: "ae6c48d60d7a6a0352616dfac4a6f21fd99cfd13"
result: "pass"
captured_at: "2026-10-17T07:09:41.879262071+00:00"
command: "cargo nextest run"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-qrwr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qrwr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes (689 tests), including the new record_refs CLI tests for short IDs, not-found, wrong-kind, invalid values and flag conflicts"
updated_at: "2026-10-17T07:10:17.168805240+00:00"
---

## Summary

Full workspace suite passes (689 tests), including the new record_refs CLI tests for short IDs, not-found, wrong-kind, invalid values and flag conflicts

## Command

```console
cargo nextest run
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 86532
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-qrwr/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.44s
────────────
 Nextest run ID baf1510c-1820-49a0-80ae-d36b2b8116f1 with nextest profile: default
    Starting 689 tests across 10 binaries
        PASS [   0.007s] (  1/689) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.079s] (  2/689) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.086s] (  3/689) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.119s] (  4/689) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.120s] (  5/689) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.145s] (  6/689) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.422s] (  7/689) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.415s] (  8/689) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.306s] (  9/689) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.439s] ( 10/689) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.431s] ( 11/689) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.551s] ( 12/689) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.555s] ( 13/689) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.171s] ( 14/689) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.595s] ( 15/689) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.203s] ( 16/689) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.052s] ( 17/689) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.100s] ( 18/689) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.150s] ( 19/689) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.008s] ( 20/689) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.127s] ( 21/689) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.008s] ( 22/689) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.010s] ( 23/689) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 24/689) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 25/689) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.010s] ( 26/689) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.008s] ( 27/689) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.114s] ( 28/689) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 29/689) atel
```

