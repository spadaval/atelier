---
created_at: "2026-10-17T08:48:04.581603734+00:00"
id: "atelier-pkje"
evidence_type: "test"
commit: "4a02534f2712b30b458eb549aaabd07040ffc0ef"
result: "pass"
captured_at: "2026-10-17T08:47:10.337277615+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-x68o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-x68o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full Rust suite passes, including new tests pinning that priority and invalid-YAML record-load errors name the path once and that a path-less inner error gains it"
updated_at: "2026-10-17T08:48:04.582951508+00:00"
---

## Summary

Full Rust suite passes, including new tests pinning that priority and invalid-YAML record-load errors name the path once and that a path-less inner error gains it

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

Bytes: 92626
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-x68o/crates/atelier-app)
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-x68o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.72s
────────────
 Nextest run ID d2119c0a-bf69-4553-b3e9-26debd7e7f42 with nextest profile: default
    Starting 739 tests across 10 binaries
        PASS [   1.645s] (  1/739) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   1.650s] (  2/739) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   1.737s] (  3/739) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.098s] (  4/739) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.763s] (  5/739) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.025s] (  6/739) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.052s] (  7/739) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.005s] (  8/739) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.027s] (  9/739) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.046s] ( 10/739) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.109s] ( 11/739) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.130s] ( 12/739) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.054s] ( 13/739) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.128s] ( 14/739) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.508s] ( 15/739) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.339s] ( 16/739) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.184s] ( 17/739) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.120s] ( 18/739) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.080s] ( 19/739) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.428s] ( 20/739) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.168s] ( 21/739) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.075s] ( 22/739) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.058s] ( 23/739) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.034s] ( 24/739) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.075s] ( 25/739) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.049s] ( 26/739) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.113s] ( 27/739) atelier-app git_freshness::tests::tree_diff_matches_git_diff_for_nested_type_and_mode_change
```

