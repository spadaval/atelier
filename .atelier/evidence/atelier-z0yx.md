---
created_at: "2026-10-17T19:24:33.546606866+00:00"
id: "atelier-z0yx"
evidence_type: "test"
commit: "3df0cdd82f5732dddf96243f89c73de69ee7bf57"
result: "pass"
captured_at: "2026-10-17T19:23:29.778415101+00:00"
command: "cargo nextest run -j 4 --workspace"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-rx22"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rx22"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass without features"
updated_at: "2026-10-17T19:24:33.547456411+00:00"
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

Bytes: 99468
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-rx22/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.09s
────────────
 Nextest run ID eb1a68b7-092e-4cca-b6c4-f535b6503ba9 with nextest profile: default
    Starting 795 tests across 10 binaries
        PASS [   1.625s] (  1/795) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.371s] (  2/795) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   3.608s] (  3/795) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.400s] (  4/795) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.236s] (  5/795) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   3.864s] (  6/795) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.020s] (  7/795) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.019s] (  8/795) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.093s] (  9/795) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.137s] ( 10/795) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.208s] ( 11/795) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.449s] ( 12/795) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.564s] ( 13/795) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.201s] ( 14/795) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   3.593s] ( 15/795) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.364s] ( 16/795) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   1.123s] ( 17/795) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.504s] ( 18/795) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.277s] ( 19/795) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.322s] ( 20/795) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   1.142s] ( 21/795) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.527s] ( 22/795) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.604s] ( 23/795) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   3.223s] ( 24/795) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.130s] ( 25/795) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.413s] ( 26/795) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.255s] ( 27/795) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.110s] ( 28/795) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.008s] ( 29/795) ate
```

