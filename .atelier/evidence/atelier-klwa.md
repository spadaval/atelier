---
created_at: "2026-10-22T21:10:35.103178277+00:00"
id: "atelier-klwa"
evidence_type: "test"
commit: "d8351d878a66a64355662ebeaa03603495bd8fa8"
result: "pass"
captured_at: "2026-10-22T21:10:06.904562012+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-ecah"
target:
  kind: "issue"
  id: "atelier-ecah"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ecah"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "full workspace suite passes after review fixes"
updated_at: "2026-10-22T21:10:35.103490349+00:00"
---

## Summary

full workspace suite passes after review fixes

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

Bytes: 103355
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.54s
────────────
 Nextest run ID ab5a72ae-c2b3-4a9b-898d-58e861ccc4f1 with nextest profile: default
    Starting 825 tests across 10 binaries
        PASS [   0.053s] (  1/825) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.133s] (  2/825) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.150s] (  3/825) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.046s] (  4/825) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.082s] (  5/825) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.237s] (  6/825) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.025s] (  7/825) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  8/825) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.005s] (  9/825) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.027s] ( 10/825) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.044s] ( 11/825) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.111s] ( 12/825) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.129s] ( 13/825) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.044s] ( 14/825) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.435s] ( 15/825) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.097s] ( 16/825) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.262s] ( 17/825) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.100s] ( 18/825) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.065s] ( 19/825) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.269s] ( 20/825) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.076s] ( 21/825) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.075s] ( 22/825) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.103s] ( 23/825) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.058s] ( 24/825) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.074s] ( 25/825) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.035s] ( 26/825) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.049s] ( 27/825) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.111s] ( 28/825) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 29/825) atelier
```

