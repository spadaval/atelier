---
created_at: "2026-10-22T20:47:36.817736432+00:00"
id: "atelier-jg3b"
evidence_type: "test"
commit: "6d983eb53c10a3d2264fe56932440f77011ce224"
result: "pass"
captured_at: "2026-10-22T20:47:09.612171768+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-2yl7"
target:
  kind: "issue"
  id: "atelier-2yl7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2yl7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes"
updated_at: "2026-10-22T20:47:36.818008990+00:00"
---

## Summary

Full workspace suite with test-hooks passes

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 103350
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-c3kh/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.66s
────────────
 Nextest run ID bd8aed55-48c0-468a-aec9-ed333d5e1476 with nextest profile: default
    Starting 824 tests across 10 binaries
        PASS [   0.070s] (  1/824) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.173s] (  2/824) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.194s] (  3/824) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.060s] (  4/824) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.103s] (  5/824) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.302s] (  6/824) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.031s] (  7/824) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.009s] (  8/824) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.012s] (  9/824) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.074s] ( 10/824) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.122s] ( 11/824) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.150s] ( 12/824) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.154s] ( 13/824) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.052s] ( 14/824) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.600s] ( 15/824) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.096s] ( 16/824) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.281s] ( 17/824) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.106s] ( 18/824) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.063s] ( 19/824) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.290s] ( 20/824) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.109s] ( 21/824) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.111s] ( 22/824) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.141s] ( 23/824) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.117s] ( 24/824) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.117s] ( 25/824) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.054s] ( 26/824) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.059s] ( 27/824) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.148s] ( 28/824) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.007s] ( 29/824) atelier
```

