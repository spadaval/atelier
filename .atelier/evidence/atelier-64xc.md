---
created_at: "2026-10-17T18:14:45.078499754+00:00"
id: "atelier-64xc"
evidence_type: "test"
commit: "7f471530d1b6d7a691a44b131b7c3483a0c6764f"
result: "pass"
captured_at: "2026-10-17T18:14:22.742269342+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-sfsq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sfsq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests pass with test-hooks after the base-policy fix, including the policy-weakening land test"
updated_at: "2026-10-17T18:14:45.079181725+00:00"
---

## Summary

Workspace tests pass with test-hooks after the base-policy fix, including the policy-weakening land test

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

Bytes: 97707
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-sfsq/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.23s
────────────
 Nextest run ID 600b1278-db0f-40d2-9e42-35e71b46ad26 with nextest profile: default
    Starting 780 tests across 10 binaries
        PASS [   0.049s] (  1/780) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.122s] (  2/780) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.137s] (  3/780) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.042s] (  4/780) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.075s] (  5/780) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.023s] (  6/780) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.221s] (  7/780) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  8/780) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.025s] (  9/780) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.039s] ( 10/780) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.097s] ( 11/780) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.114s] ( 12/780) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.041s] ( 13/780) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.394s] ( 14/780) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.091s] ( 15/780) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.235s] ( 16/780) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.095s] ( 17/780) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.050s] ( 18/780) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.243s] ( 19/780) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.073s] ( 20/780) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.068s] ( 21/780) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.092s] ( 22/780) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.053s] ( 23/780) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.067s] ( 24/780) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.031s] ( 25/780) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.044s] ( 26/780) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.102s] ( 27/780) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 28/780) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.004s] ( 29/780) atelier-app
```

