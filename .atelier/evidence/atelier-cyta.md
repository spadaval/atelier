---
created_at: "2026-10-17T07:00:46.825174551+00:00"
id: "atelier-cyta"
evidence_type: "test"
commit: "a8642178e9c9fddc501e5f288af448de9ed55e6e"
result: "pass"
captured_at: "2026-10-17T07:00:17.995359713+00:00"
command: "cargo nextest run"
exit_status: "0"
agent_identity: "claude-orchestrator"
target:
  kind: "issue"
  id: "atelier-a76d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a76d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full nextest suite passes after merging master (atelier-5rs0's SIGPIPE change) into the branch"
updated_at: "2026-10-17T07:00:46.826062155+00:00"
---

## Summary

Full nextest suite passes after merging master (atelier-5rs0's SIGPIPE change) into the branch

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

Bytes: 84977
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/o3lm-a76d/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.30s
────────────
 Nextest run ID 6860ca29-d9a6-4991-8cd0-d8a87139fc3c with nextest profile: default
    Starting 679 tests across 10 binaries
        PASS [   0.013s] (  1/679) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.048s] (  2/679) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.064s] (  3/679) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.141s] (  4/679) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.149s] (  5/679) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.162s] (  6/679) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.224s] (  7/679) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.113s] (  8/679) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.305s] (  9/679) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.320s] ( 10/679) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.256s] ( 11/679) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.406s] ( 12/679) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.161s] ( 13/679) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.375s] ( 14/679) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.247s] ( 15/679) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.499s] ( 16/679) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.211s] ( 17/679) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.167s] ( 18/679) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.121s] ( 19/679) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.016s] ( 20/679) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.195s] ( 21/679) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.021s] ( 22/679) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.225s] ( 23/679) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.020s] ( 24/679) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.008s] ( 25/679) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.015s] ( 26/679) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.008s] ( 27/679) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.014s] ( 28/679) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.014s] ( 29/679) ate
```

