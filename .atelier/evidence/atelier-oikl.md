---
created_at: "2026-10-17T07:12:33.893185702+00:00"
id: "atelier-oikl"
evidence_type: "test"
commit: "ab0b9724d7ef9197a7138e2509fa8c216343fe92"
result: "pass"
captured_at: "2026-10-17T07:11:58.603560334+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
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
title: "Full workspace nextest suite passes, including the fixture-repo test that rendered views use one Next footer of label: command lines, print no retired footer headings or Hint: Next: prefix, and label transition commands as Command: lines placed among each option's fields"
updated_at: "2026-10-17T07:12:33.899346244+00:00"
---

## Summary

Full workspace nextest suite passes, including the fixture-repo test that rendered views use one Next footer of label: command lines, print no retired footer headings or Hint: Next: prefix, and label transition commands as Command: lines placed among each option's fields

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

Bytes: 85139
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-onwh/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.20s
────────────
 Nextest run ID 6c8e6d2d-1865-44fc-81f5-26e3d6f5d170 with nextest profile: default
    Starting 678 tests across 10 binaries
        PASS [   0.108s] (  1/678) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.263s] (  2/678) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.302s] (  3/678) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.380s] (  4/678) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.095s] (  5/678) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.139s] (  6/678) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.009s] (  7/678) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.040s] (  8/678) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.046s] (  9/678) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.072s] ( 10/678) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.199s] ( 11/678) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.220s] ( 12/678) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.066s] ( 13/678) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.816s] ( 14/678) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.186s] ( 15/678) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.488s] ( 16/678) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   1.620s] ( 17/678) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   1.602s] ( 18/678) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   7.012s] ( 19/678) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   5.063s] ( 20/678) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   8.234s] ( 21/678) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   2.517s] ( 22/678) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   4.436s] ( 23/678) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        SLOW [> 10.000s] (───────) atelier-app git_freshness::tests::native_edges_match_git_diff_for_rename_configuration_and_weird_paths
        PASS [   1.306s] ( 24/678) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.739s] ( 25/678) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   5.594s] ( 26/678) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.175s] ( 27/678) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
```

