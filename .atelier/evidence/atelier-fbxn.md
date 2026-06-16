---
created_at: "2026-06-16T19:45:34.354197053+00:00"
id: "atelier-fbxn"
evidence_type: "validation"
captured_at: "2026-06-16T19:45:22.885270229+00:00"
command: "bash -lc 'cargo fmt -- --check && cargo test -p atelier-cli issues::test_issue_show_surfaces_evidence_status -- --nocapture && cargo test -p atelier-cli setup_guidance::test_root_status_reports_active_mission_contract_fields -- --nocapture && cargo test -p atelier-cli setup_guidance::test_root_status_no_ready_work_suggests_valid_blocked_list -- --nocapture && target/debug/atelier lint atelier-3gr9 && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3gr9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3gr9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Evidence status surfaces validated"
updated_at: "2026-06-16T19:45:38.149491829+00:00"
---

## Summary

Evidence status surfaces validated

## Command

```console
bash -lc 'cargo fmt -- --check && cargo test -p atelier-cli issues::test_issue_show_surfaces_evidence_status -- --nocapture && cargo test -p atelier-cli setup_guidance::test_root_status_reports_active_mission_contract_fields -- --nocapture && cargo test -p atelier-cli setup_guidance::test_root_status_no_ready_work_suggests_valid_blocked_list -- --nocapture && target/debug/atelier lint atelier-3gr9 && target/debug/atelier export --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1671
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test issues::test_issue_show_surfaces_evidence_status ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 1.46s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_root_status_reports_active_mission_contract_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.68s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_root_status_no_ready_work_suggests_valid_blocked_list ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.08s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1419
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.73s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.84s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.70s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```

