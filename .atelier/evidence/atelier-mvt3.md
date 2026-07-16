---
created_at: "2026-07-06T18:07:24.298997705+00:00"
id: "atelier-mvt3"
evidence_type: "test"
captured_at: "2026-07-06T18:07:22.658278413+00:00"
command: "cargo nextest run -p atelier-app mission_overview"
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
title: "cargo nextest run -p atelier-app mission_overview"
updated_at: "2026-07-06T18:07:28.781848404+00:00"
---

## Summary

cargo nextest run -p atelier-app mission_overview

## Command

```console
cargo nextest run -p atelier-app mission_overview
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```
## Stderr

Bytes: 1148
Truncated: no

```text
   Compiling atelier-app v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-overview/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.31s
────────────
 Nextest run ID 1639c609-f28e-4eab-8706-207c65417aa1 with nextest profile: default
    Starting 5 tests across 1 binary (105 tests skipped)
        PASS [   0.009s] (1/5) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.009s] (2/5) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.009s] (3/5) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.009s] (4/5) atelier-app mission_overview::tests::projects_directed_epics_direct_work_descendants_and_blocker_state
        PASS [   0.013s] (5/5) atelier-app mission_overview::tests::orders_rows_deterministically_and_applies_budgets_after_counting
────────────
     Summary [   0.013s] 5 tests run: 5 passed, 105 skipped
```
