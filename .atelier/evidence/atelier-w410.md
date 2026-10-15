---
created_at: "2026-10-15T23:44:50.695541002+00:00"
id: "atelier-w410"
evidence_type: "test"
commit: "f2ee48dd605827071a9588a720968d03b3b1619a"
result: "pass"
captured_at: "2026-10-15T23:44:39.448100567+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-3mjv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3mjv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after fixing the shadowed base"
updated_at: "2026-10-15T23:44:50.696259405+00:00"
---

## Summary

Full workspace nextest suite passes after fixing the shadowed base

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

Bytes: 74714
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/3mjv/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.38s
────────────
 Nextest run ID bbe787c9-79af-4efe-9d16-f3985350a2d1 with nextest profile: default
    Starting 599 tests across 10 binaries
        PASS [   0.028s] (  1/599) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.034s] (  2/599) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.056s] (  3/599) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.066s] (  4/599) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.092s] (  5/599) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.110s] (  6/599) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.114s] (  7/599) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.121s] (  8/599) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.137s] (  9/599) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.196s] ( 10/599) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.129s] ( 11/599) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.266s] ( 12/599) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.059s] ( 13/599) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.299s] ( 14/599) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.007s] ( 15/599) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.311s] ( 16/599) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.008s] ( 17/599) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.126s] ( 18/599) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.006s] ( 19/599) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.011s] ( 20/599) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 21/599) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 22/599) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.006s] ( 23/599) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.136s] ( 24/599) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 25/599) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.007s] ( 26/599) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
        PASS [   0.068s] ( 27/599) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.010s] ( 28/599) atelier-app mission_overview::tests::complete_summaries_keep_counts_beyond_display
```

