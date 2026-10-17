---
created_at: "2026-10-17T06:16:23.160095951+00:00"
id: "atelier-0d31"
evidence_type: "test"
commit: "792c8d49424aedfa4dafb437d61daed3a3828a26"
result: "pass"
captured_at: "2026-10-17T06:15:05.182194458+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
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
title: "Full suite with test-hooks passes at 792c8d49, including the new one-sided link storage, legacy-storage equivalence, and approval staleness tests"
updated_at: "2026-10-17T06:16:23.161568278+00:00"
---

## Summary

Full suite with test-hooks passes at 792c8d49, including the new one-sided link storage, legacy-storage equivalence, and approval staleness tests

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

Bytes: 86953
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-ihss/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.09s
────────────
 Nextest run ID 8e9fabac-2456-4818-9a23-be0e9e7792e9 with nextest profile: default
    Starting 682 tests across 10 binaries
        PASS [   0.017s] (  1/682) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   1.207s] (  2/682) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   1.965s] (  3/682) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   2.133s] (  4/682) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   2.305s] (  5/682) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   2.593s] (  6/682) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   2.906s] (  7/682) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   1.749s] (  8/682) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   3.891s] (  9/682) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   4.253s] ( 10/682) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   1.447s] ( 11/682) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   2.176s] ( 12/682) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.682s] ( 13/682) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   4.642s] ( 14/682) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   3.870s] ( 15/682) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   6.523s] ( 16/682) atelier-app committed_snapshot::tests::raw_blob_hashes_match_git_for_both_formats_without_filters
        PASS [   2.303s] ( 17/682) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   2.003s] ( 18/682) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   1.616s] ( 19/682) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.010s] ( 20/682) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   6.706s] ( 21/682) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.011s] ( 22/682) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.011s] ( 23/682) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.010s] ( 24/682) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.009s] ( 25/682) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.008s] ( 26/682) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.016s] ( 27/682) atelier-app issue_read::tests::mission_scope_follows_ad
```

