---
created_at: "2026-10-17T07:14:18.279111850+00:00"
id: "atelier-437y"
evidence_type: "test"
commit: "c42c2470df0fa4537aab06e3e4e64bef3b4fdfca"
result: "pass"
captured_at: "2026-10-17T07:13:30.040645478+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-onwh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-onwh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (a76d, 7iyw) with resolved import and test-append conflicts"
updated_at: "2026-10-17T07:14:18.280298184+00:00"
---

## Summary

Full nextest suite passes after merging master (a76d, 7iyw) with resolved import and test-append conflicts

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

Bytes: 86705
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling chrono v0.4.45
   Compiling tempfile v3.27.0
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-onwh/crates/atelier-cli)
   Compiling rusty-fork v0.3.1
   Compiling proptest v1.9.0
   Compiling atelier-core v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-onwh/crates/atelier-core)
   Compiling atelier-workflow v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-onwh/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-onwh/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-onwh/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 12.08s
────────────
 Nextest run ID 890db607-20aa-498e-9a5c-a7bf2cfedb4d with nextest profile: default
    Starting 688 tests across 10 binaries
        PASS [   2.406s] (  1/688) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.301s] (  2/688) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   4.249s] (  3/688) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.091s] (  4/688) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.117s] (  5/688) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.613s] (  6/688) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.007s] (  7/688) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   6.003s] (  8/688) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.460s] (  9/688) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.536s] ( 10/688) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.215s] ( 11/688) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.179s] ( 12/688) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.056s] ( 13/688) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.143s] ( 14/688) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   4.535s] ( 15/688) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.350s] ( 16/688) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.144s] ( 17/688) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.113s] ( 18/688) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.088s] ( 19/688) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.370s] ( 20/688) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.148s] ( 21/688) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   2.991s] ( 22/688) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.100s] ( 23/688) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.040s] ( 24/688) atelier-app git_freshness::tests::replacement_refs_cannot_re
```

