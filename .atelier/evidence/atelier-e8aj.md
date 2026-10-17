---
created_at: "2026-10-17T07:14:38.220808659+00:00"
id: "atelier-e8aj"
evidence_type: "test"
commit: "1d90c9f5e58c1f0db244b07434c79380047eb19e"
result: "pass"
captured_at: "2026-10-17T07:14:15.401810238+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
agent_identity: "orchestrator (Claude Opus 5.5)"
target:
  kind: "issue"
  id: "atelier-ydv0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ydv0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full suite with test hooks after the goom and liml fixes and a clean merge of master (atelier-7iyw)"
updated_at: "2026-10-17T07:14:38.221635552+00:00"
---

## Summary

Full suite with test hooks after the goom and liml fixes and a clean merge of master (atelier-7iyw)

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

Bytes: 87907
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ydv0/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.64s
────────────
 Nextest run ID 1369bc81-ad35-40d9-8090-726540b2e825 with nextest profile: default
    Starting 701 tests across 10 binaries
        PASS [   0.010s] (  1/701) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.040s] (  2/701) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.044s] (  3/701) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.068s] (  4/701) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.076s] (  5/701) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.079s] (  6/701) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.229s] (  7/701) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.243s] (  8/701) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.194s] (  9/701) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.323s] ( 10/701) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.411s] ( 11/701) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.378s] ( 12/701) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.218s] ( 13/701) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.502s] ( 14/701) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.286s] ( 15/701) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.212s] ( 16/701) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.061s] ( 17/701) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.593s] ( 18/701) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.127s] ( 19/701) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.154s] ( 20/701) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.013s] ( 21/701) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.015s] ( 22/701) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.019s] ( 23/701) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.181s] ( 24/701) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.130s] ( 25/701) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.015s] ( 26/701) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 27/701) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.013s] ( 28/701) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.011s] ( 29/701) atelier-ap
```

