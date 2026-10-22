---
created_at: "2026-10-22T21:36:27.332280049+00:00"
id: "atelier-13sr"
evidence_type: "test"
commit: "a04c32246ff993048df8aafdd890b4381c80eaf8"
result: "pass"
captured_at: "2026-10-22T21:35:14.921593031+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-rc8p"
target:
  kind: "issue"
  id: "atelier-rc8p"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rc8p"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes (831/831)"
updated_at: "2026-10-22T21:36:27.334779909+00:00"
---

## Summary

Full workspace suite with test-hooks passes (831/831)

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

Bytes: 104326
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-zuow/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.81s
────────────
 Nextest run ID b47c5f28-268d-4af0-8fcb-1705ff5cb5dc with nextest profile: default
    Starting 831 tests across 10 binaries
        PASS [   0.269s] (  1/831) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.554s] (  2/831) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.677s] (  3/831) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.763s] (  4/831) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.248s] (  5/831) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.427s] (  6/831) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.012s] (  7/831) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.083s] (  8/831) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.020s] (  9/831) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.127s] ( 10/831) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.232s] ( 11/831) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.658s] ( 12/831) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   1.599s] ( 13/831) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   1.731s] ( 14/831) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   4.431s] ( 15/831) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.168s] ( 16/831) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   3.141s] ( 17/831) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.204s] ( 18/831) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.194s] ( 19/831) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.198s] ( 20/831) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   4.641s] ( 21/831) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.716s] ( 22/831) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.134s] ( 23/831) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.147s] ( 24/831) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.246s] ( 25/831) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.045s] ( 26/831) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.104s] ( 27/831) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.069s] ( 28/831) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.008s] ( 29/831) ate
```

