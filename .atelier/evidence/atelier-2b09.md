---
created_at: "2026-10-17T02:35:05.638530049+00:00"
id: "atelier-2b09"
evidence_type: "test"
commit: "4c88804ad47130448034ca04324e06d3b1ec757e"
result: "pass"
captured_at: "2026-10-17T02:34:52.918879890+00:00"
command: "env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sdb0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sdb0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full Rust suite passes after the status/prime change (656 tests), including moved prime tests, status --hook claude, prime alias, top-5 ready list, owner block, and setup CLAUDE.md warning"
updated_at: "2026-10-17T02:35:05.639260291+00:00"
---

## Summary

Full Rust suite passes after the status/prime change (656 tests), including moved prime tests, status --hook claude, prime alias, top-5 ready list, owner block, and setup CLAUDE.md warning

## Command

```console
env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 81952
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-1w8c/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.78s
────────────
 Nextest run ID 9c33478c-8f11-42a1-9d81-bd5b432a2e10 with nextest profile: default
    Starting 656 tests across 10 binaries
        PASS [   0.008s] (  1/656) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.043s] (  2/656) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.052s] (  3/656) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.073s] (  4/656) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.098s] (  5/656) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.116s] (  6/656) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.143s] (  7/656) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.094s] (  8/656) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.203s] (  9/656) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.245s] ( 10/656) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.157s] ( 11/656) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.089s] ( 12/656) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.261s] ( 13/656) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.252s] ( 14/656) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.181s] ( 15/656) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.099s] ( 16/656) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.137s] ( 17/656) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.058s] ( 18/656) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.128s] ( 19/656) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.006s] ( 20/656) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.398s] ( 21/656) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.009s] ( 22/656) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.013s] ( 23/656) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.017s] ( 24/656) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 25/656) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.008s] ( 26/656) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.009s] ( 27/656) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.136s] ( 28/656) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.008s] ( 2
```

