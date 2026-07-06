---
created_at: "2026-07-06T17:50:26.254181964+00:00"
id: "atelier-qz2l"
evidence_type: "test"
captured_at: "2026-07-06T17:50:22.390990691+00:00"
command: "cargo nextest run -p atelier-cli test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mxnv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mxnv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly"
updated_at: "2026-07-06T17:50:37.534353061+00:00"
---

## Summary

cargo nextest run -p atelier-cli test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly

## Command

```console
cargo nextest run -p atelier-cli test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 586
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-eqq6/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.84s
────────────
 Nextest run ID 82f13c5c-dcb1-49b1-9493-a67fd5a91628 with nextest profile: default
    Starting 1 test across 4 binaries (443 tests skipped)
        PASS [   0.702s] (1/1) atelier-cli::cli_integration setup_guidance::test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly
────────────
     Summary [   0.703s] 1 test run: 1 passed, 443 skipped
```

