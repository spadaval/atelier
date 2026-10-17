---
created_at: "2026-10-17T07:19:47.609114488+00:00"
id: "atelier-3521"
evidence_type: "test"
commit: "0335020323e127a8ebe322e1c5745afdaca722c1"
result: "pass"
captured_at: "2026-10-17T07:19:19.044046937+00:00"
command: "cargo nextest run -j 4"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-jpok"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jpok"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (onwh, qrwr, ydv0) into the branch"
updated_at: "2026-10-17T07:19:47.609802391+00:00"
---

## Summary

Full nextest suite passes after merging master (onwh, qrwr, ydv0) into the branch

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

Bytes: 89342
Truncated: yes

```text
   Compiling atelier-core v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-jpok/crates/atelier-core)
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-jpok/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-jpok/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-jpok/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-jpok/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.81s
────────────
 Nextest run ID 63809ebc-d4c2-4013-b6ef-997ae6456e7f with nextest profile: default
    Starting 710 tests across 10 binaries
        PASS [   0.095s] (  1/710) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.164s] (  2/710) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.202s] (  3/710) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.040s] (  4/710) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.100s] (  5/710) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.277s] (  6/710) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.004s] (  7/710) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.023s] (  8/710) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.027s] (  9/710) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.040s] ( 10/710) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.096s] ( 11/710) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.114s] ( 12/710) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.038s] ( 13/710) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.415s] ( 14/710) atelier-app committed_snapshot::tests::initial_proof_checks_union_config_types_and_index_before_parse_errors
        PASS [   0.133s] ( 15/710) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.273s] ( 16/710) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.104s] ( 17/710) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.049s] ( 18/710) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.281s] ( 19/710) atelier-app git_freshness::tests::native_ancestry_matches_roots_siblings_for_roots_siblings_and_shallow_boundaries
        PASS [   0.070s] ( 20/710) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.075s] ( 21/710) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.102s] ( 22/710) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.053s] ( 23/710) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.031s] ( 24/710) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.071s] ( 25/710) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PAS
```

