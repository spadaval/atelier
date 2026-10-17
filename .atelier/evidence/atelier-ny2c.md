---
created_at: "2026-10-17T09:02:04.660183092+00:00"
id: "atelier-ny2c"
evidence_type: "test"
commit: "026fd1cfd066c8561650f6398ad8b753b9c99ab6"
result: "pass"
captured_at: "2026-10-17T09:01:30.370749672+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
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
title: "Full nextest suite passes, including the new bundle short/unknown/wrong-kind ID test"
updated_at: "2026-10-17T09:02:04.660979147+00:00"
---

## Summary

Full nextest suite passes, including the new bundle short/unknown/wrong-kind ID test

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

Bytes: 93514
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-lr8o/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-lr8o/crates/atelier-app)
   Compiling atelier-records v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-lr8o/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-lr8o/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-lr8o/crates/atelier-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.20s
────────────
 Nextest run ID 44b5f16d-6600-4126-8b08-5a3519778db2 with nextest profile: default
    Starting 744 tests across 10 binaries
        PASS [   0.052s] (  1/744) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.125s] (  2/744) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.141s] (  3/744) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.043s] (  4/744) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.078s] (  5/744) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.223s] (  6/744) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.022s] (  7/744) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.004s] (  8/744) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.025s] (  9/744) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.040s] ( 10/744) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.095s] ( 11/744) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.114s] ( 12/744) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.039s] ( 13/744) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.398s] ( 14/744) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.107s] ( 15/744) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.256s] ( 16/744) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.092s] ( 17/744) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.052s] ( 18/744) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.284s] ( 19/744) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.093s] ( 20/744) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.066s] ( 21/744) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.092s] ( 22/744) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.053s] ( 23/744) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.067s] ( 24/744) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.030s] ( 25/744) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PAS
```

