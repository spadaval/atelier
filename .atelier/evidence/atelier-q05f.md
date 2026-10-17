---
created_at: "2026-10-17T07:13:21.315165611+00:00"
id: "atelier-q05f"
evidence_type: "test"
commit: "6fc6f12162cc0ec8e2dc6421a57712b1f3ee9ccf"
result: "pass"
captured_at: "2026-10-17T07:13:03.729169184+00:00"
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
title: "Full suite with test-hooks passes after reading merged-record claims at the merge commit (atelier-goom) and rewording the docs (atelier-liml)"
updated_at: "2026-10-17T07:13:21.315954421+00:00"
---

## Summary

Full suite with test-hooks passes after reading merged-record claims at the merge commit (atelier-goom) and rewording the docs (atelier-liml)

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

Bytes: 87575
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ydv0/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.32s
────────────
 Nextest run ID 9b35f15f-307b-44e0-800b-8300bf537469 with nextest profile: default
    Starting 698 tests across 10 binaries
        PASS [   0.008s] (  1/698) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.280s] (  2/698) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.317s] (  3/698) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.345s] (  4/698) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.359s] (  5/698) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.366s] (  6/698) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.437s] (  7/698) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.094s] (  8/698) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.449s] (  9/698) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.188s] ( 10/698) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.567s] ( 11/698) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.128s] ( 12/698) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.588s] ( 13/698) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.308s] ( 14/698) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.220s] ( 15/698) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.681s] ( 16/698) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.098s] ( 17/698) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.153s] ( 18/698) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.139s] ( 19/698) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.007s] ( 20/698) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.008s] ( 21/698) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.066s] ( 22/698) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.013s] ( 23/698) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.018s] ( 24/698) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.154s] ( 25/698) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.008s] ( 26/698) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.009s] ( 27/698) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.011s] ( 28/698) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.007s] ( 29/698) atel
```

