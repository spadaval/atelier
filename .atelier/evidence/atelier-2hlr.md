---
created_at: "2026-10-17T18:46:58.515333775+00:00"
id: "atelier-2hlr"
evidence_type: "test"
commit: "83b8292a29776ef203c7f8d4695bf0f4bb99e675"
result: "pass"
captured_at: "2026-10-17T18:45:56.773568886+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-1mk4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1mk4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass with test-hooks, including the issue show and status decision tests"
updated_at: "2026-10-17T18:46:58.516025206+00:00"
---

## Summary

Workspace tests pass with test-hooks, including the issue show and status decision tests

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 97998
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-1mk4/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.07s
────────────
 Nextest run ID d412d52e-f0bd-4a4c-8cfb-01b75f92b164 with nextest profile: default
    Starting 782 tests across 10 binaries
        PASS [   0.087s] (  1/782) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.307s] (  2/782) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.362s] (  3/782) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.069s] (  4/782) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.145s] (  5/782) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.052s] (  6/782) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/782) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.527s] (  8/782) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.058s] (  9/782) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.116s] ( 10/782) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.203s] ( 11/782) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.241s] ( 12/782) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.094s] ( 13/782) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.990s] ( 14/782) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.205s] ( 15/782) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.501s] ( 16/782) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.150s] ( 17/782) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.122s] ( 18/782) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.115s] ( 19/782) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.441s] ( 20/782) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.114s] ( 21/782) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.200s] ( 22/782) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.121s] ( 23/782) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.055s] ( 24/782) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.134s] ( 25/782) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   1.420s] ( 26/782) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.090s] ( 27/782) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.005s] ( 28/782) atelier-app init::tests::repository_names_come_from_any_re
```

