---
created_at: "2026-07-06T18:31:19.484137410+00:00"
id: "atelier-nixw"
evidence_type: "test"
captured_at: "2026-07-06T18:31:18.687234312+00:00"
command: "cargo nextest run -p atelier-app rebuild::tests"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-idwz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-idwz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-app rebuild::tests"
updated_at: "2026-07-06T18:31:23.917794416+00:00"
---

## Summary

cargo nextest run -p atelier-app rebuild::tests

## Command

```console
cargo nextest run -p atelier-app rebuild::tests
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 926
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
────────────
 Nextest run ID 39e4a9c1-d171-44e6-8b2f-458cdd1801e9 with nextest profile: default
    Starting 5 tests across 1 binary (87 tests skipped)
        PASS [   0.116s] (1/5) atelier-app rebuild::tests::one_issue_repair_is_bounded_and_has_one_transaction_owner
        PASS [   0.120s] (2/5) atelier-app rebuild::tests::parse_failure_leaves_rows_and_source_metadata_unchanged
        PASS [   0.170s] (3/5) atelier-app rebuild::tests::full_rebuild_replaces_schema_mismatch_as_disposable_state
        PASS [   0.206s] (4/5) atelier-app rebuild::tests::graph_change_requests_one_safe_full_rebuild
        PASS [   0.287s] (5/5) atelier-app rebuild::tests::full_and_incremental_add_change_delete_are_equivalent_for_all_domains
────────────
     Summary [   0.288s] 5 tests run: 5 passed, 87 skipped
```
