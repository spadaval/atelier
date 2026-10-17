---
created_at: "2026-10-17T06:23:49.963700849+00:00"
id: "atelier-yupp"
evidence_type: "test"
commit: "d1e5a4d6114712f97900cbd0592eb923fc929032"
result: "pass"
captured_at: "2026-10-17T06:22:44.876455154+00:00"
command: "cargo nextest run"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-i2qq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-i2qq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after the review fixes"
updated_at: "2026-10-17T06:23:49.964634221+00:00"
---

## Summary

Full nextest suite passes after the review fixes

## Command

```console
cargo nextest run
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 84772
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-o3lm/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.00s
────────────
 Nextest run ID f86cad9b-9cd5-4b5d-b18e-7a04aef35ab8 with nextest profile: default
    Starting 677 tests across 10 binaries
        PASS [   0.032s] (  1/677) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.154s] (  2/677) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.195s] (  3/677) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.232s] (  4/677) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.259s] (  5/677) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.271s] (  6/677) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.142s] (  7/677) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.429s] (  8/677) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.429s] (  9/677) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.519s] ( 10/677) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.273s] ( 11/677) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.538s] ( 12/677) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.395s] ( 13/677) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.128s] ( 14/677) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.815s] ( 15/677) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.490s] ( 16/677) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.320s] ( 17/677) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.092s] ( 18/677) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.379s] ( 19/677) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.362s] ( 20/677) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.405s] ( 21/677) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.008s] ( 22/677) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.014s] ( 23/677) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.017s] ( 24/677) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.013s] ( 25/677) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.024s] ( 26/677) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 27/677) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.011s] ( 28/677) atelier-app issue_read::tests::objective_rows_keep_priorit
```

