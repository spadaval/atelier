---
created_at: "2026-10-17T02:35:24.882832461+00:00"
id: "atelier-x5k1"
evidence_type: "test"
commit: "4c88804ad47130448034ca04324e06d3b1ec757e"
result: "pass"
captured_at: "2026-10-17T02:35:13.105414851+00:00"
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
title: "Failure-path suite with test-hooks passes (660 tests) after the status/prime change"
updated_at: "2026-10-17T02:35:24.883850790+00:00"
---

## Summary

Failure-path suite with test-hooks passes (660 tests) after the status/prime change

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

Bytes: 82557
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-1w8c/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.18s
────────────
 Nextest run ID f6b58bce-60bd-4af2-a1fc-a93df8fb7e54 with nextest profile: default
    Starting 660 tests across 10 binaries
        PASS [   0.008s] (  1/660) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.039s] (  2/660) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.045s] (  3/660) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.068s] (  4/660) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.079s] (  5/660) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.088s] (  6/660) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.132s] (  7/660) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.066s] (  8/660) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.157s] (  9/660) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.198s] ( 10/660) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.080s] ( 11/660) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.222s] ( 12/660) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.143s] ( 13/660) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.195s] ( 14/660) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.150s] ( 15/660) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.105s] ( 16/660) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.084s] ( 17/660) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.046s] ( 18/660) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.111s] ( 19/660) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.007s] ( 20/660) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.109s] ( 21/660) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.008s] ( 22/660) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.006s] ( 23/660) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.010s] ( 24/660) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.146s] ( 25/660) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.008s] ( 26/660) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.011s] ( 27/660) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.008s] ( 28/660) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.011s] ( 29/660) atelie
```

