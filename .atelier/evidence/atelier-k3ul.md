---
created_at: "2026-10-15T23:10:36.294768505+00:00"
id: "atelier-k3ul"
evidence_type: "test"
commit: "74077b1a13adef4e51c6a35bff644efd457bee74"
result: "pass"
captured_at: "2026-10-15T23:10:22.335106001+00:00"
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
title: "Full workspace nextest suite passes, including the base-commit landing tests"
updated_at: "2026-10-15T23:10:36.295574697+00:00"
---

## Summary

Full workspace nextest suite passes, including the base-commit landing tests

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

Bytes: 74270
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/3mjv/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/3mjv/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/3mjv/crates/atelier-core)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/3mjv/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/3mjv/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.56s
────────────
 Nextest run ID 23863719-f682-4de6-b238-71e5460b139e with nextest profile: default
    Starting 592 tests across 10 binaries
        PASS [   0.025s] (  1/592) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.034s] (  2/592) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.059s] (  3/592) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.066s] (  4/592) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.085s] (  5/592) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.112s] (  6/592) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.116s] (  7/592) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.121s] (  8/592) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.188s] (  9/592) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.129s] ( 10/592) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.127s] ( 11/592) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.261s] ( 12/592) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.292s] ( 13/592) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.295s] ( 14/592) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.007s] ( 15/592) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.067s] ( 16/592) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.121s] ( 17/592) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.009s] ( 18/592) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 19/592) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 20/592) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 21/592) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 22/592) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.133s] ( 23/592) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 24/592) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.063s] ( 25/592) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_e
```

