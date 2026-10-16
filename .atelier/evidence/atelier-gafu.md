---
created_at: "2026-10-16T20:52:49.868746777+00:00"
id: "atelier-gafu"
evidence_type: "test"
commit: "b412a9707cc4e7d8783eecbe0c65c78bb394f03c"
result: "pass"
captured_at: "2026-10-16T20:52:12.542882371+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-cr0r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cr0r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T20:52:49.869820302+00:00"
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

Bytes: 81301
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/np/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/np/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/np/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/np/crates/atelier-core)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/np/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 11.37s
────────────
 Nextest run ID bc84e060-246c-41e1-931b-a09c63570f42 with nextest profile: default
    Starting 645 tests across 10 binaries
        PASS [   0.010s] (  1/645) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   1.390s] (  2/645) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   1.806s] (  3/645) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   2.447s] (  4/645) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   2.458s] (  5/645) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   2.481s] (  6/645) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.090s] (  7/645) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   3.565s] (  8/645) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   5.713s] (  9/645) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   6.184s] ( 10/645) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   4.646s] ( 11/645) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   5.749s] ( 12/645) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   9.030s] ( 13/645) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   4.099s] ( 14/645) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   1.995s] ( 15/645) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   7.325s] ( 16/645) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.163s] ( 17/645) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.010s] ( 18/645) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.009s] ( 19/645) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   9.221s] ( 20/645) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.013s] ( 21/645) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.007s] ( 22/645) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 23/645) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.006s] ( 24/645) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   3.052s] ( 25/645) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.008s] ( 26/645) atelier-app issue_r
```

