---
created_at: "2026-10-17T08:57:09.362511631+00:00"
id: "atelier-pljk"
evidence_type: "test"
commit: "f9831796595f8f16f1633cae62d2b6b1289f7fd6"
result: "pass"
captured_at: "2026-10-17T08:56:46.282332546+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-x68o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-x68o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-jx7g, atelier-6wyv)"
updated_at: "2026-10-17T08:57:09.363175349+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-jx7g, atelier-6wyv)

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

Bytes: 93066
Truncated: yes

```text
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-x68o/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-x68o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.64s
────────────
 Nextest run ID 2dc695e1-6749-4248-a953-81a465d85145 with nextest profile: default
    Starting 743 tests across 10 binaries
        PASS [   0.047s] (  1/743) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.120s] (  2/743) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.137s] (  3/743) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.040s] (  4/743) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.075s] (  5/743) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.021s] (  6/743) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.218s] (  7/743) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  8/743) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.026s] (  9/743) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.039s] ( 10/743) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.094s] ( 11/743) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.114s] ( 12/743) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.039s] ( 13/743) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.391s] ( 14/743) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.086s] ( 15/743) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.230s] ( 16/743) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.090s] ( 17/743) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.051s] ( 18/743) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.375s] ( 19/743) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.204s] ( 20/743) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.066s] ( 21/743) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.091s] ( 22/743) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.051s] ( 23/743) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.068s] ( 24/743) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 25/743) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.042s] ( 26/743) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.097s] ( 27/743) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 28/743) atelier-app init::tests
```

