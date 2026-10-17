---
created_at: "2026-10-17T08:04:15.713958258+00:00"
id: "atelier-qv6n"
evidence_type: "test"
commit: "782e27b524cf45099c8fb089d4cee11db5371602"
result: "pass"
captured_at: "2026-10-17T08:03:13.859775619+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
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
title: "Full nextest suite passes after merging master (krf4, 6xxb) with bare work's dashboard conflict resolved to od8b's generated help"
updated_at: "2026-10-17T08:04:15.714674675+00:00"
---

## Summary

Full nextest suite passes after merging master (krf4, 6xxb) with bare work's dashboard conflict resolved to od8b's generated help

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

Bytes: 90686
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-od8b/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.04s
────────────
 Nextest run ID eb83d9a7-5e8c-4f57-8350-0bc9664179e0 with nextest profile: default
    Starting 723 tests across 10 binaries
        PASS [   4.804s] (  1/723) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   5.181s] (  2/723) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   5.232s] (  3/723) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.078s] (  4/723) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   5.261s] (  5/723) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.042s] (  6/723) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.004s] (  7/723) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.022s] (  8/723) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.025s] (  9/723) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.041s] ( 10/723) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.098s] ( 11/723) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.119s] ( 12/723) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.041s] ( 13/723) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.750s] ( 14/723) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.097s] ( 15/723) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.249s] ( 16/723) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.125s] ( 17/723) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.056s] ( 18/723) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.068s] ( 19/723) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.271s] ( 20/723) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.097s] ( 21/723) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.072s] ( 22/723) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.054s] ( 23/723) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.032s] ( 24/723) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.070s] ( 25/723) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.047s] ( 26/723) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.105s] ( 27/723) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/723) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.006s] ( 29/723) atelier-app
```

