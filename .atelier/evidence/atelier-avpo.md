---
created_at: "2026-06-16T18:37:16.778131993+00:00"
id: "atelier-avpo"
evidence_type: "test"
captured_at: "2026-06-16T18:37:14.504781097+00:00"
command: "cargo test -p atelier-cli test_graph_tree_orders_children_by_visible_blockers -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-qh52"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qh52"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Graph tree orders visible blockers before blocked children in full and compact output"
updated_at: "2026-06-16T18:37:22.011881134+00:00"
---

## Summary

Graph tree orders visible blockers before blocked children in full and compact output

## Command

```console
cargo test -p atelier-cli test_graph_tree_orders_children_by_visible_blockers -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 536
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test setup_guidance::test_graph_tree_orders_children_by_visible_blockers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 336 filtered out; finished in 0.33s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s
```

## Stderr

Bytes: 525
Truncated: no

```text
    Blocking waiting for file lock on package cache
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.88s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
