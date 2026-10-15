---
created_at: "2026-10-15T22:55:29.059532210+00:00"
id: "atelier-dwu5"
evidence_type: "test"
commit: "85346c57e995139485e7a6d03b0d2976a7b2e80b"
result: "pass"
captured_at: "2026-10-15T22:55:18.270478138+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-fxbg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fxbg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes, including unowned_branch_lands_unless_it_changes_records"
updated_at: "2026-10-15T22:55:29.060594450+00:00"
---

## Summary

Full workspace nextest suite passes, including unowned_branch_lands_unless_it_changes_records

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

Bytes: 73829
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier/crates/atelier-core)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.52s
────────────
 Nextest run ID c6cb630a-9b7b-49ce-9dbb-47a1a2699420 with nextest profile: default
    Starting 589 tests across 10 binaries
        PASS [   0.030s] (  1/589) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.035s] (  2/589) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.058s] (  3/589) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.072s] (  4/589) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.091s] (  5/589) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.106s] (  6/589) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.120s] (  7/589) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.114s] (  8/589) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.189s] (  9/589) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.132s] ( 10/589) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.128s] ( 11/589) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.262s] ( 12/589) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.060s] ( 13/589) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.120s] ( 14/589) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.310s] ( 15/589) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.311s] ( 16/589) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.007s] ( 17/589) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 18/589) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.009s] ( 19/589) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.060s] ( 20/589) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.007s] ( 21/589) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 22/589) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 23/589) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.138s] ( 24/589) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.009s] ( 25/589) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.007s] ( 26/5
```

