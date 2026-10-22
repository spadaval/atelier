---
created_at: "2026-10-22T21:28:13.534103688+00:00"
id: "atelier-15mb"
evidence_type: "test"
commit: "3f36a1461ce1a4c6c21744587c0fb23ed33d5c46"
result: "pass"
captured_at: "2026-10-22T21:27:37.366550658+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-n642-uw4x"
target:
  kind: "issue"
  id: "atelier-n642"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n642"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test hooks passes (819/819)"
updated_at: "2026-10-22T21:28:13.534419767+00:00"
---

## Summary

Full workspace suite with test hooks passes (819/819)

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

Bytes: 102628
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1ztv/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.84s
────────────
 Nextest run ID 1318d8df-f6cb-47de-b931-338d1d186a43 with nextest profile: default
    Starting 819 tests across 10 binaries
        PASS [   1.647s] (  1/819) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.883s] (  2/819) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   3.135s] (  3/819) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.278s] (  4/819) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   3.178s] (  5/819) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.047s] (  6/819) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.006s] (  7/819) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.005s] (  8/819) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.024s] (  9/819) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.030s] ( 10/819) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.048s] ( 11/819) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.108s] ( 12/819) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.129s] ( 13/819) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.045s] ( 14/819) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.883s] ( 15/819) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.120s] ( 16/819) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.281s] ( 17/819) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.101s] ( 18/819) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.066s] ( 19/819) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.077s] ( 20/819) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.261s] ( 21/819) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.106s] ( 22/819) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.077s] ( 23/819) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.060s] ( 24/819) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.077s] ( 25/819) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.035s] ( 26/819) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.054s] ( 27/819) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.122s] ( 28/819) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/819) atelier
```

