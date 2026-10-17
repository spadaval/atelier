---
created_at: "2026-10-17T18:08:08.989093280+00:00"
id: "atelier-hk3k"
evidence_type: "test"
commit: "2c6fe5a30e9b8e841fa24926baa3124dde3589dd"
result: "pass"
captured_at: "2026-10-17T18:07:18.527530515+00:00"
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
title: "Workspace tests pass with test-hooks after the review fixes, including the scoped-child land test"
updated_at: "2026-10-17T18:08:08.989771247+00:00"
---

## Summary

Workspace tests pass with test-hooks after the review fixes, including the scoped-child land test

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

Bytes: 97583
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-sfsq/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.30s
────────────
 Nextest run ID ff385104-8bfb-482c-8e19-9622b882a78d with nextest profile: default
    Starting 779 tests across 10 binaries
        PASS [   0.049s] (  1/779) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.145s] (  2/779) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.166s] (  3/779) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.065s] (  4/779) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.113s] (  5/779) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.036s] (  6/779) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.007s] (  7/779) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.302s] (  8/779) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.159s] (  9/779) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.181s] ( 10/779) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.279s] ( 11/779) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.317s] ( 12/779) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.204s] ( 13/779) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.135s] ( 14/779) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.445s] ( 15/779) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.858s] ( 16/779) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.193s] ( 17/779) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.497s] ( 18/779) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.110s] ( 19/779) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.250s] ( 20/779) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.288s] ( 21/779) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.138s] ( 22/779) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.164s] ( 23/779) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.069s] ( 24/779) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   2.006s] ( 25/779) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.193s] ( 26/779) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.130s] ( 27/779) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.006s] ( 28/779) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.006s] ( 29/779) atelier
```

