---
created_at: "2026-10-16T00:56:01.579951479+00:00"
id: "atelier-c43v"
evidence_type: "test"
commit: "3df28a5f40d2468ee052e4935daafaec8a0c0829"
result: "pass"
captured_at: "2026-10-16T00:55:47.637216643+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-by8u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-by8u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after review fixes"
updated_at: "2026-10-16T00:56:01.580587967+00:00"
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

Bytes: 76150
Truncated: yes

```text
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/by8u/crates/atelier-core)
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/by8u/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/by8u/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/by8u/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/by8u/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.67s
────────────
 Nextest run ID 88ed0c09-0882-43b4-9dda-74c79e1c922d with nextest profile: default
    Starting 607 tests across 10 binaries
        PASS [   0.027s] (  1/607) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.042s] (  2/607) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.065s] (  3/607) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.072s] (  4/607) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.089s] (  5/607) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.121s] (  6/607) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.154s] (  7/607) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.200s] (  8/607) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.128s] (  9/607) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.136s] ( 10/607) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.137s] ( 11/607) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.271s] ( 12/607) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.312s] ( 13/607) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.061s] ( 14/607) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.007s] ( 15/607) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.062s] ( 16/607) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.007s] ( 17/607) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.336s] ( 18/607) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.006s] ( 19/607) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.141s] ( 20/607) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.009s] ( 21/607) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 22/607) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.143s] ( 23/607) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 24/607) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.009s] ( 25/607) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
```

