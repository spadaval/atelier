---
created_at: "2026-10-22T23:17:44.981045968+00:00"
id: "atelier-g71v"
evidence_type: "test"
commit: "ff01dd845de3ca6eda97a9c0bef22cf948df486e"
result: "pass"
captured_at: "2026-10-22T23:16:49.484936144+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-fngz"
target:
  kind: "issue"
  id: "atelier-1hjj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1hjj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes on the branch (all four fngz items)"
updated_at: "2026-10-22T23:17:44.981521084+00:00"
---

## Summary

Full workspace suite with test-hooks passes on the branch (all four fngz items)

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

Bytes: 110435
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.92s
────────────
 Nextest run ID abcaac48-70d4-4657-8389-04d78834de92 with nextest profile: default
    Starting 876 tests across 10 binaries
        PASS [   0.115s] (  1/876) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.236s] (  2/876) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.292s] (  3/876) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.099s] (  4/876) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.177s] (  5/876) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.446s] (  6/876) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.016s] (  7/876) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.014s] (  8/876) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.065s] (  9/876) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.064s] ( 10/876) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.090s] ( 11/876) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.226s] ( 12/876) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.200s] ( 13/876) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.072s] ( 14/876) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.755s] ( 15/876) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.199s] ( 16/876) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.203s] ( 17/876) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.526s] ( 18/876) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.082s] ( 19/876) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.515s] ( 20/876) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.120s] ( 21/876) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.159s] ( 22/876) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.130s] ( 23/876) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.085s] ( 24/876) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.049s] ( 25/876) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.146s] ( 26/876) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.092s] ( 27/876) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.167s] ( 28/876) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type
```

