---
created_at: "2026-10-17T07:57:10.481745021+00:00"
id: "atelier-468w"
evidence_type: "test"
commit: "812acbd01a11b2bf1c99856a7159ae4815dc5376"
result: "pass"
captured_at: "2026-10-17T07:56:21.934762670+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-krf4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-krf4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes, including the rollup health test and the count-phrasing and rendering tests"
updated_at: "2026-10-17T07:57:10.482940722+00:00"
---

## Summary

Full workspace nextest suite passes, including the rollup health test and the count-phrasing and rendering tests

## Command

```console
cargo nextest run -j 4
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 90986
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-krf4/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.66s
────────────
 Nextest run ID 52c9f087-1ddc-4881-91e9-ce0b8f0de74e with nextest profile: default
    Starting 720 tests across 10 binaries
        PASS [   3.107s] (  1/720) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   4.010s] (  2/720) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        SLOW [> 10.000s] (───────) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [  13.410s] (  3/720) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   9.648s] (  4/720) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [  13.658s] (  5/720) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.288s] (  6/720) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.008s] (  7/720) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.140s] (  8/720) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.108s] (  9/720) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.086s] ( 10/720) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.165s] ( 11/720) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.289s] ( 12/720) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.147s] ( 13/720) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [  11.393s] ( 14/720) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.245s] ( 15/720) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.627s] ( 16/720) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.100s] ( 17/720) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.165s] ( 18/720) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.135s] ( 19/720) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.395s] ( 20/720) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.122s] ( 21/720) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.102s] ( 22/720) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.070s] ( 23/720) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   1.453s] ( 24/720) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.122s] ( 25/720) atelier-app
```

