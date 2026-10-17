---
created_at: "2026-10-17T06:27:37.705804317+00:00"
id: "atelier-sjb8"
evidence_type: "test"
commit: "fb1d42b4ef1b78e0dedbcb4a2743e4513c1b9150"
result: "pass"
captured_at: "2026-10-17T06:27:01.600736965+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
agent_identity: "orchestrator (Claude Opus 5.5)"
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
title: "Full suite with test hooks after the review fixes and the master catch-up merge (which brought in atelier-kops)"
updated_at: "2026-10-17T06:27:37.707108590+00:00"
---

## Summary

Full suite with test hooks after the review fixes and the master catch-up merge (which brought in atelier-kops)

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

Bytes: 85791
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ydv0/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.67s
────────────
 Nextest run ID 96e081d4-7b4e-4bc8-aeb7-719a34fe66fd with nextest profile: default
    Starting 684 tests across 10 binaries
        PASS [   0.008s] (  1/684) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   2.210s] (  2/684) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   2.907s] (  3/684) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   2.925s] (  4/684) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   2.959s] (  5/684) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.037s] (  6/684) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   3.037s] (  7/684) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   3.052s] (  8/684) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.162s] (  9/684) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   3.117s] ( 10/684) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.182s] ( 11/684) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   3.283s] ( 12/684) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.329s] ( 13/684) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.256s] ( 14/684) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   1.090s] ( 15/684) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   3.306s] ( 16/684) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.211s] ( 17/684) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.063s] ( 18/684) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.164s] ( 19/684) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.117s] ( 20/684) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.008s] ( 21/684) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.008s] ( 22/684) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.009s] ( 23/684) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.005s] ( 24/684) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.005s] ( 25/684) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.005s] ( 26/684) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.005s] ( 27/684) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.006s] ( 28/684) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.005s] ( 29/684) atelier-app issue_read::tests::objec
```

