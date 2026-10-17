---
created_at: "2026-10-17T09:47:59.733482219+00:00"
id: "atelier-pojk"
evidence_type: "test"
commit: "371ecf05393267ecfacc354cf622589980100aa7"
result: "pass"
captured_at: "2026-10-17T09:47:30.446706694+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-u3vj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u3vj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after the review fixes, including the owner-status Unassigned work: check and the count-free status footer labels"
updated_at: "2026-10-17T09:47:59.734212762+00:00"
---

## Summary

Full nextest suite passes after the review fixes, including the owner-status Unassigned work: check and the count-free status footer labels

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

Bytes: 94598
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-u3vj/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.69s
────────────
 Nextest run ID 77926001-d47d-4fcc-bee8-dee05e0948ee with nextest profile: default
    Starting 755 tests across 10 binaries
        PASS [   0.162s] (  1/755) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.228s] (  2/755) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.273s] (  3/755) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.099s] (  4/755) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.056s] (  5/755) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.346s] (  6/755) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.005s] (  7/755) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.036s] (  8/755) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.032s] (  9/755) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.058s] ( 10/755) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.134s] ( 11/755) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.157s] ( 12/755) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.051s] ( 13/755) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.547s] ( 14/755) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.120s] ( 15/755) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.312s] ( 16/755) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.118s] ( 17/755) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.064s] ( 18/755) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.086s] ( 19/755) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.305s] ( 20/755) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.121s] ( 21/755) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.089s] ( 22/755) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.066s] ( 23/755) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.081s] ( 24/755) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.040s] ( 25/755) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.061s] ( 26/755) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.138s] ( 27/755) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.006s] ( 28/755) atelier-app init::tests::repository_names_come_from_any_remote_
```

