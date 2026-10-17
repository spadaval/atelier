---
created_at: "2026-10-17T02:36:03.570079243+00:00"
id: "atelier-tw22"
evidence_type: "test"
commit: "2edc95acbef61e3993bd6ab24766f3d7c6eaee22"
result: "pass"
captured_at: "2026-10-17T02:35:52.253473574+00:00"
command: "env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run --features test-hooks"
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
title: "Failure-path suite (test-hooks) passes"
updated_at: "2026-10-17T02:36:03.570748120+00:00"
---

## Summary

Failure-path suite (test-hooks) passes

## Command

```console
env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 82797
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-7u4o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.29s
────────────
 Nextest run ID a52a7b51-f4d3-42af-824e-4b583ce66034 with nextest profile: default
    Starting 662 tests across 10 binaries
        PASS [   0.008s] (  1/662) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.038s] (  2/662) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.038s] (  3/662) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.077s] (  4/662) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.084s] (  5/662) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.096s] (  6/662) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.148s] (  7/662) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.083s] (  8/662) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.200s] (  9/662) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.241s] ( 10/662) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.103s] ( 11/662) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.179s] ( 12/662) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.265s] ( 13/662) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.236s] ( 14/662) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.173s] ( 15/662) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.090s] ( 16/662) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.118s] ( 17/662) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.108s] ( 18/662) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.054s] ( 19/662) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.379s] ( 20/662) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.007s] ( 21/662) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 22/662) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.115s] ( 23/662) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 24/662) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.013s] ( 25/662) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 26/662) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.008s] ( 27/662) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 28/662) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.008s] ( 29/662) ate
```

