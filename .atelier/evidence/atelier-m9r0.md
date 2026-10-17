---
created_at: "2026-10-17T02:46:29.859337226+00:00"
id: "atelier-m9r0"
evidence_type: "test"
commit: "fe98a45f0457e6d14291d97f72127f1496790da2"
result: "pass"
captured_at: "2026-10-17T02:46:16.288219026+00:00"
command: "env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sdb0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sdb0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Failure-path suite with test-hooks passes at fe98a45f after review fixes (663 tests)"
updated_at: "2026-10-17T02:46:29.860246893+00:00"
---

## Summary

Failure-path suite with test-hooks passes at fe98a45f after review fixes (663 tests)

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

Bytes: 82932
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-1w8c/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.44s
────────────
 Nextest run ID 011b3d1c-aab7-489f-8227-6457b5eb5a0f with nextest profile: default
    Starting 663 tests across 10 binaries
        PASS [   0.008s] (  1/663) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.037s] (  2/663) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.042s] (  3/663) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.063s] (  4/663) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.064s] (  5/663) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.074s] (  6/663) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.116s] (  7/663) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.062s] (  8/663) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.155s] (  9/663) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.194s] ( 10/663) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.163s] ( 11/663) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.198s] ( 12/663) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.239s] ( 13/663) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.116s] ( 14/663) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.163s] ( 15/663) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.107s] ( 16/663) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.086s] ( 17/663) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.047s] ( 18/663) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.110s] ( 19/663) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.105s] ( 20/663) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 21/663) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.354s] ( 22/663) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.007s] ( 23/663) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.007s] ( 24/663) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.010s] ( 25/663) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.009s] ( 26/663) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.008s] ( 27/663) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 28/663) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.008s] ( 2
```

