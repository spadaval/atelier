---
created_at: "2026-06-16T18:27:50.669702164+00:00"
id: "atelier-6s74"
evidence_type: "test"
captured_at: "2026-06-16T18:27:48.224931533+00:00"
command: "cargo test -p atelier-cli test_issue_show_subissues_use_blocker_order_and_state_labels -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-nqjc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nqjc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Issue show subissue test covers visible blocker child ordering, readable row states, external blocker drill-down cue, and retained next commands."
updated_at: "2026-06-16T18:27:54.312674051+00:00"
---

## Summary

Issue show subissue test covers visible blocker child ordering, readable row states, external blocker drill-down cue, and retained next commands.

## Command

```console
cargo test -p atelier-cli test_issue_show_subissues_use_blocker_order_and_state_labels -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 537
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test issues::test_issue_show_subissues_use_blocker_order_and_state_labels ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 335 filtered out; finished in 0.60s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s
```

## Stderr

Bytes: 473
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.77s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
```
