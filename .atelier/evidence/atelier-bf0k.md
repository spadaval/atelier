---
created_at: "2026-10-17T07:18:05.508000085+00:00"
id: "atelier-bf0k"
evidence_type: "test"
commit: "d0488aaef6700f1b3dfb2f123fa4528ea803658c"
result: "pass"
captured_at: "2026-10-17T07:17:35.431888826+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-qrwr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qrwr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-onwh) into the branch"
updated_at: "2026-10-17T07:18:05.508759021+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-onwh) into the branch

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

Bytes: 88781
Truncated: yes

```text
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-qrwr/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-qrwr/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.63s
────────────
 Nextest run ID fffc1150-17b1-40a6-9efc-3fbb535a59db with nextest profile: default
    Starting 708 tests across 10 binaries
        PASS [   0.661s] (  1/708) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.685s] (  2/708) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.717s] (  3/708) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.738s] (  4/708) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.042s] (  5/708) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.077s] (  6/708) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.004s] (  7/708) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.023s] (  8/708) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.025s] (  9/708) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.070s] ( 10/708) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.278s] ( 11/708) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.358s] ( 12/708) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.086s] ( 13/708) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.775s] ( 14/708) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.147s] ( 15/708) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.452s] ( 16/708) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.126s] ( 17/708) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.074s] ( 18/708) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.097s] ( 19/708) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.326s] ( 20/708) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.127s] ( 21/708) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.092s] ( 22/708) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.073s] ( 23/708) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.042s] ( 24/708) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.088s] ( 25/708) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.062s] ( 26/708) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.136s] ( 27/708) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/708) atelier-app init::tests
```

