---
created_at: "2026-10-23T14:42:35.300981653+00:00"
id: "atelier-7wni"
evidence_type: "test"
commit: "01c60e0fd19a188b0f4ee1d1b4b2a14148455a08"
result: "pass"
captured_at: "2026-10-23T14:41:36.016891329+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-h1nc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2tux"
    role: "validates"
  - kind: "issue"
    id: "atelier-6rh7"
    role: "validates"
  - kind: "issue"
    id: "atelier-9jr8"
    role: "validates"
  - kind: "issue"
    id: "atelier-bvl3"
    role: "validates"
  - kind: "issue"
    id: "atelier-h1nc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "full workspace suite with test hooks passes (898 tests)"
updated_at: "2026-10-23T14:42:40.680547828+00:00"
---

## Summary

full workspace suite with test hooks passes (898 tests)

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

Bytes: 112854
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.71s
────────────
 Nextest run ID 396d490f-c171-47ba-ac5a-09595860cf15 with nextest profile: default
    Starting 898 tests across 10 binaries
        PASS [   0.073s] (  1/898) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.170s] (  2/898) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.184s] (  3/898) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.065s] (  4/898) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.276s] (  5/898) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.111s] (  6/898) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.006s] (  7/898) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.006s] (  8/898) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.040s] (  9/898) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.034s] ( 10/898) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.069s] ( 11/898) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.140s] ( 12/898) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.153s] ( 13/898) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.561s] ( 14/898) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.059s] ( 15/898) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.339s] ( 16/898) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.125s] ( 17/898) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.125s] ( 18/898) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.061s] ( 19/898) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.283s] ( 20/898) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.088s] ( 21/898) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.757s] ( 22/898) atelier-app git_freshness::tests::ancestry_path_matches_git_when_a_side_branch_is_dated_before_its_parents
        PASS [   0.170s] ( 23/898) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.165s] ( 24/898) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.198s] ( 25/898) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.129s] ( 26/898) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.233s] ( 27/898) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.091s] ( 28/898) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   1.559s] ( 29
```

