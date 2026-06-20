---
created_at: "2026-06-20T21:18:56.902637786+00:00"
id: "atelier-4y1a"
evidence_type: "test"
captured_at: "2026-06-20T21:18:54.520924389+00:00"
command: "cargo test -p atelier-cli forgejo -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3d81"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3d81"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli forgejo -- --nocapture"
updated_at: "2026-06-20T21:19:01.933431580+00:00"
---

## Summary

cargo test -p atelier-cli forgejo -- --nocapture

## Command

```console
cargo test -p atelier-cli forgejo -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 632
Truncated: no

```text

running 1 test
test commands::forgejo::tests::inspect_roles_reports_success_and_collapsed_mappings ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 159 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_forgejo_roles_provision_write_config_flag_is_removed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 360 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.28s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
     Running unittests src/main.rs (target/debug/deps/atelier-6490c36d57e88ab2)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-2a96ae708789461f)
```

