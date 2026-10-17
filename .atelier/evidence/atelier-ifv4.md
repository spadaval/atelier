---
created_at: "2026-10-17T09:50:47.936935646+00:00"
id: "atelier-ifv4"
evidence_type: "test"
commit: "240d0da82fae210cb9d044ad3acc623565d8a79d"
result: "pass"
captured_at: "2026-10-17T09:50:24.889504336+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-4775"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4775"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after merging master, including tests pinning argument-only transitions as allowed in brief show, full Transition Readiness, transition options and work parent readiness"
updated_at: "2026-10-17T09:50:47.937633899+00:00"
---

## Summary

Full workspace nextest suite passes after merging master, including tests pinning argument-only transitions as allowed in brief show, full Transition Readiness, transition options and work parent readiness

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

Bytes: 94671
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-4775/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.50s
────────────
 Nextest run ID 3f9f6fb4-db48-4ace-82c9-fd88bb12475a with nextest profile: default
    Starting 756 tests across 10 binaries
        PASS [   0.050s] (  1/756) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.120s] (  2/756) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.138s] (  3/756) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.041s] (  4/756) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.075s] (  5/756) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.022s] (  6/756) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/756) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.224s] (  8/756) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.024s] (  9/756) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.042s] ( 10/756) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.097s] ( 11/756) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.114s] ( 12/756) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.040s] ( 13/756) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.395s] ( 14/756) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.084s] ( 15/756) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.232s] ( 16/756) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.095s] ( 17/756) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.049s] ( 18/756) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.235s] ( 19/756) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.070s] ( 20/756) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.070s] ( 21/756) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.096s] ( 22/756) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.053s] ( 23/756) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.069s] ( 24/756) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.032s] ( 25/756) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.045s] ( 26/756) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.107s] ( 27/756) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.005s] ( 28/756) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.064s] ( 29/756) atelier-app
```

