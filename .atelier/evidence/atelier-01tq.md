---
created_at: "2026-10-22T20:40:48.635484018+00:00"
id: "atelier-01tq"
evidence_type: "test"
commit: "8ef209697fd5a28a9241c8fadf66a1452368839b"
result: "pass"
captured_at: "2026-10-22T20:39:58.536517329+00:00"
command: "cargo nextest run -j4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-ftqn"
target:
  kind: "issue"
  id: "atelier-ftqn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ftqn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes with test-hooks (816/816)"
updated_at: "2026-10-22T20:40:48.635782655+00:00"
---

## Summary

Full workspace suite passes with test-hooks (816/816)

## Command

```console
cargo nextest run -j4 --workspace --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 102333
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-9g48/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.25s
────────────
 Nextest run ID 240bedef-6c1e-4d11-9a71-a166cfaf28c8 with nextest profile: default
    Starting 816 tests across 10 binaries
        PASS [   0.187s] (  1/816) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.442s] (  2/816) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.549s] (  3/816) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.578s] (  4/816) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.166s] (  5/816) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.288s] (  6/816) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.014s] (  7/816) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.019s] (  8/816) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.063s] (  9/816) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.089s] ( 10/816) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.140s] ( 11/816) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.395s] ( 12/816) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.452s] ( 13/816) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.178s] ( 14/816) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.566s] ( 15/816) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.357s] ( 16/816) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.864s] ( 17/816) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.381s] ( 18/816) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.214s] ( 19/816) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   1.968s] ( 20/816) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.285s] ( 21/816) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.950s] ( 22/816) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.317s] ( 23/816) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.294s] ( 24/816) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.231s] ( 25/816) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.118s] ( 26/816) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.262s] ( 27/816) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.151s] ( 28/816) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_
```

