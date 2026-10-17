---
created_at: "2026-10-17T07:14:11.137422918+00:00"
id: "atelier-0zr1"
evidence_type: "test"
commit: "39ca34a92cb39cb483d93384e35e1d56d6242549"
result: "pass"
captured_at: "2026-10-17T07:13:28.425497484+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-jpok"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jpok"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full Rust test suite passes, including tests that -p P1 stores P1, --priority high and P1 filter identically, update --priority P0 stores P0, and a record storing low fails check naming P0-P3"
updated_at: "2026-10-17T07:14:11.138307567+00:00"
---

## Summary

Full Rust test suite passes, including tests that -p P1 stores P1, --priority high and P1 filter identically, update --priority P0 stores P0, and a record storing low fails check naming P0-P3

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

Bytes: 86208
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-jpok/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.55s
────────────
 Nextest run ID d78a509a-3e6d-44fc-98b0-aefabb677f09 with nextest profile: default
    Starting 689 tests across 10 binaries
        PASS [   0.052s] (  1/689) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.137s] (  2/689) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.153s] (  3/689) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.044s] (  4/689) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.079s] (  5/689) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.230s] (  6/689) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  7/689) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.024s] (  8/689) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.026s] (  9/689) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.042s] ( 10/689) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.101s] ( 11/689) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.121s] ( 12/689) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.043s] ( 13/689) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.442s] ( 14/689) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.129s] ( 15/689) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.296s] ( 16/689) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.119s] ( 17/689) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.056s] ( 18/689) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.080s] ( 19/689) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.322s] ( 20/689) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.071s] ( 21/689) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.097s] ( 22/689) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.055s] ( 23/689) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.070s] ( 24/689) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 25/689) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.053s] ( 26/689) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.124s] ( 27/689) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/689) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/689) atelier-app
```

