---
created_at: "2026-10-17T06:05:20.438861235+00:00"
id: "atelier-xxy8"
evidence_type: "test"
commit: "9420c2c7d0f9efec36f7378d178b0bd3d14b4fec"
result: "pass"
captured_at: "2026-10-17T06:04:48.921758746+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
agent_identity: "orchestrator (Claude Opus 5.5)"
target:
  kind: "issue"
  id: "atelier-kops"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kops"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full suite with test hooks after the ulvb doc fix and the master catch-up merge"
updated_at: "2026-10-17T06:05:20.440159106+00:00"
---

## Summary

Full suite with test hooks after the ulvb doc fix and the master catch-up merge

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

Bytes: 84872
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-kops/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.26s
────────────
 Nextest run ID 0566d280-0729-413a-80dc-f3bb95460268 with nextest profile: default
    Starting 677 tests across 10 binaries
        PASS [   0.046s] (  1/677) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.632s] (  2/677) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   1.020s] (  3/677) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   1.027s] (  4/677) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.996s] (  5/677) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   1.069s] (  6/677) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.229s] (  7/677) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   1.241s] (  8/677) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.243s] (  9/677) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.291s] ( 10/677) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.089s] ( 11/677) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.310s] ( 12/677) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   1.363s] ( 13/677) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.154s] ( 14/677) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.751s] ( 15/677) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.103s] ( 16/677) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   1.627s] ( 17/677) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.290s] ( 18/677) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.279s] ( 19/677) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.007s] ( 20/677) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.008s] ( 21/677) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.011s] ( 22/677) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 23/677) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.345s] ( 24/677) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.076s] ( 25/677) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.008s] ( 26/677) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.009s] ( 27/677) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.008s] ( 28/677) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.007s] ( 29/67
```

