---
created_at: "2026-10-17T08:00:51.940551038+00:00"
id: "atelier-gbtj"
evidence_type: "test"
commit: "80351c43e1efe05683800a22630c05ed01dba640"
result: "pass"
captured_at: "2026-10-17T07:59:23.009514803+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-krf4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-krf4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-6xxb)"
updated_at: "2026-10-17T08:00:51.941368002+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-6xxb)

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

Bytes: 91462
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-krf4/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.57s
────────────
 Nextest run ID fd891ec7-bfad-4c5b-b318-50f28a1417e6 with nextest profile: default
    Starting 721 tests across 10 binaries
        PASS [   2.307s] (  1/721) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   3.471s] (  2/721) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   5.812s] (  3/721) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   2.848s] (  4/721) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   7.691s] (  5/721) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   1.882s] (  6/721) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.005s] (  7/721) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.138s] (  8/721) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.352s] (  9/721) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.809s] ( 10/721) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   2.072s] ( 11/721) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   3.514s] ( 12/721) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   5.932s] ( 13/721) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.331s] ( 14/721) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   3.635s] ( 15/721) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   1.236s] ( 16/721) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   7.046s] ( 17/721) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [  15.098s] ( 18/721) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.615s] ( 19/721) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   7.219s] ( 20/721) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   2.406s] ( 21/721) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   2.556s] ( 22/721) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   6.241s] ( 23/721) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   4.773s] ( 24/721) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   2.977s] ( 25/721) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   2.384s] ( 26/721) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        SLOW [> 10.000s] (───────) atelier-app git_freshness::test
```

