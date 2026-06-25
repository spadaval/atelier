---
created_at: "2026-06-25T01:08:53.855601360+00:00"
id: "atelier-uvcy"
evidence_type: "test"
captured_at: "2026-06-25T01:08:51.236351925+00:00"
command: "cargo test -p atelier-cli test_issue_status_renders_objective_work_health -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-fasv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fasv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli test_issue_status_renders_objective_work_health -- --nocapture"
updated_at: "2026-06-25T01:08:57.173954568+00:00"
---

## Summary

cargo test -p atelier-cli test_issue_status_renders_objective_work_health -- --nocapture

## Command

```console
cargo test -p atelier-cli test_issue_status_renders_objective_work_health -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 532
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 156 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_issue_status_renders_objective_work_health ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 289 filtered out; finished in 0.67s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.88s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
     Running unittests src/main.rs (target/debug/deps/atelier-6490c36d57e88ab2)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-2a96ae708789461f)
```

