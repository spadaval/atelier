---
created_at: "2026-10-17T09:06:24.396917705+00:00"
id: "atelier-bo7c"
evidence_type: "test"
commit: "9649c971aeb4733afe66546054d53b3d7afa2dd5"
result: "pass"
captured_at: "2026-10-17T09:06:01.271197658+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-f4ko"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-f4ko"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes (745 tests), including new CLI tests for invalid status, invalid issue_type and a missing review target naming the record file once"
updated_at: "2026-10-17T09:06:24.397637057+00:00"
---

## Summary

Full nextest suite passes (745 tests), including new CLI tests for invalid status, invalid issue_type and a missing review target naming the record file once

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

Bytes: 93227
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-f4ko/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.66s
────────────
 Nextest run ID 6423fec8-761f-4ba7-888a-eaf693097323 with nextest profile: default
    Starting 745 tests across 10 binaries
        PASS [   0.049s] (  1/745) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.121s] (  2/745) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.138s] (  3/745) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.041s] (  4/745) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.076s] (  5/745) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.022s] (  6/745) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.223s] (  7/745) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  8/745) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.024s] (  9/745) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.038s] ( 10/745) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.095s] ( 11/745) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.113s] ( 12/745) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.038s] ( 13/745) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.396s] ( 14/745) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.089s] ( 15/745) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.229s] ( 16/745) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.095s] ( 17/745) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.050s] ( 18/745) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.239s] ( 19/745) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.069s] ( 20/745) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.068s] ( 21/745) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.095s] ( 22/745) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.053s] ( 23/745) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.033s] ( 24/745) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.067s] ( 25/745) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.044s] ( 26/745) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.099s] ( 27/745) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/745) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.004s] ( 29/745) atelier-app
```

