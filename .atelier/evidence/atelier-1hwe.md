---
created_at: "2026-07-07T05:31:59.311419029+00:00"
id: "atelier-1hwe"
evidence_type: "test"
captured_at: "2026-07-07T05:31:59.080962273+00:00"
command: "cargo test -p atelier-app mission_overview --lib"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-tdgs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-tdgs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-app mission_overview --lib"
updated_at: "2026-07-07T05:32:03.447358581+00:00"
---

## Summary

cargo test -p atelier-app mission_overview --lib

## Command

```console
cargo test -p atelier-app mission_overview --lib
```
Exit status: 0

## Stdout

Bytes: 755
Truncated: no

```text

running 6 tests
test mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work ... ok
test mission_overview::tests::projects_directed_epics_direct_work_descendants_and_blocker_state ... ok
test mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership ... ok
test mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested ... ok
test mission_overview::tests::orders_rows_deterministically_and_applies_budgets_after_counting ... ok
test mission_overview::tests::acquires_directed_cache_facts_and_projects_the_command_model ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 94 filtered out; finished in 0.07s
```

## Stderr

Bytes: 156
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-4714e0c86f5f1ad0)
```
