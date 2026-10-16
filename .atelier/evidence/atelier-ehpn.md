---
created_at: "2026-10-16T20:49:33.962700802+00:00"
id: "atelier-ehpn"
evidence_type: "test"
commit: "6720c7bfbca4a36daff776eafc1dad6b5c451fdb"
result: "pass"
captured_at: "2026-10-16T20:49:11.160144511+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-imbo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-imbo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo nextest run --workspace"
updated_at: "2026-10-16T20:49:33.962972118+00:00"
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

Bytes: 80581
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/imbo/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/imbo/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.53s
────────────
 Nextest run ID 9ef99c69-ece3-48b2-8046-fdbe4be87509 with nextest profile: default
    Starting 644 tests across 10 binaries
        PASS [   0.008s] (  1/644) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   2.230s] (  2/644) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   2.651s] (  3/644) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   3.022s] (  4/644) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   3.040s] (  5/644) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.146s] (  6/644) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   3.168s] (  7/644) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   3.283s] (  8/644) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   1.340s] (  9/644) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.129s] ( 10/644) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   4.281s] ( 11/644) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.287s] ( 12/644) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   1.113s] ( 13/644) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   4.397s] ( 14/644) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.746s] ( 15/644) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.184s] ( 16/644) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.084s] ( 17/644) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   4.483s] ( 18/644) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.009s] ( 19/644) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.018s] ( 20/644) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.106s] ( 21/644) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.017s] ( 22/644) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.009s] ( 23/644) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.944s] ( 24/644) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.010s] ( 25/644) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.243s] ( 26/644) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.012s] ( 27/644) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.010s] ( 28/644) atelier-app issue_read::tests::objective_ro
```

