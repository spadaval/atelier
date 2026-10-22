---
created_at: "2026-10-22T23:21:58.029256307+00:00"
id: "atelier-k62g"
evidence_type: "test"
commit: "81c707bb9dbff085e061a7410a488cf8eaf39ae4"
result: "pass"
captured_at: "2026-10-22T23:21:15.073524301+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-u6f3"
target:
  kind: "issue"
  id: "atelier-u6f3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u6f3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes with test-hooks"
updated_at: "2026-10-22T23:21:58.029550585+00:00"
---

## Summary

Full workspace suite passes with test-hooks

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

Bytes: 110456
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-j8kg/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.60s
────────────
 Nextest run ID e08f7693-ca11-4ebb-9293-0c936cc5f608 with nextest profile: default
    Starting 878 tests across 10 binaries
        PASS [   0.050s] (  1/878) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.128s] (  2/878) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.146s] (  3/878) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.045s] (  4/878) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.080s] (  5/878) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.025s] (  6/878) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/878) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.241s] (  8/878) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  9/878) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.026s] ( 10/878) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.042s] ( 11/878) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.107s] ( 12/878) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.125s] ( 13/878) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.044s] ( 14/878) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.424s] ( 15/878) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.093s] ( 16/878) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.253s] ( 17/878) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.098s] ( 18/878) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.055s] ( 19/878) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.252s] ( 20/878) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.072s] ( 21/878) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.099s] ( 22/878) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.074s] ( 23/878) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.056s] ( 24/878) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.070s] ( 25/878) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 26/878) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.049s] ( 27/878) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.108s] ( 28/878) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/878) atelier
```

