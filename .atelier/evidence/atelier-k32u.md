---
created_at: "2026-10-17T18:51:58.219854171+00:00"
id: "atelier-k32u"
evidence_type: "test"
commit: "f07f14b0a4348efe39ab0cf3222e046e281c3130"
result: "pass"
captured_at: "2026-10-17T18:51:13.951214606+00:00"
command: "cargo nextest run -j 4 --workspace"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-ho7o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ho7o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass without features after storing criteria_mission"
updated_at: "2026-10-17T18:51:58.221066744+00:00"
---

## Summary

Workspace tests pass without features after storing criteria_mission

## Command

```console
cargo nextest run -j 4 --workspace
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 98724
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-ho7o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.30s
────────────
 Nextest run ID c4933093-f5dc-4d21-b1bf-e84415adaf85 with nextest profile: default
    Starting 789 tests across 10 binaries
        PASS [   0.333s] (  1/789) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.647s] (  2/789) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.796s] (  3/789) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.811s] (  4/789) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.199s] (  5/789) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.407s] (  6/789) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.018s] (  7/789) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.015s] (  8/789) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.111s] (  9/789) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.104s] ( 10/789) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.150s] ( 11/789) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.303s] ( 12/789) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.277s] ( 13/789) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.098s] ( 14/789) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.380s] ( 15/789) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.189s] ( 16/789) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.449s] ( 17/789) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.158s] ( 18/789) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.076s] ( 19/789) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.107s] ( 20/789) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.439s] ( 21/789) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.095s] ( 22/789) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.140s] ( 23/789) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.068s] ( 24/789) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.076s] ( 25/789) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.033s] ( 26/789) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.045s] ( 27/789) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.100s] ( 28/789) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 29/789) atelier
```

