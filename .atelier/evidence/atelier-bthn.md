---
created_at: "2026-10-17T02:52:08.395801263+00:00"
id: "atelier-bthn"
evidence_type: "test"
commit: "dba1c6045a6cb215255b127adc44c5c86e19581a"
result: "pass"
captured_at: "2026-10-17T02:51:28.415464180+00:00"
command: "env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sdb0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sdb0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full Rust suite passes at dba1c604 after atelier-4vkt fix (659 tests)"
updated_at: "2026-10-17T02:52:08.397024235+00:00"
---

## Summary

Full Rust suite passes at dba1c604 after atelier-4vkt fix (659 tests)

## Command

```console
env TMPDIR=/home/supadava/.cache/atelier-tmp cargo nextest run
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 82387
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/mission-atelier-1w8c/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.05s
────────────
 Nextest run ID aef5c30c-025b-4f8e-904b-821854936b31 with nextest profile: default
    Starting 659 tests across 10 binaries
        PASS [   0.033s] (  1/659) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.126s] (  2/659) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.143s] (  3/659) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.235s] (  4/659) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.257s] (  5/659) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.256s] (  6/659) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.387s] (  7/659) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.227s] (  8/659) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.604s] (  9/659) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.633s] ( 10/659) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.430s] ( 11/659) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.711s] ( 12/659) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.766s] ( 13/659) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.307s] ( 14/659) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.686s] ( 15/659) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.545s] ( 16/659) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.143s] ( 17/659) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.354s] ( 18/659) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.303s] ( 19/659) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.017s] ( 20/659) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.420s] ( 21/659) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.016s] ( 22/659) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.021s] ( 23/659) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.012s] ( 24/659) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.014s] ( 25/659) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.021s] ( 26/659) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.182s] ( 27/659) atelier-app git_freshness::tests::working_paths_preserve_staged_unstaged_untracked_and_both_rename_endpoints
        PASS [   0.018s] ( 28/659) atelier-app issue_read::tests::objective_rows_k
```

