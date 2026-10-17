---
created_at: "2026-10-17T18:42:12.879634206+00:00"
id: "atelier-x7wa"
evidence_type: "test"
commit: "fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e"
result: "pass"
captured_at: "2026-10-17T18:41:33.580969584+00:00"
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
title: "Workspace tests pass with test-hooks after the review fixes"
updated_at: "2026-10-17T18:42:12.880954441+00:00"
---

## Summary

Workspace tests pass with test-hooks after the review fixes

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

Bytes: 98277
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-ho7o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.15s
────────────
 Nextest run ID 278d0e08-6250-4131-b230-53bf1084b25a with nextest profile: default
    Starting 784 tests across 10 binaries
        PASS [   0.051s] (  1/784) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.124s] (  2/784) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.145s] (  3/784) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.043s] (  4/784) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.076s] (  5/784) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.024s] (  6/784) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/784) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.004s] (  8/784) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.234s] (  9/784) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.026s] ( 10/784) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.041s] ( 11/784) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.103s] ( 12/784) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.122s] ( 13/784) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.042s] ( 14/784) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.409s] ( 15/784) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.102s] ( 16/784) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.248s] ( 17/784) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.095s] ( 18/784) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.052s] ( 19/784) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.246s] ( 20/784) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.070s] ( 21/784) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.098s] ( 22/784) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.071s] ( 23/784) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.056s] ( 24/784) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.068s] ( 25/784) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.031s] ( 26/784) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.046s] ( 27/784) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.105s] ( 28/784) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type
```

