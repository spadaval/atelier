---
created_at: "2026-10-16T23:28:00.295901864+00:00"
id: "atelier-enxf"
evidence_type: "test"
commit: "c2f02115a1f58a17e8a2fef875e1664dae929787"
result: "pass"
captured_at: "2026-10-16T23:27:22.957704736+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-bibd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bibd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T23:28:00.305797710+00:00"
---

## Summary

cargo nextest run --workspace

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

Bytes: 83179
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/bibd/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/bibd/crates/atelier-app)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/bibd/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/bibd/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/bibd/crates/atelier-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.98s
────────────
 Nextest run ID f67ca5f2-d902-4dfc-8886-2e8fba988a04 with nextest profile: default
    Starting 649 tests across 10 binaries
        PASS [   0.008s] (  1/649) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.461s] (  2/649) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   1.099s] (  3/649) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   1.098s] (  4/649) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   1.183s] (  5/649) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.275s] (  6/649) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   1.287s] (  7/649) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   1.460s] (  8/649) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.738s] (  9/649) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.920s] ( 10/649) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.691s] ( 11/649) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   3.023s] ( 12/649) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.593s] ( 13/649) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   3.219s] ( 14/649) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   4.365s] ( 15/649) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   4.549s] ( 16/649) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   5.335s] ( 17/649) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   7.021s] ( 18/649) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   8.157s] ( 19/649) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.004s] ( 20/649) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 21/649) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.006s] ( 22/649) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.004s] ( 23/649) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inve
```

