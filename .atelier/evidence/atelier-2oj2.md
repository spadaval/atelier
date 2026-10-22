---
created_at: "2026-10-22T20:46:48.469146510+00:00"
id: "atelier-2oj2"
evidence_type: "test"
commit: "1d2f67df28ea9dfd13863c5f41e37302f069dbba"
result: "pass"
captured_at: "2026-10-22T20:45:46.234797940+00:00"
command: "cargo nextest run -j4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-ftqn"
target:
  kind: "issue"
  id: "atelier-ftqn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ftqn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes with test-hooks after review fixes (816/816)"
updated_at: "2026-10-22T20:46:48.469453192+00:00"
---

## Summary

Full workspace suite passes with test-hooks after review fixes (816/816)

## Command

```console
cargo nextest run -j4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 102333
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-9g48/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.13s
────────────
 Nextest run ID 3806bbae-b134-40a0-a9be-28d16b39da39 with nextest profile: default
    Starting 816 tests across 10 binaries
        PASS [   0.071s] (  1/816) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.175s] (  2/816) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.212s] (  3/816) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.059s] (  4/816) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.106s] (  5/816) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.040s] (  6/816) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.326s] (  7/816) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.006s] (  8/816) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.007s] (  9/816) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.043s] ( 10/816) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.067s] ( 11/816) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.143s] ( 12/816) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.163s] ( 13/816) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.059s] ( 14/816) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.589s] ( 15/816) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.119s] ( 16/816) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.337s] ( 17/816) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.130s] ( 18/816) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.073s] ( 19/816) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.336s] ( 20/816) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.096s] ( 21/816) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.103s] ( 22/816) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.138s] ( 23/816) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.080s] ( 24/816) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.102s] ( 25/816) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.044s] ( 26/816) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.063s] ( 27/816) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.144s] ( 28/816) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type
```

