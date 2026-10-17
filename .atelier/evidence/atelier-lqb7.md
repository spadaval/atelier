---
created_at: "2026-10-17T03:07:29.109464982+00:00"
id: "atelier-lqb7"
evidence_type: "test"
commit: "529b2cc98a52ea92d0bc6714d0e3124c2a5c725b"
result: "pass"
captured_at: "2026-10-17T03:06:53.404558980+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7n5u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7n5u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Failure-path suite passes with test-hooks"
updated_at: "2026-10-17T03:07:29.110912831+00:00"
---

## Summary

Failure-path suite passes with test-hooks

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

Bytes: 83058
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-1w8c/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.90s
────────────
 Nextest run ID 598a4715-b646-48f5-a3b4-d06f7f230346 with nextest profile: default
    Starting 664 tests across 10 binaries
        PASS [   0.020s] (  1/664) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.049s] (  2/664) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.089s] (  3/664) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.171s] (  4/664) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.171s] (  5/664) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.200s] (  6/664) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.324s] (  7/664) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.198s] (  8/664) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.376s] (  9/664) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.565s] ( 10/664) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.516s] ( 11/664) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.409s] ( 12/664) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.590s] ( 13/664) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.627s] ( 14/664) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.269s] ( 15/664) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.207s] ( 16/664) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.312s] ( 17/664) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.583s] ( 18/664) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.342s] ( 19/664) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.011s] ( 20/664) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.009s] ( 21/664) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.013s] ( 22/664) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.375s] ( 23/664) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.016s] ( 24/664) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.014s] ( 25/664) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.017s] ( 26/664) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.014s] ( 27/664) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.018s] ( 28/664) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.380s] ( 29/664) atelier-
```

