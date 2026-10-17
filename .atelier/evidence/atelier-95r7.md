---
created_at: "2026-10-17T07:28:50.800770580+00:00"
id: "atelier-95r7"
evidence_type: "test"
commit: "5490a8bed2e5a0ac11061ff222ac8066e16bf3b7"
result: "pass"
captured_at: "2026-10-17T07:28:03.578273902+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-krf4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-krf4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes, including the count-phrasing unit tests, the no-(s) source test and the updated rendering tests"
updated_at: "2026-10-17T07:28:50.801753216+00:00"
---

## Summary

Full workspace nextest suite passes, including the count-phrasing unit tests, the no-(s) source test and the updated rendering tests

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

Bytes: 86432
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-krf4/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.58s
────────────
 Nextest run ID 843aae4d-4ec5-4f38-8076-a75aba2669b9 with nextest profile: default
    Starting 691 tests across 10 binaries
        PASS [   2.140s] (  1/691) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   2.164s] (  2/691) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   2.260s] (  3/691) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.129s] (  4/691) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   2.328s] (  5/691) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.085s] (  6/691) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.008s] (  7/691) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.035s] (  8/691) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.056s] (  9/691) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.075s] ( 10/691) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.243s] ( 11/691) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.254s] ( 12/691) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.068s] ( 13/691) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.195s] ( 14/691) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.882s] ( 15/691) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.439s] ( 16/691) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.108s] ( 17/691) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.178s] ( 18/691) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.117s] ( 19/691) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.395s] ( 20/691) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.158s] ( 21/691) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.132s] ( 22/691) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.098s] ( 23/691) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.114s] ( 24/691) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.059s] ( 25/691) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.065s] ( 26/691) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   1.429s] ( 27/691) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.005s] ( 28/691) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 29/691) atelier-
```

