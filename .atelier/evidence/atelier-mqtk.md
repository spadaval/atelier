---
created_at: "2026-07-09T16:29:42.391077838+00:00"
id: "atelier-mqtk"
evidence_type: "test"
captured_at: "2026-07-09T16:29:39.596656741+00:00"
command: "cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-nzu9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nzu9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags"
updated_at: "2026-07-09T16:29:42.394577916+00:00"
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

Bytes: 606
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-nzu9-validation/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.07s
────────────
 Nextest run ID eded35f2-f955-48f1-8569-a3caef77df8e with nextest profile: default
    Starting 1 test across 4 binaries (472 tests skipped)
        PASS [   0.398s] (1/1) atelier-cli::cli_integration issues::test_issue_list_is_flat_metadata_inventory_with_quiet_limit_and_removed_operational_flags
────────────
     Summary [   0.399s] 1 test run: 1 passed, 472 skipped
```

