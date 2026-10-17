---
created_at: "2026-10-17T09:07:27.161174974+00:00"
id: "atelier-pah8"
evidence_type: "test"
commit: "87e9dc730b834a2bbf1febc6d8553864a9dd9fe0"
result: "pass"
captured_at: "2026-10-17T09:06:54.428887883+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-lr8o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-lr8o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-0sia)"
updated_at: "2026-10-17T09:07:27.161844503+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-0sia)

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
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-lr8o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.85s
────────────
 Nextest run ID 89d06f04-2ac3-4d13-84fa-16d60be85597 with nextest profile: default
    Starting 749 tests across 10 binaries
        PASS [   3.388s] (  1/749) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   4.870s] (  2/749) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   6.411s] (  3/749) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.542s] (  4/749) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   6.441s] (  5/749) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.032s] (  6/749) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.064s] (  7/749) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.006s] (  8/749) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.036s] (  9/749) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.058s] ( 10/749) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.140s] ( 11/749) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.167s] ( 12/749) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.058s] ( 13/749) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   3.527s] ( 14/749) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.170s] ( 15/749) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.404s] ( 16/749) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.156s] ( 17/749) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.096s] ( 18/749) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.098s] ( 19/749) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.351s] ( 20/749) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.073s] ( 21/749) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.093s] ( 22/749) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.053s] ( 23/749) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.067s] ( 24/749) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.031s] ( 25/749) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.042s] ( 26/749) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.101s] ( 27/749) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/749) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/749) atelier-app
```

