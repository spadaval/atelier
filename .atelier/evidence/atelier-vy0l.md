---
created_at: "2026-10-17T02:49:37.770136928+00:00"
id: "atelier-vy0l"
evidence_type: "test"
commit: "c86e45e88f4a4e3db6c3b97b3564fa293e32a8d9"
result: "pass"
captured_at: "2026-10-17T02:49:26.346286485+00:00"
command: "env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run --features test-hooks"
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
title: "Failure-path suite (test-hooks) passes after review fixes"
updated_at: "2026-10-17T02:49:37.770797640+00:00"
---

## Summary

Failure-path suite (test-hooks) passes after review fixes

## Command

```console
env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 82943
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-7u4o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.15s
────────────
 Nextest run ID 7c5ce32f-9c4f-46bc-9db0-05009e99060e with nextest profile: default
    Starting 663 tests across 10 binaries
        PASS [   0.007s] (  1/663) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.036s] (  2/663) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.037s] (  3/663) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.062s] (  4/663) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.067s] (  5/663) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.083s] (  6/663) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.122s] (  7/663) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.071s] (  8/663) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.169s] (  9/663) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.128s] ( 10/663) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.196s] ( 11/663) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.083s] ( 12/663) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.225s] ( 13/663) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.218s] ( 14/663) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.162s] ( 15/663) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.100s] ( 16/663) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.131s] ( 17/663) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.130s] ( 18/663) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.071s] ( 19/663) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.008s] ( 20/663) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.009s] ( 21/663) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.011s] ( 22/663) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.389s] ( 23/663) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.008s] ( 24/663) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 25/663) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.202s] ( 26/663) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.144s] ( 27/663) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.009s] ( 28/663) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.008s] ( 29/663) atel
```

