---
created_at: "2026-10-17T08:11:36.280277054+00:00"
id: "atelier-1u40"
evidence_type: "test"
commit: "0b6e052b363f244f330597be7a9e2c0c524e656b"
result: "pass"
captured_at: "2026-10-17T08:10:49.711082332+00:00"
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
title: "Full nextest suite passes, including the output_terms grep test over rendered check, issue show, prune, help and land output"
updated_at: "2026-10-17T08:11:36.281708766+00:00"
---

## Summary

Full nextest suite passes, including the output_terms grep test over rendered check, issue show, prune, help and land output

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

Bytes: 90879
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-91w1/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.07s
────────────
 Nextest run ID a042695a-ca2b-4768-8e0a-b915bc2a7bba with nextest profile: default
    Starting 719 tests across 10 binaries
        PASS [   3.007s] (  1/719) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   8.405s] (  2/719) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        SLOW [> 10.000s] (───────) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [  11.799s] (  3/719) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [  12.144s] (  4/719) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   3.789s] (  5/719) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        SLOW [> 10.000s] (───────) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.868s] (  6/719) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.005s] (  7/719) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   1.269s] (  8/719) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.177s] (  9/719) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.319s] ( 10/719) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.310s] ( 11/719) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.225s] ( 12/719) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.056s] ( 13/719) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.156s] ( 14/719) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [  10.848s] ( 15/719) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.366s] ( 16/719) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.107s] ( 17/719) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.064s] ( 18/719) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.075s] ( 19/719) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.268s] ( 20/719) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   1.979s] ( 21/719) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   0.112s] ( 22/719) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.085s] ( 23/719) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.064s] ( 24/719) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.038s] ( 25/719) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_p
```

