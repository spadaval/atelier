---
created_at: "2026-10-17T03:17:10.999033684+00:00"
id: "atelier-1scx"
evidence_type: "test"
commit: "22e0e050eed76c895b088e653b205bbb8dbfa2de"
result: "pass"
captured_at: "2026-10-17T03:16:52.006847504+00:00"
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
title: "Failure-path suite passes with test-hooks after the unreadable-branch fix"
updated_at: "2026-10-17T03:17:10.999762222+00:00"
---

## Summary

Failure-path suite passes with test-hooks after the unreadable-branch fix

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

Bytes: 83949
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-1w8c/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.47s
────────────
 Nextest run ID a10a108a-9d25-45fe-b73a-ce07f04abeb1 with nextest profile: default
    Starting 671 tests across 10 binaries
        PASS [   0.009s] (  1/671) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   1.221s] (  2/671) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   2.168s] (  3/671) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   2.222s] (  4/671) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   2.414s] (  5/671) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   3.063s] (  6/671) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   3.086s] (  7/671) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   3.111s] (  8/671) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   3.330s] (  9/671) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.110s] ( 10/671) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.238s] ( 11/671) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   2.205s] ( 12/671) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   3.450s] ( 13/671) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.367s] ( 14/671) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   1.053s] ( 15/671) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.184s] ( 16/671) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   3.562s] ( 17/671) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.124s] ( 18/671) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.188s] ( 19/671) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.009s] ( 20/671) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 21/671) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.228s] ( 22/671) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.006s] ( 23/671) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.005s] ( 24/671) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.005s] ( 25/671) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.005s] ( 26/671) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.005s] ( 27/671) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.031s] ( 28/671) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 29/671) atelier-
```

