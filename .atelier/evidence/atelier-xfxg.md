---
created_at: "2026-10-16T01:20:16.949517890+00:00"
id: "atelier-xfxg"
evidence_type: "test"
commit: "f68771dc51caf607340777f2c2889eb0e3718622"
result: "fail"
captured_at: "2026-10-16T01:20:04.484725811+00:00"
command: "cargo nextest run --workspace"
exit_status: "100"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-ox63"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ox63"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after review fixes"
updated_at: "2026-10-16T01:20:16.950168745+00:00"
---

## Summary

Full workspace nextest suite passes after review fixes

## Command

```console
cargo nextest run --workspace
```

Exit status: 100

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 98140
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-records)
   Compiling atelier-workflow v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-workflow)
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-core)
   Compiling atelier-app v0.2.0 (/home/supadava/projects/atelier-wt/ox63/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.04s
────────────
 Nextest run ID 32bc150e-95b2-41cc-af0b-0fcce9bd5efb with nextest profile: default
    Starting 610 tests across 10 binaries
        PASS [   0.029s] (  1/610) atelier-app committed_snapshot::tests::sequential_raw_reads_report_hash_errors_before_earlier_mismatches
        PASS [   0.033s] (  2/610) atelier-app evidence_freshness::tests::code_clean_filter_follows_git_filemode_but_keeps_flagged_policy
        PASS [   0.058s] (  3/610) atelier-app committed_snapshot::tests::postraw_index_errors_precede_graph_errors_and_cleanup_restores_graph_error
        PASS [   0.064s] (  4/610) atelier-app git_freshness::tests::attached_working_paths_preserve_captured_head
        PASS [   0.086s] (  5/610) atelier-app base_policy::tests::valid_tip_policy_does_not_require_available_older_ancestry
        PASS [   0.113s] (  6/610) atelier-app committed_snapshot::tests::pinned_commit_fences_refresh_tree_and_filesystem_verification
        PASS [   0.113s] (  7/610) atelier-app git_freshness::tests::captured_first_parent_matches_native_roots_merges_shallow_and_grafts
        PASS [   0.117s] (  8/610) atelier-app git_freshness::tests::native_blob_queries_preserve_multiline_binary_bodies_and_reject_invalid_utf8
        PASS [   0.184s] (  9/610) atelier-app committed_snapshot::tests::canonical_index_fence_checks_stages_modes_and_lossless_paths
        PASS [   0.130s] ( 10/610) atelier-app git_freshness::tests::native_author_fields_preserve_show_encoding_and_unusual_identities
        PASS [   0.126s] ( 11/610) atelier-app git_freshness::tests::native_edges_preserve_tested_merge_parent_and_side_branch_reverts
        PASS [   0.262s] ( 12/610) atelier-app committed_snapshot::tests::fresh_hashes_check_the_last_of_one_thousand_canonical_files
        PASS [   0.292s] ( 13/610) atelier-app evidence_freshness::tests::shared_read_proof_matches_fresh_checks_and_new_context_observes_mutation
        PASS [   0.061s] ( 14/610) atelier-app git_freshness::tests::replacement_refs_cannot_reinterpret_pinned_objects_or_history
        PASS [   0.007s] ( 15/610) atelier-app init::tests::repository_names_come_from_any_remote_or_path_form
        PASS [   0.309s] ( 16/610) atelier-app base_policy::tests::captured_policy_preserves_first_parent_fallback_peeling_and_object_kind
        PASS [   0.128s] ( 17/610) atelier-app git_freshness::tests::range_boundaries_reject_wrong_missing_and_nonancestor_pins
        PASS [   0.008s] ( 18/610) atelier-app issue_inventory::tests::canonical_inventory_is_flat_complete_and_limited_after_id_ordering
        PASS [   0.006s] ( 19/610) atelier-app issue_inventory::tests::zero_limit_and_policy_required_filters_are_rejected_even_for_empty_inventory
        PASS [   0.012s] ( 20/610) atelier-app issue_inventory::tests::exact_filters_use_canonical_labels_and_custom_status_category_with_and_semantics
        PASS [   0.133s] ( 21/610) atelier-app git_freshness::tests::rename_configuration_preserves_git_string_errors_and_valueless_true
        PASS [   0.009s] ( 22/610) atelier-app issue_read::tests::evidence_gate_counts_validating_evidence_without_renderer
        PASS [   0.007s] ( 23/610) atelier-app issue_read::tests::mission_scope_follows_advances_linked_roots_and_descendants
        PASS [   0.007s] ( 24/610) atelier-app issue_read::tests::objective_rows_keep_priority_update_and_identity_order
        PASS [   0.008s] ( 25/610) atelier-app issue_read::tests::objective_scope_buckets_ready_blocked_and_done_children
        PASS [   0.068s]
```

