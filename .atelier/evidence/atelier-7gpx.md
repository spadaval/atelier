---
created_at: "2026-10-17T03:45:50.777465031+00:00"
id: "atelier-7gpx"
evidence_type: "test"
commit: "67955ad80086c2d58d98ab61a66c09942ca67f30"
result: "pass"
captured_at: "2026-10-17T03:45:39.253812086+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-deyy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-deyy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after the dense work-view rows"
updated_at: "2026-10-17T03:45:50.778176367+00:00"
---

## Summary

Full nextest suite passes after the dense work-view rows

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

Bytes: 84278
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-1w8c/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.22s
────────────
 Nextest run ID c669f15a-7ab5-44cc-b2a9-6c923f7c6af5 with nextest profile: default
    Starting 673 tests across 10 binaries
        PASS [   0.008s] (  1/673) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.038s] (  2/673) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.041s] (  3/673) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.062s] (  4/673) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.063s] (  5/673) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.077s] (  6/673) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.114s] (  7/673) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.062s] (  8/673) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.149s] (  9/673) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.183s] ( 10/673) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.131s] ( 11/673) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.085s] ( 12/673) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.176s] ( 13/673) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.223s] ( 14/673) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.140s] ( 15/673) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.103s] ( 16/673) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.080s] ( 17/673) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.044s] ( 18/673) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.105s] ( 19/673) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.007s] ( 20/673) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 21/673) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.144s] ( 22/673) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.010s] ( 23/673) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.117s] ( 24/673) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.008s] ( 25/673) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.007s] ( 26/673) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 27/673) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.353s] ( 28/673) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.008s] ( 29
```

