---
created_at: "2026-10-22T21:00:00.173901196+00:00"
id: "atelier-y9vz"
evidence_type: "test"
commit: "5eea164a64939ac05a36a635277a7f60e0659bcc"
result: "pass"
captured_at: "2026-10-22T20:59:50.298053611+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(start_refuses_a_branch_whose)'"
exit_status: "0"
agent_identity: "claude-implementer-mhx0"
target:
  kind: "issue"
  id: "atelier-mhx0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mhx0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "start refuses a branch with a mismatched type prefix and names the right branch"
updated_at: "2026-10-22T21:00:00.176896649+00:00"
---

## Summary

start refuses a branch with a mismatched type prefix and names the right branch

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(start_refuses_a_branch_whose)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 566
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.91s
────────────
 Nextest run ID 2a4057f8-8afe-47ac-acaa-49234cb3c0b7 with nextest profile: default
    Starting 1 test across 6 binaries (633 tests skipped)
        PASS [   0.575s] (1/1) atelier-cli::session start_refuses_a_branch_whose_type_prefix_does_not_match_the_issue
────────────
     Summary [   0.592s] 1 test run: 1 passed, 633 skipped
```

