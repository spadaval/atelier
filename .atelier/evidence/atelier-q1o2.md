---
created_at: "2026-10-17T18:48:13.404444158+00:00"
id: "atelier-q1o2"
evidence_type: "test"
commit: "83b8292a29776ef203c7f8d4695bf0f4bb99e675"
result: "pass"
captured_at: "2026-10-17T18:46:58.785160595+00:00"
command: "cargo nextest run -j 4 --workspace"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-1mk4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1mk4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass without features"
updated_at: "2026-10-17T18:48:13.405152972+00:00"
---

## Summary

Workspace tests pass without features

## Command

```console
cargo nextest run -j 4 --workspace
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 97393
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1mk4/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.18s
────────────
 Nextest run ID ac1545ed-034f-41c8-88ce-10028dbde987 with nextest profile: default
    Starting 778 tests across 10 binaries
        PASS [   0.108s] (  1/778) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.219s] (  2/778) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.246s] (  3/778) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.059s] (  4/778) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.324s] (  5/778) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.110s] (  6/778) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.006s] (  7/778) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.034s] (  8/778) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.030s] (  9/778) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.051s] ( 10/778) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.131s] ( 11/778) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.144s] ( 12/778) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.054s] ( 13/778) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.530s] ( 14/778) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.084s] ( 15/778) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.255s] ( 16/778) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.105s] ( 17/778) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.053s] ( 18/778) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.068s] ( 19/778) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.262s] ( 20/778) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.097s] ( 21/778) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.069s] ( 22/778) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.054s] ( 23/778) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.067s] ( 24/778) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.031s] ( 25/778) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.045s] ( 26/778) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.101s] ( 27/778) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 28/778) atelier-app init::tests::repository_names_come_from_any_remote
```

