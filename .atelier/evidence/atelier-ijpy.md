---
created_at: "2026-06-16T18:44:00.113455677+00:00"
id: "atelier-ijpy"
evidence_type: "test"
captured_at: "2026-06-16T18:43:53.928095467+00:00"
command: "cargo test -p atelier-cli test_mission_list_human_overview_orders_and_summarizes -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-kzfl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kzfl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission list stays compact, uses readable state labels, and cues visible blocker-chain work first"
updated_at: "2026-06-16T18:44:03.873926869+00:00"
---

## Summary

Mission list stays compact, uses readable state labels, and cues visible blocker-chain work first

## Command

```console
cargo test -p atelier-cli test_mission_list_human_overview_orders_and_summarizes -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 552
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_list_human_overview_orders_and_summarizes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 4.33s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.78s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
