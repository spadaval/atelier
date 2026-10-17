---
created_at: "2026-10-17T09:47:56.840696657+00:00"
id: "atelier-xl86"
evidence_type: "test"
commit: "a8389f10570cecd6153bfa823037f9a6c505d01d"
result: "pass"
captured_at: "2026-10-17T09:47:29.692000622+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-pyfr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-pyfr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full suite passes, including every_invalid_value_takes_the_shared_form_and_exits_2 covering link/unlink --role, --issue-type on create/update/list, unknown transitions, --limit 0 and abc, create --status, priority, evidence --result and target kinds"
updated_at: "2026-10-17T09:47:56.841443661+00:00"
---

## Summary

Full suite passes, including every_invalid_value_takes_the_shared_form_and_exits_2 covering link/unlink --role, --issue-type on create/update/list, unknown transitions, --limit 0 and abc, create --status, priority, evidence --result and target kinds

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

Bytes: 94669
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-pyfr/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.30s
────────────
 Nextest run ID 8c46f305-e72d-4442-a9fa-e748aa0602f4 with nextest profile: default
    Starting 756 tests across 10 binaries
        PASS [   0.050s] (  1/756) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.118s] (  2/756) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.137s] (  3/756) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.041s] (  4/756) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.078s] (  5/756) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.023s] (  6/756) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/756) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.224s] (  8/756) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.025s] (  9/756) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.040s] ( 10/756) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.101s] ( 11/756) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.117s] ( 12/756) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.039s] ( 13/756) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.402s] ( 14/756) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.086s] ( 15/756) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.236s] ( 16/756) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.092s] ( 17/756) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.049s] ( 18/756) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.240s] ( 19/756) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.070s] ( 20/756) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.069s] ( 21/756) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.094s] ( 22/756) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.052s] ( 23/756) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.067s] ( 24/756) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.030s] ( 25/756) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.044s] ( 26/756) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.099s] ( 27/756) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 28/756) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/756) atelier-app
```

