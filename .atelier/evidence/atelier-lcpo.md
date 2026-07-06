---
created_at: "2026-06-25T01:47:21.148494079+00:00"
id: "atelier-lcpo"
evidence_type: "test"
captured_at: "2026-06-25T01:47:19.163957828+00:00"
command: "cargo test -p atelier-cli test_top_level_help_only_shows_core_commands -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-8c91"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8c91"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli test_top_level_help_only_shows_core_commands -- --nocapture"
updated_at: "2026-06-25T01:47:26.351516280+00:00"
---

## Summary

cargo test -p atelier-cli test_top_level_help_only_shows_core_commands -- --nocapture

## Command

```console
cargo test -p atelier-cli test_top_level_help_only_shows_core_commands -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 529
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 156 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_top_level_help_only_shows_core_commands ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 290 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.89s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
     Running unittests src/main.rs (target/debug/deps/atelier-6490c36d57e88ab2)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-2a96ae708789461f)
```

