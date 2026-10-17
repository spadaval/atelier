---
created_at: "2026-10-17T07:03:05.232539345+00:00"
id: "atelier-ligt"
evidence_type: "test"
commit: "029058e7f2bfa3698f20d586fb6b944b24b45865"
result: "pass"
captured_at: "2026-10-17T07:02:30.239630315+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
agent_identity: "orchestrator (Claude Opus 5.5)"
target:
  kind: "issue"
  id: "atelier-ihss"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ihss"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full suite with test hooks after the clean merge of master that brought in atelier-a76d"
updated_at: "2026-10-17T07:03:05.233786840+00:00"
---

## Summary

Full suite with test hooks after the clean merge of master that brought in atelier-a76d

## Command

```console
cargo nextest run --features test-hooks
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 86779
Truncated: yes

```text
   Compiling chrono v0.4.45
   Compiling tempfile v3.27.0
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ihss/crates/atelier-cli)
   Compiling rusty-fork v0.3.1
   Compiling proptest v1.9.0
   Compiling atelier-core v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ihss/crates/atelier-core)
   Compiling atelier-workflow v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ihss/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ihss/crates/atelier-records)
   Compiling atelier-app v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ihss/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 21.83s
────────────
 Nextest run ID bd50c221-acab-44d8-89e2-9afea5aaaa82 with nextest profile: default
    Starting 688 tests across 10 binaries
        PASS [   0.009s] (  1/688) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.042s] (  2/688) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.067s] (  3/688) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.094s] (  4/688) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.095s] (  5/688) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.110s] (  6/688) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.148s] (  7/688) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.090s] (  8/688) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.219s] (  9/688) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.249s] ( 10/688) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.179s] ( 11/688) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.242s] ( 12/688) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.102s] ( 13/688) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.292s] ( 14/688) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.200s] ( 15/688) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.109s] ( 16/688) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.150s] ( 17/688) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.056s] ( 18/688) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.134s] ( 19/688) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.013s] ( 20/688) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.434s] ( 21/688) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.009s] ( 22/688) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.015s] ( 23/688) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.013s] ( 24/688) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
```

