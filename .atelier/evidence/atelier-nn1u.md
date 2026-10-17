---
created_at: "2026-10-17T08:28:28.854291646+00:00"
id: "atelier-nn1u"
evidence_type: "test"
commit: "2b1898fad7ab1079288a9eb7041d0350007c6d58"
result: "pass"
captured_at: "2026-10-17T08:27:35.229642235+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-91w1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-91w1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after the review fixes, including output_terms (whole-error code count, merge and invalid-policy fixtures) and the every-help-page sweep"
updated_at: "2026-10-17T08:28:28.855052616+00:00"
---

## Summary

Full nextest suite passes after the review fixes, including output_terms (whole-error code count, merge and invalid-policy fixtures) and the every-help-page sweep

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

Bytes: 90933
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-91w1/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.60s
────────────
 Nextest run ID dffc5cba-7ad6-47e9-ab8c-244b7a5544bf with nextest profile: default
    Starting 727 tests across 10 binaries
        PASS [   1.286s] (  1/727) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.308s] (  2/727) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   3.016s] (  3/727) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   1.751s] (  4/727) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.118s] (  5/727) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   3.164s] (  6/727) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.009s] (  7/727) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.051s] (  8/727) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.058s] (  9/727) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.100s] ( 10/727) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.154s] ( 11/727) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.170s] ( 12/727) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.052s] ( 13/727) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   2.354s] ( 14/727) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.146s] ( 15/727) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.319s] ( 16/727) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.137s] ( 17/727) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.092s] ( 18/727) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.098s] ( 19/727) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.370s] ( 20/727) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.168s] ( 21/727) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.121s] ( 22/727) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.084s] ( 23/727) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.054s] ( 24/727) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.106s] ( 25/727) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.062s] ( 26/727) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.139s] ( 27/727) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.007s] ( 28/727) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.007s] ( 29/727) atelier-app
```

