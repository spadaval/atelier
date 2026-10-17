---
created_at: "2026-10-17T07:32:55.314236198+00:00"
id: "atelier-dhfs"
evidence_type: "test"
commit: "7a55dcce89a901b9c724e4649f86cefdad6a9761"
result: "pass"
captured_at: "2026-10-17T07:32:33.281914158+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-6xxb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6xxb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes, including test_hints_name_real_ids_instead_of_placeholders (status, prime, work, issue list empty and non-empty, issue show brief and --full, hierarchy and update errors render real IDs and no <...id>/<transition> placeholder)"
updated_at: "2026-10-17T07:32:55.314936103+00:00"
---

## Summary

Full nextest suite passes, including test_hints_name_real_ids_instead_of_placeholders (status, prime, work, issue list empty and non-empty, issue show brief and --full, hierarchy and update errors render real IDs and no <...id>/<transition> placeholder)

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

Bytes: 89051
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-6xxb/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.92s
────────────
 Nextest run ID 5d0047ce-89eb-4b95-84da-82b01eeef6e6 with nextest profile: default
    Starting 711 tests across 10 binaries
        PASS [   0.049s] (  1/711) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.124s] (  2/711) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.138s] (  3/711) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.042s] (  4/711) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.087s] (  5/711) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.231s] (  6/711) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.025s] (  7/711) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  8/711) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.026s] (  9/711) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.040s] ( 10/711) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.098s] ( 11/711) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.120s] ( 12/711) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.042s] ( 13/711) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.411s] ( 14/711) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.134s] ( 15/711) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.287s] ( 16/711) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.102s] ( 17/711) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.073s] ( 18/711) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.278s] ( 19/711) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.069s] ( 20/711) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.094s] ( 21/711) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.071s] ( 22/711) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.054s] ( 23/711) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.031s] ( 24/711) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.069s] ( 25/711) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.048s] ( 26/711) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.106s] ( 27/711) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.004s] ( 28/711) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.005s] ( 29/711) atelier-app
```

