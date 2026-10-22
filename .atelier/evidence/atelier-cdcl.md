---
created_at: "2026-10-22T21:15:26.278316603+00:00"
id: "atelier-cdcl"
evidence_type: "test"
commit: "be10bc47ae8604e300b20d1e4fc335e82f789588"
result: "pass"
captured_at: "2026-10-22T21:14:45.343056038+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-4ixu"
target:
  kind: "issue"
  id: "atelier-4ixu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4ixu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test-hooks passes (820/820)"
updated_at: "2026-10-22T21:15:26.279680765+00:00"
---

## Summary

Full workspace suite with test-hooks passes (820/820)

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

Bytes: 102835
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-zuow/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.95s
────────────
 Nextest run ID 6eb09d3c-962a-4730-a4ab-bae18f4cf2fc with nextest profile: default
    Starting 820 tests across 10 binaries
        PASS [   0.866s] (  1/820) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.739s] (  2/820) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   2.528s] (  3/820) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.958s] (  4/820) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.199s] (  5/820) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   2.791s] (  6/820) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.021s] (  7/820) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.107s] (  8/820) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.023s] (  9/820) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.926s] ( 10/820) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   1.366s] ( 11/820) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   1.981s] ( 12/820) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   2.450s] ( 13/820) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.091s] ( 14/820) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.119s] ( 15/820) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   6.001s] ( 16/820) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.137s] ( 17/820) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.081s] ( 18/820) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.115s] ( 19/820) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   4.395s] ( 20/820) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.115s] ( 21/820) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.110s] ( 22/820) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.347s] ( 23/820) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.141s] ( 24/820) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.039s] ( 25/820) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.066s] ( 26/820) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.103s] ( 27/820) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.074s] ( 28/820) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.005s] ( 29/820) ate
```

