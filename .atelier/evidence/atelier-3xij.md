---
created_at: "2026-10-16T19:18:30.879778309+00:00"
id: "atelier-3xij"
evidence_type: "test"
commit: "9405ede15002beee7d8c61e6f29c1d8f3ba20fd9"
result: "pass"
captured_at: "2026-10-16T19:18:15.376535762+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-kqt9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kqt9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T19:18:30.880004341+00:00"
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

Bytes: 79203
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/kqt9/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/kqt9/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/kqt9/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/kqt9/crates/atelier-core)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/kqt9/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.70s
────────────
 Nextest run ID ef2342e4-6b2a-45ab-9382-62096904ce67 with nextest profile: default
    Starting 631 tests across 10 binaries
        PASS [   0.007s] (  1/631) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.029s] (  2/631) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.033s] (  3/631) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.060s] (  4/631) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.063s] (  5/631) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.073s] (  6/631) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.056s] (  7/631) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.105s] (  8/631) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.071s] (  9/631) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.166s] ( 10/631) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.183s] ( 11/631) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.185s] ( 12/631) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.126s] ( 13/631) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.105s] ( 14/631) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.191s] ( 15/631) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.046s] ( 16/631) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.261s] ( 17/631) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.007s] ( 18/631) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.098s] ( 19/631) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.088s] ( 20/631) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.009s] ( 21/631) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.007s] ( 22/631) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 23/631) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.008s] ( 24/631) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.016s] ( 25/631) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.147s] ( 26/631) atelier-
```

