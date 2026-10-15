---
created_at: "2026-10-15T23:25:14.302868649+00:00"
id: "atelier-tc6c"
evidence_type: "test"
commit: "f95cff85c765b46200e3e1c032a72a36f6411a11"
result: "pass"
captured_at: "2026-10-15T23:25:03.578645934+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-ha6k"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ha6k"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after review fixes"
updated_at: "2026-10-15T23:25:14.303492592+00:00"
---

## Summary

Full workspace nextest suite passes after review fixes

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

Bytes: 73864
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/ha6k/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.24s
────────────
 Nextest run ID ffe1fa20-768c-47fa-b2f8-322f9bde03a1 with nextest profile: default
    Starting 592 tests across 10 binaries
        PASS [   0.027s] (  1/592) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.035s] (  2/592) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.054s] (  3/592) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.066s] (  4/592) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.083s] (  5/592) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.112s] (  6/592) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.115s] (  7/592) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.126s] (  8/592) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.120s] (  9/592) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.198s] ( 10/592) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.127s] ( 11/592) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.266s] ( 12/592) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.292s] ( 13/592) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.058s] ( 14/592) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.301s] ( 15/592) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.007s] ( 16/592) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.121s] ( 17/592) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.009s] ( 18/592) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.006s] ( 19/592) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 20/592) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 21/592) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 22/592) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.006s] ( 23/592) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.007s] ( 24/592) atelier-app issue_read::tests::objective_scope_keeps_all_rows_before_rendering_and_recent_activity_includes_active_work
        PASS [   0.010s] ( 25/592) atelier-app mission_overview::tests::complete_summaries_keep_counts_beyond_display_limits_and_stable_status_priority_order
        PASS [   0.063s] ( 26/592) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.009s] ( 27/592) atelier-app mission_overview::tests::directed_membership_deduplicates_roots_and_preserves_shared_scope_and_blockers
        PASS [   0.133s] ( 28/592) atelier-app git_fresh
```

