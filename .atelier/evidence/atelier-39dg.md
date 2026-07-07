---
created_at: "2026-07-06T18:19:08.476062542+00:00"
id: "atelier-39dg"
evidence_type: "test"
captured_at: "2026-07-06T18:19:06.437696658+00:00"
command: "cargo nextest run -p atelier-app cache_manager"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-2jse"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2jse"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "CacheManager lazy freshness, disposable-cache rebuild, and stale-read safety scenarios"
updated_at: "2026-07-06T18:19:12.313580133+00:00"
---

## Summary

CacheManager lazy freshness, disposable-cache rebuild, and stale-read safety scenarios

## Command

```console
cargo nextest run -p atelier-app cache_manager
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1371
Truncated: no

```text
   Compiling atelier-app v0.2.0 (/root/.codex/worktrees/mska-x7lq/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.45s
────────────
 Nextest run ID d7d8150d-1dd5-4b2a-b43d-85b1d39d6d10 with nextest profile: default
    Starting 8 tests across 1 binary (103 tests skipped)
        PASS [   0.010s] (1/8) atelier-app cache_manager::tests::discovery_and_health_inspection_do_not_create_cache
        PASS [   0.011s] (2/8) atelier-app cache_manager::tests::schema_drift_detection_covers_config_and_workflow_errors
        PASS [   0.186s] (3/8) atelier-app cache_manager::tests::missing_cache_rebuilds_only_when_queried
        PASS [   0.191s] (4/8) atelier-app cache_manager::tests::corrupt_cache_is_discarded_and_rebuilt
        PASS [   0.201s] (5/8) atelier-app cache_manager::tests::orientation_can_degrade_but_decision_queries_reject_known_stale_rows
        PASS [   0.289s] (6/8) atelier-app cache_manager::tests::missing_source_metadata_falls_back_to_full_rebuild
        PASS [   0.290s] (7/8) atelier-app cache_manager::tests::version_mismatch_is_discarded_and_rebuilt
        PASS [   0.295s] (8/8) atelier-app cache_manager::tests::bounded_changed_record_requests_incremental_then_falls_back_safely
────────────
     Summary [   0.296s] 8 tests run: 8 passed, 103 skipped
```
