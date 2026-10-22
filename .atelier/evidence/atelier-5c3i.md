---
created_at: "2026-10-22T22:43:05.504510354+00:00"
id: "atelier-5c3i"
evidence_type: "test"
commit: "1e5098895b49dc7f65e74a604b03b6da9bdaa57a"
result: "pass"
captured_at: "2026-10-22T22:42:19.491293516+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-r28p"
target:
  kind: "issue"
  id: "atelier-ss25"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ss25"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test hooks passes after the review fixes"
updated_at: "2026-10-22T22:43:05.504783753+00:00"
---

## Summary

Full workspace suite with test hooks passes after the review fixes

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

Bytes: 108598
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.06s
────────────
 Nextest run ID 008aa99b-c259-4540-b59e-2a767c87389d with nextest profile: default
    Starting 863 tests across 10 binaries
        PASS [   0.126s] (  1/863) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.262s] (  2/863) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.341s] (  3/863) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.081s] (  4/863) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.452s] (  5/863) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.232s] (  6/863) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.045s] (  7/863) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.010s] (  8/863) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.017s] (  9/863) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.089s] ( 10/863) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.085s] ( 11/863) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.266s] ( 12/863) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.330s] ( 13/863) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.084s] ( 14/863) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.085s] ( 15/863) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.241s] ( 16/863) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.289s] ( 17/863) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.706s] ( 18/863) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.132s] ( 19/863) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.268s] ( 20/863) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.798s] ( 21/863) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.144s] ( 22/863) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.217s] ( 23/863) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.142s] ( 24/863) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.152s] ( 25/863) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.101s] ( 26/863) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.150s] ( 27/863) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   2.126s] ( 28/863) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.007s] ( 29/863) ate
```

