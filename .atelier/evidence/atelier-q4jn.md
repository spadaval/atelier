---
created_at: "2026-10-18T07:10:39.271547720+00:00"
id: "atelier-q4jn"
evidence_type: "test"
commit: "002565b649b3b0e03ef24f4212665d13de18165f"
result: "pass"
captured_at: "2026-10-18T07:09:59.859906374+00:00"
command: "env TMPDIR=/home/supadava/.cache/atelier-tmp RUSTC_WRAPPER= cargo nextest run -j 4"
exit_status: "0"
agent_identity: "Claude Opus 5.5 (implementer)"
target:
  kind: "issue"
  id: "atelier-8bil"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8bil"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full suite passes on the reviewed branch, including the 17 debrief tests"
updated_at: "2026-10-18T07:10:39.272014468+00:00"
---

## Summary

Full suite passes on the reviewed branch, including the 17 debrief tests

## Command

```console
env TMPDIR=/home/supadava/.cache/atelier-tmp RUSTC_WRAPPER= cargo nextest run -j 4
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 101652
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
────────────
 Nextest run ID eaaed163-d3e4-4554-81a8-15f04effe1da with nextest profile: default
    Starting 812 tests across 10 binaries
        PASS [   0.097s] (  1/812) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.233s] (  2/812) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.273s] (  3/812) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.065s] (  4/812) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.120s] (  5/812) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.035s] (  6/812) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.394s] (  7/812) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.007s] (  8/812) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.007s] (  9/812) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.047s] ( 10/812) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.068s] ( 11/812) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.166s] ( 12/812) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.192s] ( 13/812) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.066s] ( 14/812) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.649s] ( 15/812) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.140s] ( 16/812) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.368s] ( 17/812) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.146s] ( 18/812) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.081s] ( 19/812) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.390s] ( 20/812) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.112s] ( 21/812) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.112s] ( 22/812) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.149s] ( 23/812) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.091s] ( 24/812) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.126s] ( 25/812) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.047s] ( 26/812) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.069s] ( 27/812) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.162s] ( 28/812) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.007s] ( 29/812) atelier-app init::tests
```

