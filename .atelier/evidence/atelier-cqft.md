---
created_at: "2026-10-16T01:06:37.532444309+00:00"
id: "atelier-cqft"
evidence_type: "test"
commit: "a27bf476d9938f295a00b92d2c8e404cec1e49fe"
result: "pass"
captured_at: "2026-10-16T01:06:25.516691813+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-ox63"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ox63"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes, including test_decisions_wait_for_the_human_and_close_with_a_recorded_decision"
updated_at: "2026-10-16T01:06:37.533846344+00:00"
---

## Summary

Full workspace nextest suite passes, including test_decisions_wait_for_the_human_and_close_with_a_recorded_decision

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

Bytes: 76422
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-records)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-core)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.61s
────────────
 Nextest run ID c8de822a-5001-4e2c-8784-11702e016c11 with nextest profile: default
    Starting 609 tests across 10 binaries
        PASS [   0.030s] (  1/609) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.033s] (  2/609) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.057s] (  3/609) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.064s] (  4/609) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.089s] (  5/609) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.113s] (  6/609) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.115s] (  7/609) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.118s] (  8/609) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.131s] (  9/609) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.195s] ( 10/609) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.131s] ( 11/609) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.259s] ( 12/609) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.063s] ( 13/609) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.309s] ( 14/609) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.123s] ( 15/609) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.312s] ( 16/609) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.007s] ( 17/609) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 18/609) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.010s] ( 19/609) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 20/609) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.063s] ( 21/609) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.007s] ( 22/609) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.133s] ( 23/609) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 24/609) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 25/609) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
```

