---
created_at: "2026-10-23T14:34:26.649126245+00:00"
id: "atelier-su46"
evidence_type: "test"
commit: "e7e76fde3096864c0735f705a8c742f5de060599"
result: "pass"
captured_at: "2026-10-23T14:33:38.026872601+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-phfo"
target:
  kind: "issue"
  id: "atelier-phfo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-093h"
    role: "validates"
  - kind: "issue"
    id: "atelier-3u2u"
    role: "validates"
  - kind: "issue"
    id: "atelier-d8ku"
    role: "validates"
  - kind: "issue"
    id: "atelier-h6o4"
    role: "validates"
  - kind: "issue"
    id: "atelier-hulj"
    role: "validates"
  - kind: "issue"
    id: "atelier-jw1n"
    role: "validates"
  - kind: "issue"
    id: "atelier-phfo"
    role: "validates"
  - kind: "issue"
    id: "atelier-uv5d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite with test hooks passes at the implementation commit (900 tests)"
updated_at: "2026-10-23T14:34:32.657081026+00:00"
---

## Summary

Full workspace suite with test hooks passes at the implementation commit (900 tests)

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

Bytes: 113166
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-phfo/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.86s
────────────
 Nextest run ID c1230364-166a-488b-b601-6323fc7f42a2 with nextest profile: default
    Starting 900 tests across 10 binaries
        PASS [   0.060s] (  1/900) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.152s] (  2/900) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.172s] (  3/900) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.051s] (  4/900) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.095s] (  5/900) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.027s] (  6/900) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.007s] (  7/900) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.006s] (  8/900) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.287s] (  9/900) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.032s] ( 10/900) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.055s] ( 11/900) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.124s] ( 12/900) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.147s] ( 13/900) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.495s] ( 14/900) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.049s] ( 15/900) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.110s] ( 16/900) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.289s] ( 17/900) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.112s] ( 18/900) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.062s] ( 19/900) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.295s] ( 20/900) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.086s] ( 21/900) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.702s] ( 22/900) atelier-app git_freshness::tests::ancestry_path_matches_git_when_a_side_branch_is_dated_before_its_parents
        PASS [   0.117s] ( 23/900) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   1.045s] ( 24/900) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.068s] ( 25/900) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.089s] ( 26/900) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.039s] ( 27/900) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.086s] ( 28/900) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.056s] ( 29/900) ateli
```

