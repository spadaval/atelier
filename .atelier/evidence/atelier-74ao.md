---
created_at: "2026-10-17T07:05:29.921352613+00:00"
id: "atelier-74ao"
evidence_type: "test"
commit: "b39bd8302457392c3df8a7b135f173759645386e"
result: "pass"
captured_at: "2026-10-17T07:05:11.033797573+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
agent_identity: "orchestrator (Claude Opus 5.5)"
target:
  kind: "issue"
  id: "atelier-ydv0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ydv0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full suite with test hooks after the m4gv and 8ysu fixes and merging master (atelier-ihss, atelier-i2qq, atelier-a76d) with the merge.rs success-line conflict resolved"
updated_at: "2026-10-17T07:05:29.922220350+00:00"
---

## Summary

Full suite with test hooks after the m4gv and 8ysu fixes and merging master (atelier-ihss, atelier-i2qq, atelier-a76d) with the merge.rs success-line conflict resolved

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

Bytes: 87436
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ydv0/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.94s
────────────
 Nextest run ID 1a1afe4f-9d65-49b1-a7d6-f278252e03ac with nextest profile: default
    Starting 697 tests across 10 binaries
        PASS [   0.014s] (  1/697) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.039s] (  2/697) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.057s] (  3/697) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.078s] (  4/697) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.100s] (  5/697) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.150s] (  6/697) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.259s] (  7/697) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.347s] (  8/697) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.375s] (  9/697) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.413s] ( 10/697) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.496s] ( 11/697) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.403s] ( 12/697) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.182s] ( 13/697) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.510s] ( 14/697) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.562s] ( 15/697) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.260s] ( 16/697) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.205s] ( 17/697) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.058s] ( 18/697) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.111s] ( 19/697) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.009s] ( 20/697) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.151s] ( 21/697) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.015s] ( 22/697) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.012s] ( 23/697) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.009s] ( 24/697) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.010s] ( 25/697) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.010s] ( 26/697) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 27/697) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.010s] ( 28/697) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.011s] ( 29/697) atelier-app issue_r
```

