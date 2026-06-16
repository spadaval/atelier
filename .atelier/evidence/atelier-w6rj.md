---
created_at: "2026-06-16T18:17:33.220265820+00:00"
id: "atelier-w6rj"
evidence_type: "test"
captured_at: "2026-06-16T18:17:30.562406071+00:00"
command: "cargo test -p atelier-cli test_root_status_ -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-d226"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-d226"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Root status tests cover readable states, visible blocker ordering, blocked drill-down cues, current-work state labels, and no-mission ready guidance."
updated_at: "2026-06-16T18:17:36.956135527+00:00"
---

## Summary

Root status tests cover readable states, visible blocker ordering, blocked drill-down cues, current-work state labels, and no-mission ready guidance.

## Command

```console
cargo test -p atelier-cli test_root_status_ -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 804
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test setup_guidance::test_root_status_no_ready_work_suggests_valid_blocked_list ... ok
test setup_guidance::test_root_status_summarizes_checkout_orientation ... ok
test setup_guidance::test_root_status_guides_current_work_to_transition_and_worktree_status ... ok
test setup_guidance::test_root_status_reports_active_mission_contract_fields ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 331 filtered out; finished in 0.84s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.74s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
