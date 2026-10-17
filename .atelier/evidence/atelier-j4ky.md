---
created_at: "2026-10-17T06:25:52.135681958+00:00"
id: "atelier-j4ky"
evidence_type: "test"
commit: "e8402f1edfeaaac3e729d4b1addd95ffaafe84c8"
result: "pass"
captured_at: "2026-10-17T06:25:24.075432201+00:00"
command: "cargo nextest run"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-5rs0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5rs0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes with SIGPIPE restored, including the closed-stdout test"
updated_at: "2026-10-17T06:25:52.137035684+00:00"
---

## Summary

Full nextest suite passes with SIGPIPE restored, including the closed-stdout test

## Command

```console
cargo nextest run
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 84406
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-5rs0/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.87s
────────────
 Nextest run ID fe39a9db-9b59-46a4-acd1-5656fdad6019 with nextest profile: default
    Starting 674 tests across 10 binaries
        PASS [   0.035s] (  1/674) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.153s] (  2/674) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.210s] (  3/674) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.275s] (  4/674) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.288s] (  5/674) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.294s] (  6/674) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.210s] (  7/674) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.491s] (  8/674) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.588s] (  9/674) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.629s] ( 10/674) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.422s] ( 11/674) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.258s] ( 12/674) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.975s] ( 13/674) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.885s] ( 14/674) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.674s] ( 15/674) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   1.170s] ( 16/674) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.549s] ( 17/674) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.279s] ( 18/674) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.156s] ( 19/674) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.584s] ( 20/674) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.029s] ( 21/674) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.021s] ( 22/674) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.018s] ( 23/674) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.033s] ( 24/674) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.211s] ( 25/674) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.018s] ( 26/674) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.361s] ( 27/674) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.030s] ( 28/674) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [
```

