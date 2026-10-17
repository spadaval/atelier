---
created_at: "2026-10-17T07:11:48.828829738+00:00"
id: "atelier-csbp"
evidence_type: "test"
commit: "3001fd342663d1725402e67849bb993d507ed1c1"
result: "pass"
captured_at: "2026-10-17T07:11:06.581039030+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-qrwr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qrwr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-7iyw) into the branch"
updated_at: "2026-10-17T07:11:48.829955677+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-7iyw) into the branch

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

Bytes: 87416
Truncated: yes

```text
   Compiling atelier-records v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-qrwr/crates/atelier-records)
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-qrwr/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-qrwr/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 12.22s
────────────
 Nextest run ID 1547c8b0-9baf-46d3-986d-5e0beebd9b47 with nextest profile: default
    Starting 697 tests across 10 binaries
        PASS [   2.617s] (  1/697) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   5.129s] (  2/697) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   6.822s] (  3/697) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   7.194s] (  4/697) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.383s] (  5/697) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   2.080s] (  6/697) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.006s] (  7/697) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.036s] (  8/697) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.039s] (  9/697) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.058s] ( 10/697) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.344s] ( 11/697) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   1.628s] ( 12/697) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   2.206s] ( 13/697) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   1.415s] ( 14/697) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   1.043s] ( 15/697) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   7.852s] ( 16/697) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.058s] ( 17/697) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   2.942s] ( 18/697) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.103s] ( 19/697) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.079s] ( 20/697) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.284s] ( 21/697) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.106s] ( 22/697) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.061s] ( 23/697) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.080s] ( 24/697) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.036s] ( 25/697) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.081s] ( 26/697) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.052s] ( 27/697) atelier-app git_freshness::tests::working_paths_preser
```

