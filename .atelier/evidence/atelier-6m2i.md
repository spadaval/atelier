---
created_at: "2026-10-17T08:34:17.189501691+00:00"
id: "atelier-6m2i"
evidence_type: "test"
commit: "c4a3db09f51e1d06a04bc0f5aec2db39c63b56ad"
result: "pass"
captured_at: "2026-10-17T08:33:42.239229358+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-ksx6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ksx6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after the atelier-r5n7 and atelier-cd3l fixes, on the tree merged with master"
updated_at: "2026-10-17T08:34:17.190338282+00:00"
---

## Summary

Full workspace nextest suite passes after the atelier-r5n7 and atelier-cd3l fixes, on the tree merged with master

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

Bytes: 91494
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-ksx6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.05s
────────────
 Nextest run ID ebc04295-89b3-4846-ba7e-613c8aa3efdc with nextest profile: default
    Starting 731 tests across 10 binaries
        PASS [   0.048s] (  1/731) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.123s] (  2/731) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.150s] (  3/731) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.043s] (  4/731) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.205s] (  5/731) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.087s] (  6/731) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.005s] (  7/731) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.023s] (  8/731) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.026s] (  9/731) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.041s] ( 10/731) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.098s] ( 11/731) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.113s] ( 12/731) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.041s] ( 13/731) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.466s] ( 14/731) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.123s] ( 15/731) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.281s] ( 16/731) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.101s] ( 17/731) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.068s] ( 18/731) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.295s] ( 19/731) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.140s] ( 20/731) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.070s] ( 21/731) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.094s] ( 22/731) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.054s] ( 23/731) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.069s] ( 24/731) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 25/731) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.046s] ( 26/731) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.102s] ( 27/731) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/731) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/731) atelier-app
```

