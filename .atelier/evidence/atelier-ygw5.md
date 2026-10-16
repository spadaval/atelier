---
created_at: "2026-10-16T18:51:35.995044929+00:00"
id: "atelier-ygw5"
evidence_type: "test"
commit: "eb7946e0513cee0ffab21c0ad10323679aa97f2d"
result: "pass"
captured_at: "2026-10-16T18:51:19.572829869+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-9co2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9co2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T18:51:35.995382600+00:00"
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

Bytes: 78274
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/9co2/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/9co2/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.92s
────────────
 Nextest run ID cdc647c2-de23-4d20-a403-79ec19f63694 with nextest profile: default
    Starting 626 tests across 10 binaries
        PASS [   0.011s] (  1/626) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.035s] (  2/626) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.074s] (  3/626) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.116s] (  4/626) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.117s] (  5/626) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.137s] (  6/626) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.120s] (  7/626) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.186s] (  8/626) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.119s] (  9/626) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.216s] ( 10/626) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.289s] ( 11/626) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.302s] ( 12/626) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.326s] ( 13/626) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.226s] ( 14/626) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.171s] ( 15/626) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.364s] ( 16/626) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.083s] ( 17/626) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.017s] ( 18/626) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.014s] ( 19/626) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.010s] ( 20/626) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.152s] ( 21/626) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.165s] ( 22/626) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.008s] ( 23/626) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.104s] ( 24/626) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.012s] ( 25/626) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.010s] ( 26/626) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.018s] ( 27/626) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.010s] ( 28/626) atelier-app issue_read::tests::objective_s
```

