---
created_at: "2026-07-09T16:15:37.391450940+00:00"
id: "atelier-90hp"
evidence_type: "test"
captured_at: "2026-07-09T16:15:34.039180863+00:00"
command: "cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags"
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
    id: "atelier-kiyq"
    role: "validates"
  - kind: "issue"
    id: "atelier-yf06"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags"
updated_at: "2026-07-09T16:15:49.980988031+00:00"
---

## Summary

cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags

## Command

```console
cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 604
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-yf06-recovery/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.51s
────────────
 Nextest run ID ece0e0bd-10a0-4a0e-8296-7dcd2ab94571 with nextest profile: default
    Starting 1 test across 4 binaries (472 tests skipped)
        PASS [   0.481s] (1/1) atelier-cli::cli_integration issues::test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags
────────────
     Summary [   0.482s] 1 test run: 1 passed, 472 skipped
```
