---
created_at: "2026-10-22T22:34:00.092770621+00:00"
id: "atelier-3ng9"
evidence_type: "test"
commit: "12ffb6f0af9dc064a123b4c405102faa0c991255"
result: "pass"
captured_at: "2026-10-22T22:33:11.110552091+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer-s2ei"
target:
  kind: "issue"
  id: "atelier-s2ei"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-s2ei"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes (869/869) with test-hooks after the review fixes"
updated_at: "2026-10-22T22:34:00.093284769+00:00"
---

## Summary

Full workspace suite passes (869/869) with test-hooks after the review fixes

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

Bytes: 109400
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-s2ei/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.19s
────────────
 Nextest run ID 0e20ed18-1c67-45e6-80a5-515fa3631c29 with nextest profile: default
    Starting 869 tests across 10 binaries
        PASS [   0.064s] (  1/869) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.182s] (  2/869) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.188s] (  3/869) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.050s] (  4/869) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.255s] (  5/869) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.096s] (  6/869) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.027s] (  7/869) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.006s] (  8/869) atelier-app criteria::tests::this_repositorys_strategies_parse_without_problems
        PASS [   0.007s] (  9/869) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.029s] ( 10/869) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.050s] ( 11/869) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.160s] ( 12/869) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.181s] ( 13/869) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.051s] ( 14/869) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.541s] ( 15/869) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.113s] ( 16/869) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.337s] ( 17/869) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.134s] ( 18/869) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.093s] ( 19/869) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.343s] ( 20/869) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.094s] ( 21/869) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.153s] ( 22/869) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.105s] ( 23/869) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.073s] ( 24/869) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.093s] ( 25/869) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.041s] ( 26/869) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.083s] ( 27/869) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.159s] ( 28/869) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.007s] ( 29/869) atelier
```

