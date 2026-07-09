---
created_at: "2026-07-06T18:04:13.223025536+00:00"
id: "atelier-ytmy"
evidence_type: "test"
captured_at: "2026-07-06T18:04:12.827505938+00:00"
command: "cargo nextest run -p atelier-app issue_inventory"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yf06"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yf06"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-app issue_inventory"
updated_at: "2026-07-06T18:04:17.462896055+00:00"
---

## Summary

cargo nextest run -p atelier-app issue_inventory

## Command

```console
cargo nextest run -p atelier-app issue_inventory
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```
## Stderr

Bytes: 1049
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID 9747a58b-b9d5-4336-b464-f27777988677 with nextest profile: default
    Starting 6 tests across 1 binary (105 tests skipped)
        PASS [   0.009s] (1/6) atelier-app issue_inventory::tests::default_and_explicit_positive_limits_apply_after_selection_and_ordering
        PASS [   0.009s] (2/6) atelier-app issue_inventory::tests::zero_limit_is_rejected
        PASS [   0.009s] (3/6) atelier-app issue_inventory::tests::empty_view_retains_selection_and_budget_facts
        PASS [   0.010s] (4/6) atelier-app issue_inventory::tests::duplicate_canonical_identity_produces_one_row
        PASS [   0.010s] (5/6) atelier-app issue_inventory::tests::exact_metadata_filters_compose_with_and_semantics
        PASS [   0.010s] (6/6) atelier-app issue_inventory::tests::default_inventory_is_flat_id_ordered_and_includes_done_records
────────────
     Summary [   0.011s] 6 tests run: 6 passed, 105 skipped
```
