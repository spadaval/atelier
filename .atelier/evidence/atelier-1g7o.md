---
created_at: "2026-10-17T09:54:28.483629543+00:00"
id: "atelier-1g7o"
evidence_type: "test"
commit: "afa81dac08f2f0b809ccf88d6ed72d8d3d084129"
result: "pass"
captured_at: "2026-10-17T09:54:01.243012165+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-4775"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4775"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-fnak)"
updated_at: "2026-10-17T09:54:28.484449713+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-fnak)

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

Bytes: 94671
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-4775/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.72s
────────────
 Nextest run ID 842ad71e-139c-4aed-a346-3f5baacfad5d with nextest profile: default
    Starting 756 tests across 10 binaries
        PASS [   0.062s] (  1/756) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.154s] (  2/756) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.173s] (  3/756) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.054s] (  4/756) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.092s] (  5/756) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.030s] (  6/756) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/756) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.290s] (  8/756) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.029s] (  9/756) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.052s] ( 10/756) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.124s] ( 11/756) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.142s] ( 12/756) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.050s] ( 13/756) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.498s] ( 14/756) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.109s] ( 15/756) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.291s] ( 16/756) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.113s] ( 17/756) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.061s] ( 18/756) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.291s] ( 19/756) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.085s] ( 20/756) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.084s] ( 21/756) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.118s] ( 22/756) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.065s] ( 23/756) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.081s] ( 24/756) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.038s] ( 25/756) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.055s] ( 26/756) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.126s] ( 27/756) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/756) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.006s] ( 29/756) atelier-app
```

