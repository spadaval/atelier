---
created_at: "2026-07-06T19:01:52.890798059+00:00"
id: "atelier-poa1"
evidence_type: "test"
captured_at: "2026-07-06T19:01:50.253822174+00:00"
command: "cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli -E 'test(/cache::tests|cache_manager::tests|rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains|rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner|rebuild::tests::cross_domain_graph_change_requests_one_safe_full_rebuild|rebuild::tests::issue_repair_preserves_inbound_relations|rebuild::tests::parse_failure_leaves_rows_and_source_metadata_unchanged|rebuild::tests::full_rebuild_replaces_schema_mismatch_as_disposable_state|test_lint_validates_canonical_markdown_even_when_projection_metadata_is_fresh|test_lint_repairs_unindexed_issue_before_indexed_rules_run/)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-x7lq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-x7lq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli -E 'test(/cache::tests|cache_manager::tests|rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains|rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner|rebuild::tests::cross_domain_graph_change_requests_one_safe_full_rebuild|rebuild::tests::issue_repair_preserves_inbound_relations|rebuild::tests::parse_failure_leaves_rows_and_source_metadata_unchanged|rebuild::tests::full_rebuild_replaces_schema_mismatch_as_disposable_state|test_lint_validates_canonical_markdown_even_when_projection_metadata_is_fresh|test_lint_repairs_unindexed_issue_before_indexed_rules_run/)'"
updated_at: "2026-07-06T19:01:56.663866313+00:00"
---

## Summary

cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli -E 'test(/cache::tests|cache_manager::tests|rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains|rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner|rebuild::tests::cross_domain_graph_change_requests_one_safe_full_rebuild|rebuild::tests::issue_repair_preserves_inbound_relations|rebuild::tests::parse_failure_leaves_rows_and_source_metadata_unchanged|rebuild::tests::full_rebuild_replaces_schema_mismatch_as_disposable_state|test_lint_validates_canonical_markdown_even_when_projection_metadata_is_fresh|test_lint_repairs_unindexed_issue_before_indexed_rules_run/)'

## Command

```console
cargo nextest run -p atelier-sqlite -p atelier-app -p atelier-cli -E 'test(/cache::tests|cache_manager::tests|rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains|rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner|rebuild::tests::cross_domain_graph_change_requests_one_safe_full_rebuild|rebuild::tests::issue_repair_preserves_inbound_relations|rebuild::tests::parse_failure_leaves_rows_and_source_metadata_unchanged|rebuild::tests::full_rebuild_replaces_schema_mismatch_as_disposable_state|test_lint_validates_canonical_markdown_even_when_projection_metadata_is_fresh|test_lint_repairs_unindexed_issue_before_indexed_rules_run/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 3337
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-x7lq/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.91s
────────────
 Nextest run ID e726e77c-2000-495a-a00e-8cec85e303ce with nextest profile: default
    Starting 24 tests across 6 binaries (544 tests skipped)
        PASS [   0.009s] ( 1/24) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.018s] ( 2/24) atelier-sqlite cache::tests::foreign_or_unversioned_layout_is_classified_for_rebuild
        PASS [   0.021s] ( 3/24) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.045s] ( 4/24) atelier-sqlite cache::tests::incompatible_cache_version_is_classified_without_migration
        PASS [   0.120s] ( 5/24) atelier-app cache_manager::tests::corrupt_cache_is_discarded_and_rebuilt
        PASS [   0.126s] ( 6/24) atelier-app cache_manager::tests::orientation_can_degrade_but_decision_queries_reject_known_stale_rows
        PASS [   0.127s] ( 7/24) atelier-app cache_manager::tests::missing_cache_rebuilds_only_when_queried
        PASS [   0.143s] ( 8/24) atelier-sqlite cache::tests::schema_has_no_body_or_generic_payload_columns
        PASS [   0.144s] ( 9/24) atelier-sqlite cache::tests::new_database_has_only_domain_tables_and_explicit_indexes
        PASS [   0.147s] (10/24) atelier-app rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner
        PASS [   0.152s] (11/24) atelier-app cache_manager::tests::bounded_changed_record_is_repaired_incrementally
        PASS [   0.133s] (12/24) atelier-sqlite cache::tests::evidence_rows_add_replace_reverse_query_and_delete_atomically
        PASS [   0.154s] (13/24) atelier-sqlite cache::tests::review_room_rows_add_replace_query_and_delete_atomically
        PASS [   0.158s] (14/24) atelier-app rebuild::tests::parse_failure_leaves_rows_and_source_metadata_unchanged
        PASS [   0.162s] (15/24) atelier-sqlite cache::tests::issue_rows_add_replace_query_and_delete_atomically
        PASS [   0.179s] (16/24) atelier-app rebuild::tests::issue_repair_preserves_inbound_relations
        PASS [   0.209s] (17/24) atelier-app cache_manager::tests::application_id_mismatch_is_discarded_and_rebuilt
        PASS [   0.212s] (18/24) atelier-app cache_manager::tests::version_mismatch_is_discarded_and_rebuilt
        PASS [   0.225s] (19/24) atelier-app rebuild::tests::cross_domain_graph_change_requests_one_safe_full_rebuild
        PASS [   0.231s] (20/24) atelier-app cache_manager::tests::missing_source_metadata_falls_back_to_full_rebuild
        PASS [   0.232s] (21/24) atelier-app rebuild::tests::full_rebuild_replaces_schema_mismatch_as_disposable_state
        PASS [   0.247s] (22/24) atelier-cli::cli_integration mission_projection_worktree::test_lint_repairs_unindexed_issue_before_indexed_rules_run
        PASS [   0.231s] (23/24) atelier-cli::cli_integration mission_projection_worktree::test_lint_validates_canonical_markdown_even_when_projection_metadata_is_fresh
        PASS [   0.359s] (24/24) atelier-app rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains
────────────
     Summary [   0.360s] 24 tests run: 24 passed, 544 skipped
```
