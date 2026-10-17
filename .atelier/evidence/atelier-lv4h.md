---
created_at: "2026-10-17T05:30:47.001406705+00:00"
id: "atelier-lv4h"
evidence_type: "test"
commit: "b32fd8baff82601e272ebe343577d312d7314675"
result: "pass"
captured_at: "2026-10-17T05:30:21.986138710+00:00"
command: "cargo nextest run --features test-hooks"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-kops"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kops"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "test-hooks nextest suite passes"
updated_at: "2026-10-17T05:30:47.002234959+00:00"
---

## Summary

test-hooks nextest suite passes

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

Bytes: 84872
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/.t3/worktrees/atelier/k8h8-kops/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.06s
────────────
 Nextest run ID 1f0e66a1-1bfe-4eb4-9cad-1fe58bc8a026 with nextest profile: default
    Starting 677 tests across 10 binaries
        PASS [   0.011s] (  1/677) atelier-app epic_migration::tests::migration_refuses_to_drop_sections
        PASS [   0.041s] (  2/677) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.071s] (  3/677) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.107s] (  4/677) atelier-app evidence_freshness::tests::epic_migration_is_not_a_new_completion_claim
        PASS [   0.118s] (  5/677) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.121s] (  6/677) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.224s] (  7/677) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.117s] (  8/677) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.272s] (  9/677) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.409s] ( 10/677) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.422s] ( 11/677) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.476s] ( 12/677) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.268s] ( 13/677) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.496s] ( 14/677) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.383s] ( 15/677) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.323s] ( 16/677) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.073s] ( 17/677) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.183s] ( 18/677) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.135s] ( 19/677) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.017s] ( 20/677) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.178s] ( 21/677) atelier-app git_freshness::tests::native_reads_pin_heads_and_preserve_tree_modes_in_both_hash_formats
        PASS [   0.016s] ( 22/677) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.168s] ( 23/677) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.007s] ( 24/677) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.212s] ( 25/677) atelier-app git_freshness::tests::native_reads_keep_exact_ranges_and_reject_bad_pins
        PASS [   0.024s] ( 26/677) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.016s] ( 27/677) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.011s] ( 28/677) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.015s] ( 29/677) ateli
```

