---
created_at: "2026-10-17T08:15:25.214005997+00:00"
id: "atelier-lcqj"
evidence_type: "test"
commit: "33f0a635571223489f1ad5318750a449b6c844ae"
result: "pass"
captured_at: "2026-10-17T08:14:07.147056881+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-91w1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-91w1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes on the master merge (krf4, od8b), including output_terms and krf4's count tests"
updated_at: "2026-10-17T08:15:25.216211903+00:00"
---

## Summary

Full nextest suite passes on the master merge (krf4, od8b), including output_terms and krf4's count tests

## Command

```console
cargo nextest run -j 4
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 91197
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-91w1/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.08s
────────────
 Nextest run ID af1fe6da-af0a-4015-aab5-58c2d37c55c6 with nextest profile: default
    Starting 725 tests across 10 binaries
        PASS [   2.195s] (  1/725) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   2.946s] (  2/725) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   6.946s] (  3/725) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   4.754s] (  4/725) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   7.885s] (  5/725) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.967s] (  6/725) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.006s] (  7/725) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.035s] (  8/725) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.039s] (  9/725) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.061s] ( 10/725) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.145s] ( 11/725) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.169s] ( 12/725) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.058s] ( 13/725) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   6.118s] ( 14/725) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.125s] ( 15/725) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.337s] ( 16/725) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.119s] ( 17/725) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.052s] ( 18/725) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.070s] ( 19/725) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.457s] ( 20/725) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.264s] ( 21/725) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.067s] ( 22/725) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.053s] ( 23/725) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.067s] ( 24/725) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.031s] ( 25/725) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.045s] ( 26/725) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.100s] ( 27/725) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 28/725) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/725) atelier-app
```

