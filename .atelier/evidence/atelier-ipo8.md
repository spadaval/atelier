---
created_at: "2026-10-17T09:14:36.657202212+00:00"
id: "atelier-ipo8"
evidence_type: "test"
commit: "855d2f6dda0aaea28022e86edf3ae9eb70ea0051"
result: "pass"
captured_at: "2026-10-17T09:14:11.378341899+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-2hh8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2hh8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes on Linux with the Apple-only SO_NOSIGPIPE setup in place"
updated_at: "2026-10-17T09:14:36.657950368+00:00"
---

## Summary

Full workspace nextest suite passes on Linux with the Apple-only SO_NOSIGPIPE setup in place

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

Bytes: 93770
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-2hh8/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.73s
────────────
 Nextest run ID 1acf0492-b591-4df5-93f6-041eeab8b010 with nextest profile: default
    Starting 749 tests across 10 binaries
        PASS [   0.117s] (  1/749) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.192s] (  2/749) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.233s] (  3/749) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.046s] (  4/749) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.099s] (  5/749) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.023s] (  6/749) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.316s] (  7/749) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  8/749) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.027s] (  9/749) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.042s] ( 10/749) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.105s] ( 11/749) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.122s] ( 12/749) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.043s] ( 13/749) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.438s] ( 14/749) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.100s] ( 15/749) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.264s] ( 16/749) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.098s] ( 17/749) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.053s] ( 18/749) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.255s] ( 19/749) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.074s] ( 20/749) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.100s] ( 21/749) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.073s] ( 22/749) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.055s] ( 23/749) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.072s] ( 24/749) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 25/749) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.047s] ( 26/749) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.110s] ( 27/749) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/749) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/749) atelier-app
```

