---
created_at: "2026-06-16T18:17:42.600454515+00:00"
id: "atelier-azsb"
evidence_type: "test"
captured_at: "2026-06-16T18:17:40.734719721+00:00"
command: "cargo test -p atelier-cli work_order -- --nocapture"
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
title: "Shared work_order tests still pass for blocker graph and row-state cases."
updated_at: "2026-06-16T18:17:46.142424022+00:00"
---

## Summary

Shared work_order tests still pass for blocker graph and row-state cases.

## Command

```console
cargo test -p atelier-cli work_order -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 1039
Truncated: no

```text

running 7 tests
test commands::work_order::tests::hidden_blockers_do_not_insert_phantom_rows ... ok
test commands::work_order::tests::done_blockers_are_ignored_by_open_blocker_input ... ok
test commands::work_order::tests::orders_simple_visible_blocker_chain ... ok
test commands::work_order::tests::orders_diamond_dependencies_before_join ... ok
test commands::work_order::tests::cycles_are_deterministic_and_preserve_all_rows ... ok
test commands::work_order::tests::row_state_labels_cover_work_view_vocabulary ... ok
test commands::work_order::tests::ties_use_priority_update_time_and_id ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 164 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 335 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.79s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
