---
created_at: "2026-10-17T08:21:12.119970218+00:00"
id: "atelier-uxbj"
evidence_type: "test"
commit: "1246633a1643a0e06b8c610f5585470e39383beb"
result: "pass"
captured_at: "2026-10-17T08:20:03.612321014+00:00"
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
title: "Full workspace nextest suite passes on the tree merged with master (krf4, od8b)"
updated_at: "2026-10-17T08:21:12.121044071+00:00"
---

## Summary

Full workspace nextest suite passes on the tree merged with master (krf4, od8b)

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

Bytes: 91423
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-ksx6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.56s
────────────
 Nextest run ID 83bc7280-dd99-47c4-8865-0584fc71869e with nextest profile: default
    Starting 730 tests across 10 binaries
        PASS [   0.073s] (  1/730) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.193s] (  2/730) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.226s] (  3/730) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.071s] (  4/730) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.121s] (  5/730) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.034s] (  6/730) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.349s] (  7/730) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.010s] (  8/730) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.040s] (  9/730) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.064s] ( 10/730) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.143s] ( 11/730) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.172s] ( 12/730) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.057s] ( 13/730) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.612s] ( 14/730) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.132s] ( 15/730) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.336s] ( 16/730) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.130s] ( 17/730) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.074s] ( 18/730) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.340s] ( 19/730) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.099s] ( 20/730) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.098s] ( 21/730) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.137s] ( 22/730) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.074s] ( 23/730) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.044s] ( 24/730) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.099s] ( 25/730) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.067s] ( 26/730) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.146s] ( 27/730) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.007s] ( 28/730) atelier-app init::tests::repository_names_come_from_any_remote_
```

