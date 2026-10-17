---
created_at: "2026-10-17T18:51:13.509369063+00:00"
id: "atelier-jufg"
evidence_type: "test"
commit: "f07f14b0a4348efe39ab0cf3222e046e281c3130"
result: "pass"
captured_at: "2026-10-17T18:50:39.988571515+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-ho7o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ho7o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass with test-hooks after storing criteria_mission"
updated_at: "2026-10-17T18:51:13.510290954+00:00"
---

## Summary

Workspace tests pass with test-hooks after storing criteria_mission

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

Bytes: 99329
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-ho7o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.27s
────────────
 Nextest run ID 1fefd32a-9180-4a59-8beb-a5770823841a with nextest profile: default
    Starting 793 tests across 10 binaries
        PASS [   3.466s] (  1/793) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.978s] (  2/793) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   4.038s] (  3/793) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.089s] (  4/793) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   4.070s] (  5/793) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.050s] (  6/793) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.005s] (  7/793) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.026s] (  8/793) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  9/793) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.027s] ( 10/793) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.043s] ( 11/793) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.105s] ( 12/793) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.123s] ( 13/793) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.042s] ( 14/793) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.913s] ( 15/793) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.088s] ( 16/793) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.238s] ( 17/793) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.090s] ( 18/793) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.052s] ( 19/793) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.069s] ( 20/793) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.241s] ( 21/793) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.099s] ( 22/793) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.070s] ( 23/793) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.056s] ( 24/793) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.070s] ( 25/793) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.031s] ( 26/793) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.045s] ( 27/793) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.103s] ( 28/793) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 29/793) atelier
```

