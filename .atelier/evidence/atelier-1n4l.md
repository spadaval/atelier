---
created_at: "2026-10-22T23:08:27.603690101+00:00"
id: "atelier-1n4l"
evidence_type: "test"
commit: "c5833bba72a3097a6c409f32903b490b43ccbb49"
result: "pass"
captured_at: "2026-10-22T23:07:45.223258870+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-7b6e"
target:
  kind: "issue"
  id: "atelier-7b6e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7b6e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test hooks passes"
updated_at: "2026-10-22T23:08:27.604166379+00:00"
---

## Summary

Full workspace suite with test hooks passes

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

Bytes: 109580
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-7b6e/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.72s
────────────
 Nextest run ID 0e925779-5699-49c6-92a1-dae9f3162d10 with nextest profile: default
    Starting 872 tests across 10 binaries
        PASS [   0.079s] (  1/872) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.303s] (  2/872) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.385s] (  3/872) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.486s] (  4/872) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.122s] (  5/872) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.245s] (  6/872) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.008s] (  7/872) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.008s] (  8/872) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.058s] (  9/872) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.044s] ( 10/872) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.070s] ( 11/872) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.158s] ( 12/872) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.181s] ( 13/872) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.831s] ( 14/872) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.060s] ( 15/872) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.135s] ( 16/872) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.357s] ( 17/872) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.138s] ( 18/872) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.092s] ( 19/872) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.386s] ( 20/872) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.111s] ( 21/872) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.854s] ( 22/872) atelier-app git_freshness::tests::ancestry_path_matches_git_when_a_side_branch_is_dated_before_its_parents
        PASS [   0.142s] ( 23/872) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.102s] ( 24/872) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.074s] ( 25/872) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.044s] ( 26/872) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.095s] ( 27/872) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.064s] ( 28/872) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.656s] ( 29
```

