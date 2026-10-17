---
created_at: "2026-10-17T07:05:45.349225731+00:00"
id: "atelier-cj67"
evidence_type: "test"
commit: "d49333ba374f1b0a682809852bf57a961a73a7c6"
result: "pass"
captured_at: "2026-10-17T07:04:53.993293883+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-7iyw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7iyw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after the review fixes and a master merge"
updated_at: "2026-10-17T07:05:45.349946494+00:00"
---

## Summary

Full nextest suite passes after the review fixes and a master merge

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

Bytes: 86566
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling chrono v0.4.45
   Compiling tempfile v3.27.0
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-7iyw/crates/atelier-cli)
   Compiling rusty-fork v0.3.1
   Compiling proptest v1.9.0
   Compiling atelier-core v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-7iyw/crates/atelier-core)
   Compiling atelier-workflow v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-7iyw/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-7iyw/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-7iyw/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 12.26s
────────────
 Nextest run ID 1097fb18-1fa0-4ead-bb58-5e7eeb8e4720 with nextest profile: default
    Starting 687 tests across 10 binaries
        PASS [   0.640s] (  1/687) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.062s] (  2/687) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   1.147s] (  3/687) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.101s] (  4/687) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.177s] (  5/687) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.046s] (  6/687) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.005s] (  7/687) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.033s] (  8/687) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.030s] (  9/687) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.048s] ( 10/687) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.106s] ( 11/687) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.126s] ( 12/687) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.435s] ( 13/687) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.289s] ( 14/687) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.145s] ( 15/687) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.801s] ( 16/687) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.187s] ( 17/687) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.108s] ( 18/687) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.101s] ( 19/687) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.443s] ( 20/687) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.098s] ( 21/687) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.137s] ( 22/687) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.065s] ( 23/687) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.083s] ( 24/687) atelier-app git_freshness::tests::rename_configuration_preserves_
```

