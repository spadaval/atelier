---
created_at: "2026-07-06T18:31:24.332887451+00:00"
id: "atelier-q6d9"
evidence_type: "test"
captured_at: "2026-07-06T18:31:19.839340054+00:00"
command: "cargo nextest run -p atelier-app -p atelier-cli cache_"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-5yd6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5yd6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Cache-dependent command inventory, lazy repair, decision safety, and degraded orientation"
updated_at: "2026-07-06T18:31:28.789720229+00:00"
---

## Summary

Cache-dependent command inventory, lazy repair, decision safety, and degraded orientation

## Command

```console
cargo nextest run -p atelier-app -p atelier-cli cache_
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 3071
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-x7lq/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/root/.codex/worktrees/mska-x7lq/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.15s
────────────
 Nextest run ID 2b1eafa3-1560-44dc-985c-710fb0077d24 with nextest profile: default
    Starting 19 tests across 5 binaries (549 tests skipped)
        PASS [   0.010s] ( 1/19) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.012s] ( 2/19) atelier-app use_cases::tests::cache_command_inventory_limits_degraded_reads_to_orientation
        PASS [   0.013s] ( 3/19) atelier-cli::bin/atelier cache_acquisition_tests::central_dispatch_has_no_raw_database_open_bypass
        PASS [   0.011s] ( 4/19) atelier-cli::bin/atelier cache_acquisition_tests::cache_dependent_query_families_use_named_app_accessors
        PASS [   0.032s] ( 5/19) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.139s] ( 6/19) atelier-app cache_manager::tests::corrupt_cache_is_discarded_and_rebuilt
        PASS [   0.144s] ( 7/19) atelier-app cache_manager::tests::missing_cache_rebuilds_only_when_queried
        PASS [   0.147s] ( 8/19) atelier-app cache_manager::tests::orientation_can_degrade_but_decision_queries_reject_known_stale_rows
        PASS [   0.273s] ( 9/19) atelier-app cache_manager::tests::missing_source_metadata_falls_back_to_full_rebuild
        PASS [   0.288s] (10/19) atelier-app cache_manager::tests::bounded_changed_record_requests_incremental_then_falls_back_safely
        PASS [   0.296s] (11/19) atelier-app cache_manager::tests::version_mismatch_is_discarded_and_rebuilt
        PASS [   0.466s] (12/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_query_rebuilds_missing_cache_on_demand
        PASS [   0.468s] (13/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_decision_query_never_returns_known_stale_rows
        PASS [   0.479s] (14/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_orientation_names_degraded_last_good_state
        PASS [   0.594s] (15/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_query_distinguishes_schema_drift_from_malformed_records
        PASS [   0.621s] (16/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_rebuilds_changed_sources_before_issue_queries
        PASS [   0.874s] (17/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_repairs_deleted_and_unindexed_sources_before_issue_queries
        PASS [   1.025s] (18/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_rebuilds_dep_list_and_lint_but_ignores_derived_files
        PASS [   2.032s] (19/19) atelier-cli::cli_integration mission_projection_worktree::test_cache_bounds_many_changed_sources_and_rebuilds
────────────
     Summary [   2.051s] 19 tests run: 19 passed, 549 skipped
```
