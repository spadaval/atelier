---
created_at: "2026-10-17T08:55:02.364892565+00:00"
id: "atelier-nuve"
evidence_type: "test"
commit: "8d98633d2cedf3586f72ed3860aa161f980bc05f"
result: "pass"
captured_at: "2026-10-17T08:54:02.060077282+00:00"
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
title: "Full Rust suite passes after the atelier-gt0n fix, including tests that bad priority, invalid YAML and broken review records name their .atelier path once in issue list and check"
updated_at: "2026-10-17T08:55:02.365546544+00:00"
---

## Summary

Full Rust suite passes after the atelier-gt0n fix, including tests that bad priority, invalid YAML and broken review records name their .atelier path once in issue list and check

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

Bytes: 92674
Truncated: yes

```text
   Compiling atelier-records v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-x68o/crates/atelier-records)
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-x68o/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-x68o/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.43s
────────────
 Nextest run ID db34b289-ed1c-4238-af3b-48c3b4a4ed99 with nextest profile: default
    Starting 739 tests across 10 binaries
        PASS [   0.062s] (  1/739) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.144s] (  2/739) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.168s] (  3/739) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.051s] (  4/739) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.094s] (  5/739) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.031s] (  6/739) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.006s] (  7/739) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.289s] (  8/739) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.031s] (  9/739) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.049s] ( 10/739) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.115s] ( 11/739) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.152s] ( 12/739) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.048s] ( 13/739) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.494s] ( 14/739) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.146s] ( 15/739) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.277s] ( 16/739) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.065s] ( 17/739) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.110s] ( 18/739) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.301s] ( 19/739) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.102s] ( 20/739) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.119s] ( 21/739) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.084s] ( 22/739) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.069s] ( 23/739) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.089s] ( 24/739) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.041s] ( 25/739) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.056s] ( 26/739) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.150s] ( 27/739) atelier-app git_freshness::tests::tree_diff_
```

