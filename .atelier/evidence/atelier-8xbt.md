---
created_at: "2026-10-17T07:59:03.310879848+00:00"
id: "atelier-8xbt"
evidence_type: "test"
commit: "e3410d8938e9569dea51d3c3f0db1352432e47f3"
result: "pass"
captured_at: "2026-10-17T07:58:30.649354398+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-od8b"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-od8b"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes, including the root help, bare work and every-visible-subcommand-listed tests"
updated_at: "2026-10-17T07:59:03.311716218+00:00"
---

## Summary

Full workspace nextest suite passes, including the root help, bare work and every-visible-subcommand-listed tests

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

Bytes: 89851
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-od8b/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.37s
────────────
 Nextest run ID 08929aaa-60ab-42df-a01e-595fd6c837d7 with nextest profile: default
    Starting 718 tests across 10 binaries
        PASS [   0.064s] (  1/718) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.140s] (  2/718) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.160s] (  3/718) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.049s] (  4/718) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.083s] (  5/718) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.240s] (  6/718) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  7/718) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.024s] (  8/718) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.028s] (  9/718) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.045s] ( 10/718) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.106s] ( 11/718) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.128s] ( 12/718) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.055s] ( 13/718) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.434s] ( 14/718) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.113s] ( 15/718) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.294s] ( 16/718) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.097s] ( 17/718) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.062s] ( 18/718) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.266s] ( 19/718) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.082s] ( 20/718) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.101s] ( 21/718) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.071s] ( 22/718) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.057s] ( 23/718) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.032s] ( 24/718) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.083s] ( 25/718) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.047s] ( 26/718) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.107s] ( 27/718) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/718) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/718) atelier-app
```

