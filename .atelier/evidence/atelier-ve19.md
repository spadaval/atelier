---
created_at: "2026-10-17T02:59:22.791929844+00:00"
id: "atelier-ve19"
evidence_type: "test"
commit: "dee0d2478d836bef878bd5cc36a7ea32cc695385"
result: "pass"
captured_at: "2026-10-17T02:59:10.450374365+00:00"
command: "cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7u4o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7u4o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite after merging master (with atelier-sdb0)"
updated_at: "2026-10-17T02:59:22.792641892+00:00"
---

## Summary

Full nextest suite after merging master (with atelier-sdb0)

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

Bytes: 82974
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-7u4o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.38s
────────────
 Nextest run ID 42f08300-e71e-4867-9bae-a5b5e8e6b2b2 with nextest profile: default
    Starting 664 tests across 10 binaries
        PASS [   0.010s] (  1/664) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.038s] (  2/664) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.044s] (  3/664) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.078s] (  4/664) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.079s] (  5/664) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.099s] (  6/664) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.142s] (  7/664) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.087s] (  8/664) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.203s] (  9/664) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.239s] ( 10/664) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.161s] ( 11/664) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.087s] ( 12/664) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.267s] ( 13/664) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.238s] ( 14/664) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.154s] ( 15/664) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.048s] ( 16/664) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.112s] ( 17/664) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.089s] ( 18/664) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.115s] ( 19/664) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.007s] ( 20/664) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.008s] ( 21/664) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.387s] ( 22/664) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.111s] ( 23/664) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.150s] ( 24/664) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.007s] ( 25/664) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.010s] ( 26/664) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 27/664) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 28/664) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.008s] ( 29/664) atel
```

