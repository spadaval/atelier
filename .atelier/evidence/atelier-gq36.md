---
created_at: "2026-10-17T09:02:45.403765135+00:00"
id: "atelier-gq36"
evidence_type: "test"
commit: "eff128aec8f6e5412a1e7f7aa221167132fc3b6e"
result: "pass"
captured_at: "2026-10-17T09:01:43.750678559+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-0sia"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0sia"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes (747 tests), including four new atelier-0sia tests for parent proof gaps, done-parent readiness, issue link footer on done records, and brief Close Reason hint"
updated_at: "2026-10-17T09:02:45.404426348+00:00"
---

## Summary

Full nextest suite passes (747 tests), including four new atelier-0sia tests for parent proof gaps, done-parent readiness, issue link footer on done records, and brief Close Reason hint

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

Bytes: 93555
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-0sia/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.70s
────────────
 Nextest run ID d01bbe22-0d29-438b-96d0-ec0421fd0760 with nextest profile: default
    Starting 747 tests across 10 binaries
        PASS [   0.072s] (  1/747) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.226s] (  2/747) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.277s] (  3/747) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.071s] (  4/747) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.145s] (  5/747) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.372s] (  6/747) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.006s] (  7/747) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.032s] (  8/747) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.032s] (  9/747) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.052s] ( 10/747) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.128s] ( 11/747) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.149s] ( 12/747) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.059s] ( 13/747) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.641s] ( 14/747) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.135s] ( 15/747) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.343s] ( 16/747) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.119s] ( 17/747) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.065s] ( 18/747) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.331s] ( 19/747) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.102s] ( 20/747) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.104s] ( 21/747) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.129s] ( 22/747) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.071s] ( 23/747) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.086s] ( 24/747) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.040s] ( 25/747) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.057s] ( 26/747) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.130s] ( 27/747) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_changes
        PASS [   0.007s] ( 28/747) atelier-app init::tests::repository_names_come_from_any_remote_
```

