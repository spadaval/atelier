---
created_at: "2026-06-16T18:24:06.417627701+00:00"
id: "atelier-1rlh"
evidence_type: "test"
captured_at: "2026-06-16T18:24:01.636652831+00:00"
command: "cargo test -p atelier-cli test_mission_status_cli_reports_control_state -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-f7vd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-f7vd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission status test covers readable state rows, blocker-first actionable guidance, blocked drill-down cues, evidence gaps, workflow state, and mission-vs-issue blocker separation."
updated_at: "2026-06-16T18:24:10.047673434+00:00"
---

## Summary

Mission status test covers readable state rows, blocker-first actionable guidance, blocked drill-down cues, evidence gaps, workflow state, and mission-vs-issue blocker separation.

## Command

```console
cargo test -p atelier-cli test_mission_status_cli_reports_control_state -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 543
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mission_projection_worktree::test_mission_status_cli_reports_control_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 334 filtered out; finished in 2.90s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
