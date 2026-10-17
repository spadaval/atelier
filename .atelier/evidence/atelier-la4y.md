---
created_at: "2026-10-17T06:58:08.944132463+00:00"
id: "atelier-la4y"
evidence_type: "test"
commit: "354f65160eb9b6402120c9fe88f9daf1986db2e6"
result: "pass"
captured_at: "2026-10-17T06:57:42.254800253+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-a76d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a76d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Test-hooks suite passes, covering the bundle cleanup warnings now printed with eprintln"
updated_at: "2026-10-17T06:58:08.944776934+00:00"
---

## Summary

Test-hooks suite passes, covering the bundle cleanup warnings now printed with eprintln

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

Bytes: 85443
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-a76d/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.61s
────────────
 Nextest run ID 95eb151b-2b41-4905-91e2-f38e547ddaeb with nextest profile: default
    Starting 682 tests across 10 binaries
        PASS [   0.012s] (  1/682) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.057s] (  2/682) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.116s] (  3/682) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.147s] (  4/682) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.165s] (  5/682) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.195s] (  6/682) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.318s] (  7/682) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.178s] (  8/682) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.410s] (  9/682) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.470s] ( 10/682) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.195s] ( 11/682) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.359s] ( 12/682) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.540s] ( 13/682) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.494s] ( 14/682) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.592s] ( 15/682) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.113s] ( 16/682) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.394s] ( 17/682) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.279s] ( 18/682) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.244s] ( 19/682) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.014s] ( 20/682) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.261s] ( 21/682) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.017s] ( 22/682) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.019s] ( 23/682) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.023s] ( 24/682) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.012s] ( 25/682) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.023s] ( 26/682) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.015s] ( 27/682) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.317s] ( 28/682) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.025s] ( 29/682) atel
```

