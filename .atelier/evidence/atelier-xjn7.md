---
created_at: "2026-10-17T08:36:21.896442219+00:00"
id: "atelier-xjn7"
evidence_type: "test"
commit: "f9879858bf6c802b47968108ba04f7ea0d5d77ae"
result: "pass"
captured_at: "2026-10-17T08:35:00.150738152+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-ksx6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ksx6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after the review fixes, on the tree merged with master including atelier-pr4k"
updated_at: "2026-10-17T08:36:21.897156862+00:00"
---

## Summary

Full workspace nextest suite passes after the review fixes, on the tree merged with master including atelier-pr4k

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

Bytes: 92167
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-ksx6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.94s
────────────
 Nextest run ID d5066197-2b96-4f03-9d84-93f9b502d520 with nextest profile: default
    Starting 732 tests across 10 binaries
        PASS [   1.258s] (  1/732) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.707s] (  2/732) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   2.785s] (  3/732) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   3.990s] (  4/732) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   2.915s] (  5/732) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   5.746s] (  6/732) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.011s] (  7/732) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.098s] (  8/732) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.492s] (  9/732) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.607s] ( 10/732) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.454s] ( 11/732) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   1.820s] ( 12/732) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   3.789s] ( 13/732) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   1.305s] ( 14/732) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.529s] ( 15/732) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   2.852s] ( 16/732) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [  12.631s] ( 17/732) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   4.428s] ( 18/732) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   7.394s] ( 19/732) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.377s] ( 20/732) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.477s] ( 21/732) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.234s] ( 22/732) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.497s] ( 23/732) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.673s] ( 24/732) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.045s] ( 25/732) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.147s] ( 26/732) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.214s] ( 27/732) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoi
```

