---
created_at: "2026-10-16T00:49:09.572382027+00:00"
id: "atelier-zdmo"
evidence_type: "test"
commit: "511a2384534581b43ab0498cb2d1dea6417c6b03"
result: "pass"
captured_at: "2026-10-16T00:48:58.272744422+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-4n7v"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4n7v"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after the nesting-check fix"
updated_at: "2026-10-16T00:49:09.573025026+00:00"
---

## Summary

Full workspace nextest suite passes after the nesting-check fix

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

Bytes: 75634
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/4n7v/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.24s
────────────
 Nextest run ID 59fb0fc2-2111-4a21-8c41-c852b9b9ed9e with nextest profile: default
    Starting 606 tests across 10 binaries
        PASS [   0.028s] (  1/606) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.037s] (  2/606) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.056s] (  3/606) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.064s] (  4/606) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.082s] (  5/606) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.109s] (  6/606) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.116s] (  7/606) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.121s] (  8/606) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.196s] (  9/606) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.145s] ( 10/606) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.134s] ( 11/606) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.270s] ( 12/606) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.300s] ( 13/606) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.060s] ( 14/606) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.008s] ( 15/606) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.008s] ( 16/606) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.124s] ( 17/606) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.007s] ( 18/606) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.010s] ( 19/606) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.330s] ( 20/606) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.065s] ( 21/606) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.134s] ( 22/606) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.008s] ( 23/606) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.006s] ( 24/606) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.009s] ( 25/606) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 26/606) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
        PASS [   0.008s] ( 27/606) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.011s] ( 28/606) atelier-app mission_overview::tests::complete_summaries_keep_counts_beyond_display
```

