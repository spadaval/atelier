---
created_at: "2026-10-17T07:55:35.904806743+00:00"
id: "atelier-bja9"
evidence_type: "test"
commit: "aa1026b267c95dcb036f82fc69410ec05496422b"
result: "pass"
captured_at: "2026-10-17T07:53:55.711619764+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-6xxb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6xxb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after restoring master's role-error test helper"
updated_at: "2026-10-17T07:55:35.905738512+00:00"
---

## Summary

Full nextest suite passes after restoring master's role-error test helper

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

Bytes: 89942
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-6xxb/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.38s
────────────
 Nextest run ID bd0a74bf-8554-4e6e-bafa-d8768f6d1f05 with nextest profile: default
    Starting 717 tests across 10 binaries
        PASS [   0.079s] (  1/717) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.189s] (  2/717) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.213s] (  3/717) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.145s] (  4/717) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.190s] (  5/717) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.380s] (  6/717) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.006s] (  7/717) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.068s] (  8/717) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.069s] (  9/717) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.067s] ( 10/717) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.214s] ( 11/717) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.280s] ( 12/717) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.150s] ( 13/717) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.936s] ( 14/717) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.179s] ( 15/717) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.571s] ( 16/717) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.133s] ( 17/717) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.086s] ( 18/717) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   2.039s] ( 19/717) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   1.757s] ( 20/717) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.097s] ( 21/717) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.144s] ( 22/717) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   2.890s] ( 23/717) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.080s] ( 24/717) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.045s] ( 25/717) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.102s] ( 26/717) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.065s] ( 27/717) atelier-app git_freshness::tests::working_paths
```

