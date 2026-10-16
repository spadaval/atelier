---
created_at: "2026-10-16T19:55:41.129925088+00:00"
id: "atelier-dvrg"
evidence_type: "test"
commit: "cc23cfedeb74f7b7d902e1d4fab447ecee00e0b7"
result: "pass"
captured_at: "2026-10-16T19:55:20.996857349+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-v9ve"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v9ve"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T19:55:41.130160287+00:00"
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

Bytes: 79823
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/v9ve/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/v9ve/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/v9ve/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/v9ve/crates/atelier-core)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/v9ve/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.36s
────────────
 Nextest run ID 3a4ea3ba-414d-4533-aafc-d6895793563e with nextest profile: default
    Starting 636 tests across 10 binaries
        PASS [   0.009s] (  1/636) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.049s] (  2/636) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.067s] (  3/636) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.090s] (  4/636) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.099s] (  5/636) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.111s] (  6/636) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.072s] (  7/636) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.182s] (  8/636) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.153s] (  9/636) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.112s] ( 10/636) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.239s] ( 11/636) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.290s] ( 12/636) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.203s] ( 13/636) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.309s] ( 14/636) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.329s] ( 15/636) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.169s] ( 16/636) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.147s] ( 17/636) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.069s] ( 18/636) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.132s] ( 19/636) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.010s] ( 20/636) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.009s] ( 21/636) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.009s] ( 22/636) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.009s] ( 23/636) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.012s] ( 24/636) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 25/636) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.009s] ( 26/636) atelier-
```

