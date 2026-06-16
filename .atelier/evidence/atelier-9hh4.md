---
created_at: "2026-06-16T19:35:37.618406126+00:00"
id: "atelier-9hh4"
evidence_type: "validation"
captured_at: "2026-06-16T19:35:27.352260803+00:00"
command: "bash -lc 'cargo fmt -- --check; cargo test -p atelier-cli issues::test_create_issue -- --nocapture; cargo test -p atelier-cli test_issue_create_scaffold_edit_lint_show_flow -- --nocapture; cargo test -p atelier-cli test_issue_create_help_is_markdown_first -- --nocapture; target/debug/atelier lint atelier-3yoa; target/debug/atelier export --check; git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3yoa"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3yoa"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Final markdown-first create checks pass after focused lint fix"
updated_at: "2026-06-16T19:35:41.157858517+00:00"
---

## Summary

Final markdown-first create checks pass after focused lint fix

## Command

```console
bash -lc 'cargo fmt -- --check; cargo test -p atelier-cli issues::test_create_issue -- --nocapture; cargo test -p atelier-cli test_issue_create_scaffold_edit_lint_show_flow -- --nocapture; cargo test -p atelier-cli test_issue_create_help_is_markdown_first -- --nocapture; target/debug/atelier lint atelier-3yoa; target/debug/atelier export --check; git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1781
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test issues::test_create_issue_rejects_work_flag ... ok
test issues::test_create_issue_with_description_is_rejected ... ok
test issues::test_create_issue ... ok
test issues::test_create_issue_with_priority ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 335 filtered out; finished in 0.12s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test issues::test_issue_create_scaffold_edit_lint_show_flow ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 338 filtered out; finished in 0.22s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test issues::test_issue_create_help_is_markdown_first ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 338 filtered out; finished in 0.09s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1634
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.04s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.13s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.72s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
