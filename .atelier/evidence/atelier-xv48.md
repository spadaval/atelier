---
created_at: "2026-10-17T07:40:45.377856284+00:00"
id: "atelier-xv48"
evidence_type: "test"
commit: "1a9d6b9cf37f6f15fc5ec87ff0fde763791ea431"
result: "pass"
captured_at: "2026-10-17T07:34:56.785974265+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-7w1g"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7w1g"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace suite passes after the review fixes, including the fixed-value usage-error test and the EvidenceResult/FindingDecision ALL round-trip tests"
updated_at: "2026-10-17T07:40:45.379096049+00:00"
---

## Summary

Full workspace suite passes after the review fixes, including the fixed-value usage-error test and the EvidenceResult/FindingDecision ALL round-trip tests

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

Bytes: 89657
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-7w1g/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.82s
────────────
 Nextest run ID 7d4f2500-6dda-4faf-9356-d631096f3061 with nextest profile: default
    Starting 716 tests across 10 binaries
        PASS [   0.070s] (  1/716) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.168s] (  2/716) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.183s] (  3/716) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.054s] (  4/716) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.090s] (  5/716) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.285s] (  6/716) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.033s] (  7/716) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.007s] (  8/716) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.063s] (  9/716) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.102s] ( 10/716) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   1.338s] ( 11/716) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   3.907s] ( 12/716) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.846s] ( 13/716) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   4.959s] ( 14/716) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   5.246s] ( 15/716) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.173s] ( 16/716) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.097s] ( 17/716) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   3.783s] ( 18/716) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.170s] ( 19/716) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.142s] ( 20/716) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.457s] ( 21/716) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.183s] ( 22/716) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.115s] ( 23/716) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.089s] ( 24/716) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.136s] ( 25/716) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.081s] ( 26/716) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.083s] ( 27/716) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.011s] ( 28/716) atelier-app init::tests::repository_names_come_from_any_rem
```

